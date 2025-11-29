use std::path::PathBuf;

use tracel_xtask::prelude::*;

#[derive(clap::Args)]
pub struct GenerateCmdArgs {
    /// The provider to generate the bindings from.
    #[arg(value_enum)]
    pub prodiver: Provider,
    /// If set then choose the JSON generating all the resources and data sources for a given provider.
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Provider {
    /// Hashicorp AWS Provider
    Aws,
}

impl Provider {
    pub fn path(&self, all: bool) -> PathBuf {
        match self {
            Provider::Aws => {
                if all {
                    PathBuf::from("crates/tracel-terrars-hashicorp-aws/terrars-all.json")
                } else {
                    PathBuf::from("crates/tracel-terrars-hashicorp-aws/terrars-min.json")
                }
            }
        }
    }
}

pub fn handle_command(args: GenerateCmdArgs) -> anyhow::Result<()> {
    let json = git::git_repo_root_or_cwd()
        .unwrap()
        .join(args.prodiver.path(args.all));
    run_process(
        "cargo",
        &["generate", &json.to_string_lossy()],
        None,
        None,
        "",
    )
}
