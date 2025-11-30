use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerPipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_description: Option<PrimField<String>>,
    pipeline_display_name: PrimField<String>,
    pipeline_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism_configuration: Option<Vec<SagemakerPipelineParallelismConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_definition_s3_location: Option<Vec<SagemakerPipelinePipelineDefinitionS3LocationEl>>,
    dynamic: SagemakerPipelineDynamic,
}

struct SagemakerPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerPipelineData>,
}

#[derive(Clone)]
pub struct SagemakerPipeline(Rc<SagemakerPipeline_>);

impl SagemakerPipeline {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_definition`.\n"]
    pub fn set_pipeline_definition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pipeline_definition = Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_description`.\n"]
    pub fn set_pipeline_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pipeline_description = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
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

    #[doc = "Set the field `parallelism_configuration`.\n"]
    pub fn set_parallelism_configuration(
        self,
        v: impl Into<BlockAssignable<SagemakerPipelineParallelismConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parallelism_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parallelism_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `pipeline_definition_s3_location`.\n"]
    pub fn set_pipeline_definition_s3_location(
        self,
        v: impl Into<BlockAssignable<SagemakerPipelinePipelineDefinitionS3LocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_definition_s3_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .pipeline_definition_s3_location = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition` after provisioning.\n"]
    pub fn pipeline_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_description` after provisioning.\n"]
    pub fn pipeline_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_display_name` after provisioning.\n"]
    pub fn pipeline_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(
        &self,
    ) -> ListRef<SagemakerPipelineParallelismConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parallelism_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_definition_s3_location` after provisioning.\n"]
    pub fn pipeline_definition_s3_location(
        &self,
    ) -> ListRef<SagemakerPipelinePipelineDefinitionS3LocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_definition_s3_location", self.extract_ref()),
        )
    }
}

impl Referable for SagemakerPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerPipeline {}

impl ToListMappable for SagemakerPipeline {
    type O = ListRef<SagemakerPipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerPipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_pipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerPipeline {
    pub tf_id: String,
    #[doc = ""]
    pub pipeline_display_name: PrimField<String>,
    #[doc = ""]
    pub pipeline_name: PrimField<String>,
}

impl BuildSagemakerPipeline {
    pub fn build(self, stack: &mut Stack) -> SagemakerPipeline {
        let out = SagemakerPipeline(Rc::new(SagemakerPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                pipeline_definition: core::default::Default::default(),
                pipeline_description: core::default::Default::default(),
                pipeline_display_name: self.pipeline_display_name,
                pipeline_name: self.pipeline_name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                parallelism_configuration: core::default::Default::default(),
                pipeline_definition_s3_location: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerPipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerPipelineRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition` after provisioning.\n"]
    pub fn pipeline_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_description` after provisioning.\n"]
    pub fn pipeline_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_display_name` after provisioning.\n"]
    pub fn pipeline_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(
        &self,
    ) -> ListRef<SagemakerPipelineParallelismConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parallelism_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_definition_s3_location` after provisioning.\n"]
    pub fn pipeline_definition_s3_location(
        &self,
    ) -> ListRef<SagemakerPipelinePipelineDefinitionS3LocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_definition_s3_location", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerPipelineParallelismConfigurationEl {
    max_parallel_execution_steps: PrimField<f64>,
}

impl SagemakerPipelineParallelismConfigurationEl {}

impl ToListMappable for SagemakerPipelineParallelismConfigurationEl {
    type O = BlockAssignable<SagemakerPipelineParallelismConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerPipelineParallelismConfigurationEl {
    #[doc = ""]
    pub max_parallel_execution_steps: PrimField<f64>,
}

impl BuildSagemakerPipelineParallelismConfigurationEl {
    pub fn build(self) -> SagemakerPipelineParallelismConfigurationEl {
        SagemakerPipelineParallelismConfigurationEl {
            max_parallel_execution_steps: self.max_parallel_execution_steps,
        }
    }
}

pub struct SagemakerPipelineParallelismConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerPipelineParallelismConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SagemakerPipelineParallelismConfigurationElRef {
        SagemakerPipelineParallelismConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerPipelineParallelismConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_parallel_execution_steps` after provisioning.\n"]
    pub fn max_parallel_execution_steps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_execution_steps", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerPipelinePipelineDefinitionS3LocationEl {
    bucket: PrimField<String>,
    object_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
}

impl SagemakerPipelinePipelineDefinitionS3LocationEl {
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerPipelinePipelineDefinitionS3LocationEl {
    type O = BlockAssignable<SagemakerPipelinePipelineDefinitionS3LocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerPipelinePipelineDefinitionS3LocationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub object_key: PrimField<String>,
}

impl BuildSagemakerPipelinePipelineDefinitionS3LocationEl {
    pub fn build(self) -> SagemakerPipelinePipelineDefinitionS3LocationEl {
        SagemakerPipelinePipelineDefinitionS3LocationEl {
            bucket: self.bucket,
            object_key: self.object_key,
            version_id: core::default::Default::default(),
        }
    }
}

pub struct SagemakerPipelinePipelineDefinitionS3LocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerPipelinePipelineDefinitionS3LocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerPipelinePipelineDefinitionS3LocationElRef {
        SagemakerPipelinePipelineDefinitionS3LocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerPipelinePipelineDefinitionS3LocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key` after provisioning.\n"]
    pub fn object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_key", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerPipelineDynamic {
    parallelism_configuration: Option<DynamicBlock<SagemakerPipelineParallelismConfigurationEl>>,
    pipeline_definition_s3_location:
        Option<DynamicBlock<SagemakerPipelinePipelineDefinitionS3LocationEl>>,
}
