use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ImagebuilderImagePipelineData {
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
    description: Option<PrimField<String>>,
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_configuration:
        Option<Vec<ImagebuilderImagePipelineImageScanningConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_configuration: Option<Vec<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<ImagebuilderImagePipelineScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<Vec<ImagebuilderImagePipelineWorkflowEl>>,
    dynamic: ImagebuilderImagePipelineDynamic,
}

struct ImagebuilderImagePipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderImagePipelineData>,
}

#[derive(Clone)]
pub struct ImagebuilderImagePipeline(Rc<ImagebuilderImagePipeline_>);

impl ImagebuilderImagePipeline {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
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
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineImageScanningConfigurationEl>>,
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
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
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

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineWorkflowEl>>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_last_run", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_next_run", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<ImagebuilderImagePipelineImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_tests_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<ImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> ListRef<ImagebuilderImagePipelineWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workflow", self.extract_ref()),
        )
    }
}

impl Referable for ImagebuilderImagePipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ImagebuilderImagePipeline {}

impl ToListMappable for ImagebuilderImagePipeline {
    type O = ListRef<ImagebuilderImagePipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ImagebuilderImagePipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_image_pipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImagePipeline {
    pub tf_id: String,
    #[doc = ""]
    pub infrastructure_configuration_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildImagebuilderImagePipeline {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderImagePipeline {
        let out = ImagebuilderImagePipeline(Rc::new(ImagebuilderImagePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderImagePipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container_recipe_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution_configuration_arn: core::default::Default::default(),
                enhanced_image_metadata_enabled: core::default::Default::default(),
                execution_role: core::default::Default::default(),
                id: core::default::Default::default(),
                image_recipe_arn: core::default::Default::default(),
                infrastructure_configuration_arn: self.infrastructure_configuration_arn,
                name: self.name,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                image_scanning_configuration: core::default::Default::default(),
                image_tests_configuration: core::default::Default::default(),
                schedule: core::default::Default::default(),
                workflow: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImagePipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ImagebuilderImagePipelineRef {
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

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_last_run", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_next_run", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> ListRef<ImagebuilderImagePipelineImageScanningConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_scanning_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_tests_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<ImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> ListRef<ImagebuilderImagePipelineWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workflow", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
}

impl ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
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

impl ToListMappable for ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    type O =
        BlockAssignable<ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {}

impl BuildImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    pub fn build(self) -> ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
        ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
        ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
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
struct ImagebuilderImagePipelineImageScanningConfigurationElDynamic {
    ecr_configuration: Option<
        DynamicBlock<ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineImageScanningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_configuration:
        Option<Vec<ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>>,
    dynamic: ImagebuilderImagePipelineImageScanningConfigurationElDynamic,
}

impl ImagebuilderImagePipelineImageScanningConfigurationEl {
    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_scanning_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl,
            >,
        >,
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

impl ToListMappable for ImagebuilderImagePipelineImageScanningConfigurationEl {
    type O = BlockAssignable<ImagebuilderImagePipelineImageScanningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineImageScanningConfigurationEl {}

impl BuildImagebuilderImagePipelineImageScanningConfigurationEl {
    pub fn build(self) -> ImagebuilderImagePipelineImageScanningConfigurationEl {
        ImagebuilderImagePipelineImageScanningConfigurationEl {
            image_scanning_enabled: core::default::Default::default(),
            ecr_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineImageScanningConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderImagePipelineImageScanningConfigurationElRef {
        ImagebuilderImagePipelineImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineImageScanningConfigurationElRef {
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
    ) -> ListRef<ImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecr_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_minutes: Option<PrimField<f64>>,
}

impl ImagebuilderImagePipelineImageTestsConfigurationEl {
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

impl ToListMappable for ImagebuilderImagePipelineImageTestsConfigurationEl {
    type O = BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineImageTestsConfigurationEl {}

impl BuildImagebuilderImagePipelineImageTestsConfigurationEl {
    pub fn build(self) -> ImagebuilderImagePipelineImageTestsConfigurationEl {
        ImagebuilderImagePipelineImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineImageTestsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderImagePipelineImageTestsConfigurationElRef {
        ImagebuilderImagePipelineImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineImageTestsConfigurationElRef {
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
pub struct ImagebuilderImagePipelineScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_execution_start_condition: Option<PrimField<String>>,
    schedule_expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
}

impl ImagebuilderImagePipelineScheduleEl {
    #[doc = "Set the field `pipeline_execution_start_condition`.\n"]
    pub fn set_pipeline_execution_start_condition(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.pipeline_execution_start_condition = Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timezone = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderImagePipelineScheduleEl {
    type O = BlockAssignable<ImagebuilderImagePipelineScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineScheduleEl {
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildImagebuilderImagePipelineScheduleEl {
    pub fn build(self) -> ImagebuilderImagePipelineScheduleEl {
        ImagebuilderImagePipelineScheduleEl {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            timezone: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineScheduleElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImagePipelineScheduleElRef {
        ImagebuilderImagePipelineScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_execution_start_condition` after provisioning.\n"]
    pub fn pipeline_execution_start_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_execution_start_condition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_expression", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineWorkflowElParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl ImagebuilderImagePipelineWorkflowElParameterEl {}

impl ToListMappable for ImagebuilderImagePipelineWorkflowElParameterEl {
    type O = BlockAssignable<ImagebuilderImagePipelineWorkflowElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineWorkflowElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildImagebuilderImagePipelineWorkflowElParameterEl {
    pub fn build(self) -> ImagebuilderImagePipelineWorkflowElParameterEl {
        ImagebuilderImagePipelineWorkflowElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ImagebuilderImagePipelineWorkflowElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineWorkflowElParameterElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImagePipelineWorkflowElParameterElRef {
        ImagebuilderImagePipelineWorkflowElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineWorkflowElParameterElRef {
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
struct ImagebuilderImagePipelineWorkflowElDynamic {
    parameter: Option<DynamicBlock<ImagebuilderImagePipelineWorkflowElParameterEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineWorkflowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_group: Option<PrimField<String>>,
    workflow_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<ImagebuilderImagePipelineWorkflowElParameterEl>>,
    dynamic: ImagebuilderImagePipelineWorkflowElDynamic,
}

impl ImagebuilderImagePipelineWorkflowEl {
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
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineWorkflowElParameterEl>>,
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

impl ToListMappable for ImagebuilderImagePipelineWorkflowEl {
    type O = BlockAssignable<ImagebuilderImagePipelineWorkflowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineWorkflowEl {
    #[doc = ""]
    pub workflow_arn: PrimField<String>,
}

impl BuildImagebuilderImagePipelineWorkflowEl {
    pub fn build(self) -> ImagebuilderImagePipelineWorkflowEl {
        ImagebuilderImagePipelineWorkflowEl {
            on_failure: core::default::Default::default(),
            parallel_group: core::default::Default::default(),
            workflow_arn: self.workflow_arn,
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineWorkflowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineWorkflowElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImagePipelineWorkflowElRef {
        ImagebuilderImagePipelineWorkflowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineWorkflowElRef {
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
struct ImagebuilderImagePipelineDynamic {
    image_scanning_configuration:
        Option<DynamicBlock<ImagebuilderImagePipelineImageScanningConfigurationEl>>,
    image_tests_configuration:
        Option<DynamicBlock<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    schedule: Option<DynamicBlock<ImagebuilderImagePipelineScheduleEl>>,
    workflow: Option<DynamicBlock<ImagebuilderImagePipelineWorkflowEl>>,
}
