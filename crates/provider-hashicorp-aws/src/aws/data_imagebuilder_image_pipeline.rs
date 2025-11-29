use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderImagePipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataImagebuilderImagePipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderImagePipelineData>,
}

#[derive(Clone)]
pub struct DataImagebuilderImagePipeline(Rc<DataImagebuilderImagePipeline_>);

impl DataImagebuilderImagePipeline {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataImagebuilderImagePipelineImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<DataImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DataImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataImagebuilderImagePipeline {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataImagebuilderImagePipeline { }

impl ToListMappable for DataImagebuilderImagePipeline {
    type O = ListRef<DataImagebuilderImagePipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderImagePipeline_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_image_pipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderImagePipeline {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataImagebuilderImagePipeline {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderImagePipeline {
        let out = DataImagebuilderImagePipeline(Rc::new(DataImagebuilderImagePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderImagePipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderImagePipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImagePipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataImagebuilderImagePipelineRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataImagebuilderImagePipelineImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<DataImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DataImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
}

impl DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
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

impl ToListMappable for DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {}

impl BuildDataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
    pub fn build(self) -> DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
        DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
        DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.container_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipelineImageScanningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_configuration: Option<ListField<DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_enabled: Option<PrimField<bool>>,
}

impl DataImagebuilderImagePipelineImageScanningConfigurationEl {
    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v: impl Into<ListField<DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationEl>>,
    ) -> Self {
        self.ecr_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_scanning_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImagePipelineImageScanningConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImagePipelineImageScanningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineImageScanningConfigurationEl {}

impl BuildDataImagebuilderImagePipelineImageScanningConfigurationEl {
    pub fn build(self) -> DataImagebuilderImagePipelineImageScanningConfigurationEl {
        DataImagebuilderImagePipelineImageScanningConfigurationEl {
            ecr_configuration: core::default::Default::default(),
            image_scanning_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImagePipelineImageScanningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImagePipelineImageScanningConfigurationElRef {
        DataImagebuilderImagePipelineImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineImageScanningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ecr_configuration` after provisioning.\n"]
    pub fn ecr_configuration(
        &self,
    ) -> ListRef<DataImagebuilderImagePipelineImageScanningConfigurationElEcrConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ecr_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `image_scanning_enabled` after provisioning.\n"]
    pub fn image_scanning_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_scanning_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipelineImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_minutes: Option<PrimField<f64>>,
}

impl DataImagebuilderImagePipelineImageTestsConfigurationEl {
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

impl ToListMappable for DataImagebuilderImagePipelineImageTestsConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImagePipelineImageTestsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineImageTestsConfigurationEl {}

impl BuildDataImagebuilderImagePipelineImageTestsConfigurationEl {
    pub fn build(self) -> DataImagebuilderImagePipelineImageTestsConfigurationEl {
        DataImagebuilderImagePipelineImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineImageTestsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImagePipelineImageTestsConfigurationElRef {
        DataImagebuilderImagePipelineImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tests_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipelineScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_execution_start_condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression: Option<PrimField<String>>,
}

impl DataImagebuilderImagePipelineScheduleEl {
    #[doc = "Set the field `pipeline_execution_start_condition`.\n"]
    pub fn set_pipeline_execution_start_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pipeline_execution_start_condition = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_expression = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImagePipelineScheduleEl {
    type O = BlockAssignable<DataImagebuilderImagePipelineScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineScheduleEl {}

impl BuildDataImagebuilderImagePipelineScheduleEl {
    pub fn build(self) -> DataImagebuilderImagePipelineScheduleEl {
        DataImagebuilderImagePipelineScheduleEl {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImagePipelineScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImagePipelineScheduleElRef {
        DataImagebuilderImagePipelineScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_execution_start_condition` after provisioning.\n"]
    pub fn pipeline_execution_start_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_execution_start_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}
