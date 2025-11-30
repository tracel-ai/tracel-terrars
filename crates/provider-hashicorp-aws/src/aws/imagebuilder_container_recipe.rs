use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ImagebuilderContainerRecipeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    container_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_template_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_template_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent_image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<Vec<ImagebuilderContainerRecipeComponentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_configuration: Option<Vec<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_repository: Option<Vec<ImagebuilderContainerRecipeTargetRepositoryEl>>,
    dynamic: ImagebuilderContainerRecipeDynamic,
}

struct ImagebuilderContainerRecipe_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderContainerRecipeData>,
}

#[derive(Clone)]
pub struct ImagebuilderContainerRecipe(Rc<ImagebuilderContainerRecipe_>);

impl ImagebuilderContainerRecipe {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_template_data`.\n"]
    pub fn set_dockerfile_template_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dockerfile_template_data = Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_template_uri`.\n"]
    pub fn set_dockerfile_template_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dockerfile_template_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_override`.\n"]
    pub fn set_platform_override(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_override = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().working_directory = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        self,
        v: impl Into<BlockAssignable<ImagebuilderContainerRecipeComponentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().component = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.component = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `instance_configuration`.\n"]
    pub fn set_instance_configuration(
        self,
        v: impl Into<BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `target_repository`.\n"]
    pub fn set_target_repository(
        self,
        v: impl Into<BlockAssignable<ImagebuilderContainerRecipeTargetRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_repository = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dockerfile_template_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_uri` after provisioning.\n"]
    pub fn dockerfile_template_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dockerfile_template_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_override` after provisioning.\n"]
    pub fn platform_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_override", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.working_directory", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<ImagebuilderContainerRecipeComponentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.component", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(
        &self,
    ) -> ListRef<ImagebuilderContainerRecipeInstanceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> ListRef<ImagebuilderContainerRecipeTargetRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_repository", self.extract_ref()),
        )
    }
}

impl Referable for ImagebuilderContainerRecipe {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ImagebuilderContainerRecipe {}

impl ToListMappable for ImagebuilderContainerRecipe {
    type O = ListRef<ImagebuilderContainerRecipeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ImagebuilderContainerRecipe_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_container_recipe".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderContainerRecipe {
    pub tf_id: String,
    #[doc = ""]
    pub container_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub parent_image: PrimField<String>,
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildImagebuilderContainerRecipe {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderContainerRecipe {
        let out = ImagebuilderContainerRecipe(Rc::new(ImagebuilderContainerRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderContainerRecipeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container_type: self.container_type,
                description: core::default::Default::default(),
                dockerfile_template_data: core::default::Default::default(),
                dockerfile_template_uri: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                parent_image: self.parent_image,
                platform_override: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: self.version,
                working_directory: core::default::Default::default(),
                component: core::default::Default::default(),
                instance_configuration: core::default::Default::default(),
                target_repository: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderContainerRecipeRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ImagebuilderContainerRecipeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dockerfile_template_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_uri` after provisioning.\n"]
    pub fn dockerfile_template_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dockerfile_template_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_image", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_override` after provisioning.\n"]
    pub fn platform_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_override", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.working_directory", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<ImagebuilderContainerRecipeComponentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.component", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(
        &self,
    ) -> ListRef<ImagebuilderContainerRecipeInstanceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> ListRef<ImagebuilderContainerRecipeTargetRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_repository", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeComponentElParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl ImagebuilderContainerRecipeComponentElParameterEl {}

impl ToListMappable for ImagebuilderContainerRecipeComponentElParameterEl {
    type O = BlockAssignable<ImagebuilderContainerRecipeComponentElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeComponentElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildImagebuilderContainerRecipeComponentElParameterEl {
    pub fn build(self) -> ImagebuilderContainerRecipeComponentElParameterEl {
        ImagebuilderContainerRecipeComponentElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ImagebuilderContainerRecipeComponentElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeComponentElParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderContainerRecipeComponentElParameterElRef {
        ImagebuilderContainerRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeComponentElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeComponentElDynamic {
    parameter: Option<DynamicBlock<ImagebuilderContainerRecipeComponentElParameterEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeComponentEl {
    component_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<ImagebuilderContainerRecipeComponentElParameterEl>>,
    dynamic: ImagebuilderContainerRecipeComponentElDynamic,
}

impl ImagebuilderContainerRecipeComponentEl {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderContainerRecipeComponentElParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ImagebuilderContainerRecipeComponentEl {
    type O = BlockAssignable<ImagebuilderContainerRecipeComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeComponentEl {
    #[doc = ""]
    pub component_arn: PrimField<String>,
}

impl BuildImagebuilderContainerRecipeComponentEl {
    pub fn build(self) -> ImagebuilderContainerRecipeComponentEl {
        ImagebuilderContainerRecipeComponentEl {
            component_arn: self.component_arn,
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeComponentElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderContainerRecipeComponentElRef {
        ImagebuilderContainerRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.component_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable
    for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl
{
    type O = BlockAssignable<
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    pub fn build(
        self,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElDynamic {
    ebs: Option<
        DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs: Option<Vec<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
    dynamic: ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElDynamic,
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_device = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v: impl Into<
            BlockAssignable<
                ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    type O =
        BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    pub fn build(self) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(
        &self,
    ) -> ListRef<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeInstanceConfigurationElDynamic {
    block_device_mapping: Option<
        DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_device_mapping:
        Option<Vec<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
    dynamic: ImagebuilderContainerRecipeInstanceConfigurationElDynamic,
}

impl ImagebuilderContainerRecipeInstanceConfigurationEl {
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        mut self,
        v: impl Into<
            BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.block_device_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.block_device_mapping = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ImagebuilderContainerRecipeInstanceConfigurationEl {
    type O = BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationEl {
    pub fn build(self) -> ImagebuilderContainerRecipeInstanceConfigurationEl {
        ImagebuilderContainerRecipeInstanceConfigurationEl {
            image: core::default::Default::default(),
            block_device_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeTargetRepositoryEl {
    repository_name: PrimField<String>,
    service: PrimField<String>,
}

impl ImagebuilderContainerRecipeTargetRepositoryEl {}

impl ToListMappable for ImagebuilderContainerRecipeTargetRepositoryEl {
    type O = BlockAssignable<ImagebuilderContainerRecipeTargetRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeTargetRepositoryEl {
    #[doc = ""]
    pub repository_name: PrimField<String>,
    #[doc = ""]
    pub service: PrimField<String>,
}

impl BuildImagebuilderContainerRecipeTargetRepositoryEl {
    pub fn build(self) -> ImagebuilderContainerRecipeTargetRepositoryEl {
        ImagebuilderContainerRecipeTargetRepositoryEl {
            repository_name: self.repository_name,
            service: self.service,
        }
    }
}

pub struct ImagebuilderContainerRecipeTargetRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderContainerRecipeTargetRepositoryElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderContainerRecipeTargetRepositoryElRef {
        ImagebuilderContainerRecipeTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeTargetRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeDynamic {
    component: Option<DynamicBlock<ImagebuilderContainerRecipeComponentEl>>,
    instance_configuration:
        Option<DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    target_repository: Option<DynamicBlock<ImagebuilderContainerRecipeTargetRepositoryEl>>,
}
