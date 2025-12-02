use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ImagebuilderImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_recipe_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution_configuration_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_image_metadata_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_recipe_arn: Option<PrimField<String>>,
    infrastructure_configuration_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_configuration: Option<Vec<ImagebuilderImageImageScanningConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_configuration: Option<Vec<ImagebuilderImageImageTestsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ImagebuilderImageTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<Vec<ImagebuilderImageWorkflowEl>>,
    dynamic: ImagebuilderImageDynamic,
}
struct ImagebuilderImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderImageData>,
}
#[derive(Clone)]
pub struct ImagebuilderImage(Rc<ImagebuilderImage_>);
impl ImagebuilderImage {
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
    #[doc = "Set the field `container_recipe_arn`.\n"]
    pub fn set_container_recipe_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_recipe_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `distribution_configuration_arn`.\n"]
    pub fn set_distribution_configuration_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().distribution_configuration_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `enhanced_image_metadata_enabled`.\n"]
    pub fn set_enhanced_image_metadata_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_image_metadata_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_role`.\n"]
    pub fn set_execution_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `image_recipe_arn`.\n"]
    pub fn set_image_recipe_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_recipe_arn = Some(v.into());
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
    #[doc = "Set the field `image_scanning_configuration`.\n"]
    pub fn set_image_scanning_configuration(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImageImageScanningConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_scanning_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .image_scanning_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_tests_configuration`.\n"]
    pub fn set_image_tests_configuration(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImageImageTestsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_tests_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_tests_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ImagebuilderImageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(self, v: impl Into<BlockAssignable<ImagebuilderImageWorkflowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workflow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workflow = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_recipe_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_recipe_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.os_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> ListRef<ImagebuilderImageOutputResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<ImagebuilderImageImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> ListRef<ImagebuilderImageImageTestsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_tests_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ImagebuilderImage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ImagebuilderImage {}
impl ToListMappable for ImagebuilderImage {
    type O = ListRef<ImagebuilderImageRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ImagebuilderImage_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_image".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildImagebuilderImage {
    pub tf_id: String,
    #[doc = ""]
    pub infrastructure_configuration_arn: PrimField<String>,
}
impl BuildImagebuilderImage {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderImage {
        let out = ImagebuilderImage(Rc::new(ImagebuilderImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container_recipe_arn: core::default::Default::default(),
                distribution_configuration_arn: core::default::Default::default(),
                enhanced_image_metadata_enabled: core::default::Default::default(),
                execution_role: core::default::Default::default(),
                id: core::default::Default::default(),
                image_recipe_arn: core::default::Default::default(),
                infrastructure_configuration_arn: self.infrastructure_configuration_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                image_scanning_configuration: core::default::Default::default(),
                image_tests_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workflow: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ImagebuilderImageRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ImagebuilderImageRef {
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
    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_recipe_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_recipe_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.os_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> ListRef<ImagebuilderImageOutputResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<ImagebuilderImageImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> ListRef<ImagebuilderImageImageTestsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_tests_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageOutputResourcesElAmisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl ImagebuilderImageOutputResourcesElAmisEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageOutputResourcesElAmisEl {
    type O = BlockAssignable<ImagebuilderImageOutputResourcesElAmisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageOutputResourcesElAmisEl {}
impl BuildImagebuilderImageOutputResourcesElAmisEl {
    pub fn build(self) -> ImagebuilderImageOutputResourcesElAmisEl {
        ImagebuilderImageOutputResourcesElAmisEl {
            account_id: core::default::Default::default(),
            description: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageOutputResourcesElAmisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageOutputResourcesElAmisElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageOutputResourcesElAmisElRef {
        ImagebuilderImageOutputResourcesElAmisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageOutputResourcesElAmisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageOutputResourcesElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uris: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl ImagebuilderImageOutputResourcesElContainersEl {
    #[doc = "Set the field `image_uris`.\n"]
    pub fn set_image_uris(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.image_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageOutputResourcesElContainersEl {
    type O = BlockAssignable<ImagebuilderImageOutputResourcesElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageOutputResourcesElContainersEl {}
impl BuildImagebuilderImageOutputResourcesElContainersEl {
    pub fn build(self) -> ImagebuilderImageOutputResourcesElContainersEl {
        ImagebuilderImageOutputResourcesElContainersEl {
            image_uris: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageOutputResourcesElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageOutputResourcesElContainersElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageOutputResourcesElContainersElRef {
        ImagebuilderImageOutputResourcesElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageOutputResourcesElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_uris` after provisioning.\n"]
    pub fn image_uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.image_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageOutputResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amis: Option<SetField<ImagebuilderImageOutputResourcesElAmisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<SetField<ImagebuilderImageOutputResourcesElContainersEl>>,
}
impl ImagebuilderImageOutputResourcesEl {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(
        mut self,
        v: impl Into<SetField<ImagebuilderImageOutputResourcesElAmisEl>>,
    ) -> Self {
        self.amis = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<SetField<ImagebuilderImageOutputResourcesElContainersEl>>,
    ) -> Self {
        self.containers = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageOutputResourcesEl {
    type O = BlockAssignable<ImagebuilderImageOutputResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageOutputResourcesEl {}
impl BuildImagebuilderImageOutputResourcesEl {
    pub fn build(self) -> ImagebuilderImageOutputResourcesEl {
        ImagebuilderImageOutputResourcesEl {
            amis: core::default::Default::default(),
            containers: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageOutputResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageOutputResourcesElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageOutputResourcesElRef {
        ImagebuilderImageOutputResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageOutputResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> SetRef<ImagebuilderImageOutputResourcesElAmisElRef> {
        SetRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> SetRef<ImagebuilderImageOutputResourcesElContainersElRef> {
        SetRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
}
impl ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    #[doc = "Set the field `container_tags`.\n"]
    pub fn set_container_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.container_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_name = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    type O = BlockAssignable<ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {}
impl BuildImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    pub fn build(self) -> ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
        ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
        ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.container_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderImageImageScanningConfigurationElDynamic {
    ecr_configuration:
        Option<DynamicBlock<ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderImageImageScanningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_configuration: Option<Vec<ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>>,
    dynamic: ImagebuilderImageImageScanningConfigurationElDynamic,
}
impl ImagebuilderImageImageScanningConfigurationEl {
    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_scanning_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderImageImageScanningConfigurationEl {
    type O = BlockAssignable<ImagebuilderImageImageScanningConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageImageScanningConfigurationEl {}
impl BuildImagebuilderImageImageScanningConfigurationEl {
    pub fn build(self) -> ImagebuilderImageImageScanningConfigurationEl {
        ImagebuilderImageImageScanningConfigurationEl {
            image_scanning_enabled: core::default::Default::default(),
            ecr_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderImageImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageImageScanningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageImageScanningConfigurationElRef {
        ImagebuilderImageImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageImageScanningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_scanning_enabled` after provisioning.\n"]
    pub fn image_scanning_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_scanning_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ecr_configuration` after provisioning.\n"]
    pub fn ecr_configuration(
        &self,
    ) -> ListRef<ImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecr_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_minutes: Option<PrimField<f64>>,
}
impl ImagebuilderImageImageTestsConfigurationEl {
    #[doc = "Set the field `image_tests_enabled`.\n"]
    pub fn set_image_tests_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_tests_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_minutes = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageImageTestsConfigurationEl {
    type O = BlockAssignable<ImagebuilderImageImageTestsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageImageTestsConfigurationEl {}
impl BuildImagebuilderImageImageTestsConfigurationEl {
    pub fn build(self) -> ImagebuilderImageImageTestsConfigurationEl {
        ImagebuilderImageImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageImageTestsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageImageTestsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageImageTestsConfigurationElRef {
        ImagebuilderImageImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageImageTestsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_tests_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_minutes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl ImagebuilderImageTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderImageTimeoutsEl {
    type O = BlockAssignable<ImagebuilderImageTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageTimeoutsEl {}
impl BuildImagebuilderImageTimeoutsEl {
    pub fn build(self) -> ImagebuilderImageTimeoutsEl {
        ImagebuilderImageTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderImageTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderImageWorkflowElParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl ImagebuilderImageWorkflowElParameterEl {}
impl ToListMappable for ImagebuilderImageWorkflowElParameterEl {
    type O = BlockAssignable<ImagebuilderImageWorkflowElParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageWorkflowElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildImagebuilderImageWorkflowElParameterEl {
    pub fn build(self) -> ImagebuilderImageWorkflowElParameterEl {
        ImagebuilderImageWorkflowElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct ImagebuilderImageWorkflowElParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageWorkflowElParameterElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageWorkflowElParameterElRef {
        ImagebuilderImageWorkflowElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageWorkflowElParameterElRef {
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
struct ImagebuilderImageWorkflowElDynamic {
    parameter: Option<DynamicBlock<ImagebuilderImageWorkflowElParameterEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderImageWorkflowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_group: Option<PrimField<String>>,
    workflow_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<ImagebuilderImageWorkflowElParameterEl>>,
    dynamic: ImagebuilderImageWorkflowElDynamic,
}
impl ImagebuilderImageWorkflowEl {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `parallel_group`.\n"]
    pub fn set_parallel_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parallel_group = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderImageWorkflowElParameterEl>>,
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
impl ToListMappable for ImagebuilderImageWorkflowEl {
    type O = BlockAssignable<ImagebuilderImageWorkflowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderImageWorkflowEl {
    #[doc = ""]
    pub workflow_arn: PrimField<String>,
}
impl BuildImagebuilderImageWorkflowEl {
    pub fn build(self) -> ImagebuilderImageWorkflowEl {
        ImagebuilderImageWorkflowEl {
            on_failure: core::default::Default::default(),
            parallel_group: core::default::Default::default(),
            workflow_arn: self.workflow_arn,
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderImageWorkflowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderImageWorkflowElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageWorkflowElRef {
        ImagebuilderImageWorkflowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderImageWorkflowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }
    #[doc = "Get a reference to the value of field `parallel_group` after provisioning.\n"]
    pub fn parallel_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parallel_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workflow_arn` after provisioning.\n"]
    pub fn workflow_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workflow_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderImageDynamic {
    image_scanning_configuration:
        Option<DynamicBlock<ImagebuilderImageImageScanningConfigurationEl>>,
    image_tests_configuration: Option<DynamicBlock<ImagebuilderImageImageTestsConfigurationEl>>,
    workflow: Option<DynamicBlock<ImagebuilderImageWorkflowEl>>,
}
