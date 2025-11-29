use crate::generatelib::{
    generate::{
        generate_block_fields, generate_fields_from_value_map, to_camel, to_snake, TopLevelFields,
    },
    sourceschema::ProviderSchemas,
};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

pub mod generatelib;

pub trait CollCommand {
    fn run(&mut self) -> Result<()>;
}

impl CollCommand for Command {
    fn run(&mut self) -> Result<()> {
        let status = self
            .status()
            .with_context(|| format!("Failed to run command {:?}", self))?;

        if status.success() {
            Ok(())
        } else {
            bail!("Command should succeed but exited with status {status}")
        }
    }
}

// ANSI colors
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_WHITE: &str = "\x1b[37m";

/// Configuration file format
#[derive(Serialize, Deserialize)]
struct Config {
    provider: String,
    version: String,

    include_resources: Option<Vec<String>>,
    exclude_resources: Option<Vec<String>>,
    include_datasources: Option<Vec<String>>,
    exclude_datasources: Option<Vec<String>>,

    dest: PathBuf,
    feature_gate: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path to terrars config jsons.
    #[arg(value_name = "CONFIG")]
    configs: Vec<PathBuf>,

    /// Save the provider json in this dir (debug helper).
    #[arg(long)]
    dump: bool,
}

/// Include/exclude filter for resources or datasources.
struct Filter {
    include: HashSet<String>,
    exclude: HashSet<String>,
}

impl Filter {
    fn from_options(include: &Option<Vec<String>>, exclude: &Option<Vec<String>>) -> Self {
        Self {
            include: include.clone().unwrap_or_default().into_iter().collect(),
            exclude: exclude.clone().unwrap_or_default().into_iter().collect(),
        }
    }

    /// Ensure no name is present in both include and exclude.
    fn validate(&self, kind: &str, include_label: &str, exclude_label: &str) -> Result<()> {
        if let Some(name) = self.include.intersection(&self.exclude).next() {
            bail!(
                "{kind} '{name}' is present in both {include_label} and {exclude_label}",
                kind = kind,
                name = name,
                include_label = include_label,
                exclude_label = exclude_label,
            );
        }
        Ok(())
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("❌ Error: {err:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Arguments::parse();

    if args.configs.is_empty() {
        bail!("No configs specified; nothing to do");
    }

    for config_path in args.configs {
        let raw = fs::read(&config_path).with_context(|| {
            format!(
                "Failed to read config file {}",
                config_path.to_string_lossy()
            )
        })?;

        let config: Config = serde_json::from_slice(&raw).with_context(|| {
            format!(
                "Error parsing config json from {}",
                config_path.to_string_lossy()
            )
        })?;

        eprintln!(
            "📦 Processing provider {provider}@{version}\n   📁 Dest: {dest}",
            provider = config.provider,
            version = config.version,
            dest = config.dest.to_string_lossy(),
        );

        let (vendor, shortname) = config
            .provider
            .split_once('/')
            .unwrap_or_else(|| ("hashicorp", &config.provider));
        let provider_prefix = format!("{}_", shortname);

        // Build filters from config
        let mut resource_filter =
            Filter::from_options(&config.include_resources, &config.exclude_resources);
        let mut datasource_filter =
            Filter::from_options(&config.include_datasources, &config.exclude_datasources);

        // Validate that nothing is both included and excluded
        resource_filter.validate("Resource", "include_resources", "exclude_resources")?;
        datasource_filter.validate("Datasource", "include_datasources", "exclude_datasources")?;

        // Feature output
        let mut features = vec![];

        // Get provider schema
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("providers.tf.json"),
            &serde_json::to_vec(&json!({
                "terraform": {
                    "required_providers": {
                        shortname: {
                            "source": config.provider,
                            "version": config.version,
                        }
                    }
                }
            }))?,
        )
        .context("Failed to write bootstrap terraform code for provider schema extraction")?;

        eprintln!("⚙️  Running `terraform init`...");
        Command::new("terraform")
            .args(["init"])
            .current_dir(&dir)
            .run()
            .context("Error initializing terraform in export dir")?;

        eprintln!("🧩 Generating terraform provider schema...");
        let schema_raw = Command::new("terraform")
            .args(["providers", "schema", "-json", "-no-color"])
            .current_dir(&dir)
            .output()
            .context("Error outputting terraform provider schema")?
            .stdout;

        if args.dump {
            fs::write("dump.json", &schema_raw)?;
            eprintln!("💾 Schema dump written to dump.json");
        }

        let schema: ProviderSchemas = serde_json::from_slice(&schema_raw)
            .context("Error parsing provider schema json from terraform")?;
        eprintln!("✅ Provider schema loaded.");

        // Generate
        fn write_file(path: &Path, contents: Vec<TokenStream>) -> Result<()> {
            let formatted = genemichaels_lib::format_ast(
                syn::parse2::<syn::File>(quote!(#(#contents)*)).with_context(|| {
                    let context = contents
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");
                    let numbered = context
                        .lines()
                        .enumerate()
                        .map(|(ln, l)| format!("{:0>4} {}", ln + 1, l))
                        .collect::<Vec<String>>()
                        .join("\n");
                    format!(
                        "Failed to parse generated code AST for formatting\n{}",
                        numbered
                    )
                })?,
                &genemichaels_lib::FormatConfig::default(),
                Default::default(),
            )
            .map_err(|e| anyhow!("Error formatting generated code: {e}"))?;

            File::create(path)
                .with_context(|| format!("Failed to create rust file {}", path.to_string_lossy()))?
                .write_all(formatted.rendered.as_bytes())
                .with_context(|| format!("Failed to write rust file {}", path.to_string_lossy()))?;

            Ok(())
        }

        fn rustfile_template() -> Vec<TokenStream> {
            vec![quote!(
                use serde::Serialize;
                use std::cell::RefCell;
                use std::rc::Rc;
                use terrars::*;
            )]
        }

        eprintln!("🦀 Generating Rust bindings...");

        // Provider type + provider
        let provider_schema = {
            let key = format!("registry.terraform.io/{}/{}", vendor, shortname);
            schema.provider_schemas.get(&key).with_context(|| {
                format!(
                    "Missing provider schema for listed provider {} (key: {})",
                    config.provider, key
                )
            })?
        };
        let provider_name_parts = &shortname
            .split('-')
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let provider_dir = config.dest;
        if provider_dir.exists() {
            remove_dir_all(&provider_dir).with_context(|| {
                format!(
                    "Failed to remove existing provider directory {}",
                    provider_dir.to_string_lossy()
                )
            })?;
        }
        create_dir_all(&provider_dir).with_context(|| {
            format!(
                "Failed to create provider directory {}",
                provider_dir.to_string_lossy()
            )
        })?;
        let mut mod_out = vec![];
        let provider_ident: Ident;
        {
            let mut out = rustfile_template();
            let camel_name = to_camel(provider_name_parts);
            let source = &config.provider;
            let version = &config.version;
            let provider_inner_mut_ident = format_ident!("Provider{}Data", camel_name);
            let mut raw_fields = TopLevelFields::default();
            generate_fields_from_value_map(
                &mut raw_fields,
                provider_name_parts,
                &provider_schema.provider.block.attributes,
                true,
            );
            let builder_fields = raw_fields.builder_fields;
            let copy_builder_fields = raw_fields.copy_builder_fields;
            let extra_types = raw_fields.extra_types;
            let provider_fields = raw_fields.fields;
            let provider_mut_methods = raw_fields.mut_methods;
            provider_ident = format_ident!("Provider{}", camel_name);
            let provider_inner_ident = format_ident!("Provider{}_", camel_name);
            let provider_builder_ident = format_ident!("BuildProvider{}", camel_name);
            out.push(quote! {
                #[derive(Serialize)]
                struct #provider_inner_mut_ident {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    alias: Option<String>,
                    #(#provider_fields,)*
                }

                struct #provider_inner_ident {
                    data: RefCell<#provider_inner_mut_ident>,
                }

                pub struct #provider_ident(Rc<#provider_inner_ident>);

                impl #provider_ident {
                    pub fn provider_ref(&self) -> String {
                        let data = self.0.data.borrow();
                        if let Some(alias) = &data.alias {
                            format!("{}.{}", #shortname, alias)
                        } else {
                            #shortname.into()
                        }
                    }

                    pub fn set_alias(self, alias: impl ToString) -> Self {
                        self.0.data.borrow_mut().alias = Some(alias.to_string());
                        self
                    }

                    #(#provider_mut_methods)*
                }

                impl Provider for #provider_inner_ident {
                    fn extract_type_tf_id(&self) -> String {
                        #shortname.into()
                    }

                    fn extract_provider_type(&self) -> serde_json::Value {
                        serde_json::json!({
                            "source": #source,
                            "version": #version,
                        })
                    }

                    fn extract_provider(&self) -> serde_json::Value {
                        serde_json::to_value(&self.data).unwrap()
                    }
                }

                pub struct #provider_builder_ident {
                    #(#builder_fields,)*
                }

                impl #provider_builder_ident {
                    pub fn build(self, stack: &mut Stack) -> #provider_ident {
                        let out = #provider_ident(Rc::new(#provider_inner_ident {
                            data: RefCell::new(#provider_inner_mut_ident {
                                alias: None,
                                #(#copy_builder_fields,)*
                            }),
                        }));
                        stack.add_provider(out.0.clone());
                        out
                    }
                }

                #(#extra_types)*
            });
            write_file(&provider_dir.join("provider.rs"), out)?;
            let path_ident = format_ident!("provider");
            mod_out.push(quote!(pub mod #path_ident; pub use #path_ident::*;));
        }

        // ----- Resources -----
        eprintln!("───────────────────────────────────────────────────────────────────────────");
        eprintln!("⚡ Resources generation");
        eprintln!("───────────────────────────────────────────────────────────────────────────");

        for (resource_name, resource) in &provider_schema.resource_schemas {
            let mut out = rustfile_template();
            out.push(quote!(use super::provider::#provider_ident;));
            let use_name_parts = resource_name
                .strip_prefix(&provider_prefix)
                .with_context(|| {
                    format!(
                        "Name missing expected provider prefix (resource: {resource_name}, prefix: {provider_prefix})"
                    )
                })?
                .split('_')
                .map(ToString::to_string)
                .collect::<Vec<String>>();

            let nice_resource_name = to_snake(&use_name_parts);
            let was_included = resource_filter.include.remove(&nice_resource_name);
            let is_excluded = resource_filter.exclude.contains(&nice_resource_name);
            if !log_triage(&nice_resource_name, was_included, is_excluded) {
                continue;
            }

            let camel_name = to_camel(&use_name_parts);
            let mut raw_fields = TopLevelFields::default();
            generate_fields_from_value_map(
                &mut raw_fields,
                &use_name_parts,
                &resource.block.attributes,
                true,
            );
            generate_block_fields(
                &mut raw_fields,
                &use_name_parts,
                &resource.block.block_types,
                true,
            );
            raw_fields.finish(&camel_name);
            let builder_fields = raw_fields.builder_fields;
            let copy_builder_fields = raw_fields.copy_builder_fields;
            let extra_types = raw_fields.extra_types;
            let resource_fields = raw_fields.fields;
            let resource_mut_methods = raw_fields.mut_methods;
            let resource_ref_methods = raw_fields.ref_methods;
            let resource_ident = format_ident!("{}", camel_name);
            let resource_inner_ident = format_ident!("{}_", camel_name);
            let resource_inner_mut_ident = format_ident!("{}Data", camel_name);
            let resource_builder_ident = format_ident!("Build{}", camel_name);
            let resource_ref_ident = format_ident!("{}Ref", camel_name);

            out.push(quote! {
                #[derive(Serialize)]
                struct #resource_inner_mut_ident {
                    #[serde(skip_serializing_if = "Vec::is_empty")]
                    depends_on: Vec<String>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    provider: Option<String>,
                    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
                    lifecycle: ResourceLifecycle,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    for_each: Option<String>,
                    #(#resource_fields,)*
                }

                struct #resource_inner_ident {
                    shared: StackShared,
                    tf_id: String,
                    data: RefCell<#resource_inner_mut_ident>,
                }

                #[derive(Clone)]
                pub struct #resource_ident(Rc<#resource_inner_ident>);

                impl #resource_ident {
                    fn shared(&self) -> &StackShared {
                        &self.0.shared
                    }

                    pub fn depends_on(self, dep: &impl Referable) -> Self {
                        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
                        self
                    }

                    pub fn set_provider(self, provider: &#provider_ident) -> Self {
                        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
                        self
                    }

                    pub fn set_create_before_destroy(self, v: bool) -> Self {
                        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
                        self
                    }

                    pub fn set_prevent_destroy(self, v: bool) -> Self {
                        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
                        self
                    }

                    pub fn ignore_changes_to_all(self) -> Self {
                        self.0.data.borrow_mut().lifecycle.ignore_changes =
                            Some(IgnoreChanges::All(IgnoreChangesAll::All));
                        self
                    }

                    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
                        {
                            let mut d = self.0.data.borrow_mut();
                            if match &mut d.lifecycle.ignore_changes {
                                Some(i) => match i {
                                    IgnoreChanges::All(_) => true,
                                    IgnoreChanges::Refs(r) => {
                                        r.push(attr.to_string());
                                        false
                                    }
                                },
                                None => true,
                            } {
                                d.lifecycle.ignore_changes =
                                    Some(IgnoreChanges::Refs(vec![attr.to_string()]));
                            }
                        }
                        self
                    }

                    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
                        self.0
                            .data
                            .borrow_mut()
                            .lifecycle
                            .replace_triggered_by
                            .push(r.extract_ref());
                        self
                    }

                    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
                        self.0
                            .data
                            .borrow_mut()
                            .lifecycle
                            .replace_triggered_by
                            .push(attr.to_string());
                        self
                    }

                    #(#resource_mut_methods)*
                    #(#resource_ref_methods)*
                }

                impl Referable for #resource_ident {
                    fn extract_ref(&self) -> String {
                        format!(
                            "{}.{}",
                            self.0.extract_resource_type(),
                            self.0.extract_tf_id()
                        )
                    }
                }

                impl Resource for #resource_ident {}

                impl ToListMappable for #resource_ident {
                    type O = ListRef<#resource_ref_ident>;

                    fn do_map(self, base: String) -> Self::O {
                        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
                        ListRef::new(self.0.shared.clone(), self.extract_ref())
                    }
                }

                impl Resource_ for #resource_inner_ident {
                    fn extract_resource_type(&self) -> String {
                        #resource_name.into()
                    }

                    fn extract_tf_id(&self) -> String {
                        self.tf_id.clone()
                    }

                    fn extract_value(&self) -> serde_json::Value {
                        serde_json::to_value(&self.data).unwrap()
                    }
                }

                pub struct #resource_builder_ident {
                    pub tf_id: String,
                    #(#builder_fields,)*
                }

                impl #resource_builder_ident {
                    pub fn build(self, stack: &mut Stack) -> #resource_ident {
                        let out = #resource_ident(Rc::new(#resource_inner_ident {
                            shared: stack.shared.clone(),
                            tf_id: self.tf_id,
                            data: RefCell::new(#resource_inner_mut_ident {
                                depends_on: core::default::Default::default(),
                                provider: None,
                                lifecycle: core::default::Default::default(),
                                for_each: None,
                                #(#copy_builder_fields,)*
                            }),
                        }));
                        stack.add_resource(out.0.clone());
                        out
                    }
                }

                pub struct #resource_ref_ident {
                    shared: StackShared,
                    base: String,
                }

                impl Ref for #resource_ref_ident {
                    fn new(shared: StackShared, base: String) -> Self {
                        Self { shared, base }
                    }
                }

                impl #resource_ref_ident {
                    fn extract_ref(&self) -> String {
                        self.base.clone()
                    }

                    fn shared(&self) -> &StackShared {
                        &self.shared
                    }

                    #(#resource_ref_methods)*
                }

                #(#extra_types)*
            });

            write_file(
                &provider_dir.join(format!("{}.rs", nice_resource_name)),
                out,
            )?;
            let path_ident = format_ident!("{}", nice_resource_name);
            let feature_gate = if config.feature_gate.is_some() {
                features.push(nice_resource_name.clone());
                quote!(#[cfg(feature = #nice_resource_name)])
            } else {
                quote!()
            };
            mod_out.push(quote! {
                #feature_gate pub mod #path_ident;
                #feature_gate pub use #path_ident::*;
            });
        }

        // ----- Data sources -----
        eprintln!("───────────────────────────────────────────────────────────────────────────");
        eprintln!("⚡ Data sources generation");
        eprintln!("───────────────────────────────────────────────────────────────────────────");

        for (datasource_name, datasource) in &provider_schema.data_source_schemas {
            let mut out = rustfile_template();
            out.push(quote!(use super::provider::#provider_ident;));
            let use_name_parts = ["data"]
                .into_iter()
                .chain(
                    datasource_name
                        .strip_prefix(&provider_prefix)
                        .with_context(|| {
                            format!(
                                "Name missing expected provider prefix (datasource: {datasource_name}, prefix: {provider_prefix})"
                            )
                        })?
                        .split('_'),
                )
                .map(ToString::to_string)
                .collect::<Vec<String>>();
            let nice_datasource_name = to_snake(&use_name_parts);
            let was_included = datasource_filter.include.remove(&nice_datasource_name);
            let is_excluded = datasource_filter.exclude.contains(&nice_datasource_name);
            if !log_triage(&nice_datasource_name, was_included, is_excluded) {
                continue;
            }

            let camel_name = to_camel(&use_name_parts);
            let mut raw_fields = TopLevelFields::default();
            generate_fields_from_value_map(
                &mut raw_fields,
                &use_name_parts,
                &datasource.block.attributes,
                true,
            );
            generate_block_fields(
                &mut raw_fields,
                &use_name_parts,
                &datasource.block.block_types,
                true,
            );
            raw_fields.finish(&camel_name);
            let builder_fields = raw_fields.builder_fields;
            let copy_builder_fields = raw_fields.copy_builder_fields;
            let extra_types = raw_fields.extra_types;
            let datasource_fields = raw_fields.fields;
            let datasource_mut_methods = raw_fields.mut_methods;
            let datasource_ref_methods = raw_fields.ref_methods;
            let datasource_ident = format_ident!("{}", camel_name);
            let datasource_inner_ident = format_ident!("{}_", camel_name);
            let datasource_inner_mut_ident = format_ident!("{}Data", camel_name);
            let datasource_builder_ident = format_ident!("Build{}", camel_name);
            let datasource_ref_ident = format_ident!("{}Ref", camel_name);

            out.push(quote! {
                #[derive(Serialize)]
                struct #datasource_inner_mut_ident {
                    #[serde(skip_serializing_if = "Vec::is_empty")]
                    depends_on: Vec<String>,
                    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
                    provider: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    for_each: Option<String>,
                    #(#datasource_fields,)*
                }

                struct #datasource_inner_ident {
                    shared: StackShared,
                    tf_id: String,
                    data: RefCell<#datasource_inner_mut_ident>,
                }

                #[derive(Clone)]
                pub struct #datasource_ident(Rc<#datasource_inner_ident>);

                impl #datasource_ident {
                    fn shared(&self) -> &StackShared {
                        &self.0.shared
                    }

                    pub fn depends_on(self, dep: &impl Referable) -> Self {
                        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
                        self
                    }

                    pub fn set_provider(&self, provider: &#provider_ident) -> &Self {
                        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
                        self
                    }

                    #(#datasource_mut_methods)*
                    #(#datasource_ref_methods)*
                }

                impl Referable for #datasource_ident {
                    fn extract_ref(&self) -> String {
                        format!(
                            "data.{}.{}",
                            self.0.extract_datasource_type(),
                            self.0.extract_tf_id()
                        )
                    }
                }

                impl Datasource for #datasource_ident {}

                impl ToListMappable for #datasource_ident {
                    type O = ListRef<#datasource_ref_ident>;

                    fn do_map(self, base: String) -> Self::O {
                        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
                        ListRef::new(self.0.shared.clone(), self.extract_ref())
                    }
                }

                impl Datasource_ for #datasource_inner_ident {
                    fn extract_datasource_type(&self) -> String {
                        #datasource_name.into()
                    }

                    fn extract_tf_id(&self) -> String {
                        self.tf_id.clone()
                    }

                    fn extract_value(&self) -> serde_json::Value {
                        serde_json::to_value(&self.data).unwrap()
                    }
                }

                pub struct #datasource_builder_ident {
                    pub tf_id: String,
                    #(#builder_fields,)*
                }

                impl #datasource_builder_ident {
                    pub fn build(self, stack: &mut Stack) -> #datasource_ident {
                        let out = #datasource_ident(Rc::new(#datasource_inner_ident {
                            shared: stack.shared.clone(),
                            tf_id: self.tf_id,
                            data: RefCell::new(#datasource_inner_mut_ident {
                                depends_on: core::default::Default::default(),
                                provider: None,
                                for_each: None,
                                #(#copy_builder_fields,)*
                            }),
                        }));
                        stack.add_datasource(out.0.clone());
                        out
                    }
                }

                pub struct #datasource_ref_ident {
                    shared: StackShared,
                    base: String,
                }

                impl Ref for #datasource_ref_ident {
                    fn new(shared: StackShared, base: String) -> Self {
                        Self { shared, base }
                    }
                }

                impl #datasource_ref_ident {
                    fn shared(&self) -> &StackShared {
                        &self.shared
                    }

                    fn extract_ref(&self) -> String {
                        self.base.clone()
                    }

                    #(#datasource_ref_methods)*
                }

                #(#extra_types)*
            });

            write_file(
                &provider_dir.join(format!("{}.rs", nice_datasource_name)),
                out,
            )?;
            let path_ident = format_ident!("{}", nice_datasource_name);
            let feature_gate = if config.feature_gate.is_some() {
                features.push(nice_datasource_name.clone());
                quote!(#[cfg(feature = #nice_datasource_name)])
            } else {
                quote!()
            };
            mod_out.push(quote! {
                #feature_gate pub mod #path_ident;
                #feature_gate pub use #path_ident::*;
            });
        }

        write_file(&provider_dir.join("mod.rs"), mod_out)?;

        // Any included items that we never saw is an error
        if !resource_filter.include.is_empty() || !datasource_filter.include.is_empty() {
            bail!(
                "Included resources/datasources were not found: resources={:?}, datasources={:?}",
                resource_filter.include,
                datasource_filter.include
            );
        }

        if !features.is_empty() {
            let cargo_path = config.feature_gate.unwrap();
            let mut manifest =
                cargo_toml::Manifest::from_slice(&fs::read(&cargo_path).with_context(|| {
                    format!(
                        "Error opening Cargo.toml to update features (path: {})",
                        cargo_path.to_string_lossy()
                    )
                })?)
                .with_context(|| {
                    format!(
                        "Error parsing Cargo.toml (path: {})",
                        cargo_path.to_string_lossy()
                    )
                })?;

            manifest.features.clear();
            for f in features {
                manifest.features.insert(f, vec![]);
            }

            fs::write(
                &cargo_path,
                toml::to_string(&manifest)
                    .context("Error serializing modified Cargo.toml")?
                    .into_bytes(),
            )
            .with_context(|| {
                format!(
                    "Error writing to Cargo.toml (path: {})",
                    cargo_path.to_string_lossy()
                )
            })?;
        }

        eprintln!(
            "🎉 Finished generating bindings for {provider}@{version}",
            provider = config.provider,
            version = config.version
        );
    }

    Ok(())
}

fn log_triage(name: &str, was_included: bool, is_excluded: bool) -> bool {
    if was_included {
        eprintln!(
            "{green}✅ [allowed]{reset} {white}{name}{reset}...",
            green = COLOR_GREEN,
            reset = COLOR_RESET,
            white = COLOR_WHITE,
            name = name,
        );
        true
    } else if is_excluded {
        eprintln!(
            "{red}⛔ [ignored]{reset} {white}{name}{reset}",
            red = COLOR_RED,
            reset = COLOR_RESET,
            white = COLOR_WHITE,
            name = name,
        );
        false
    } else {
        eprintln!(
            "{yellow}❔ [unrated]{reset} {white}{name}{reset}",
            yellow = COLOR_YELLOW,
            reset = COLOR_RESET,
            white = COLOR_WHITE,
            name = name,
        );
        false
    }
}
