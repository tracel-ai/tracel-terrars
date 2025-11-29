use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDataQualityJobDefinitionData {
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
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_quality_app_specification: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_quality_baseline_config: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_quality_job_input: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_quality_job_output_config: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_resources: Option<Vec<SagemakerDataQualityJobDefinitionJobResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<SagemakerDataQualityJobDefinitionNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stopping_condition: Option<Vec<SagemakerDataQualityJobDefinitionStoppingConditionEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDynamic,
}

struct SagemakerDataQualityJobDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerDataQualityJobDefinitionData>,
}

#[derive(Clone)]
pub struct SagemakerDataQualityJobDefinition(Rc<SagemakerDataQualityJobDefinition_>);

impl SagemakerDataQualityJobDefinition {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `data_quality_app_specification`.\n"]
    pub fn set_data_quality_app_specification(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_app_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_app_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_baseline_config`.\n"]
    pub fn set_data_quality_baseline_config(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_baseline_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_baseline_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_job_input`.\n"]
    pub fn set_data_quality_job_input(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_job_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_job_input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_job_output_config`.\n"]
    pub fn set_data_quality_job_output_config(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_job_output_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_job_output_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_resources`.\n"]
    pub fn set_job_resources(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionJobResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_resources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_resources = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stopping_condition`.\n"]
    pub fn set_stopping_condition(
        self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionStoppingConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stopping_condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stopping_condition = Some(d);
            },
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_app_specification` after provisioning.\n"]
    pub fn data_quality_app_specification(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_app_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_baseline_config` after provisioning.\n"]
    pub fn data_quality_baseline_config(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_baseline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_input` after provisioning.\n"]
    pub fn data_quality_job_input(&self) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_job_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_output_config` after provisioning.\n"]
    pub fn data_quality_job_output_config(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_job_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_resources` after provisioning.\n"]
    pub fn job_resources(&self) -> ListRef<SagemakerDataQualityJobDefinitionJobResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<SagemakerDataQualityJobDefinitionNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> ListRef<SagemakerDataQualityJobDefinitionStoppingConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }
}

impl Referable for SagemakerDataQualityJobDefinition {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerDataQualityJobDefinition { }

impl ToListMappable for SagemakerDataQualityJobDefinition {
    type O = ListRef<SagemakerDataQualityJobDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerDataQualityJobDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_data_quality_job_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDataQualityJobDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildSagemakerDataQualityJobDefinition {
    pub fn build(self, stack: &mut Stack) -> SagemakerDataQualityJobDefinition {
        let out = SagemakerDataQualityJobDefinition(Rc::new(SagemakerDataQualityJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerDataQualityJobDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_quality_app_specification: core::default::Default::default(),
                data_quality_baseline_config: core::default::Default::default(),
                data_quality_job_input: core::default::Default::default(),
                data_quality_job_output_config: core::default::Default::default(),
                job_resources: core::default::Default::default(),
                network_config: core::default::Default::default(),
                stopping_condition: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDataQualityJobDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerDataQualityJobDefinitionRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_app_specification` after provisioning.\n"]
    pub fn data_quality_app_specification(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_app_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_baseline_config` after provisioning.\n"]
    pub fn data_quality_baseline_config(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_baseline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_input` after provisioning.\n"]
    pub fn data_quality_job_input(&self) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_job_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_output_config` after provisioning.\n"]
    pub fn data_quality_job_output_config(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_quality_job_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_resources` after provisioning.\n"]
    pub fn job_resources(&self) -> ListRef<SagemakerDataQualityJobDefinitionJobResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<SagemakerDataQualityJobDefinitionNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> ListRef<SagemakerDataQualityJobDefinitionStoppingConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<RecField<PrimField<String>>>,
    image_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_analytics_processor_source_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_preprocessor_source_uri: Option<PrimField<String>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment = Some(v.into());
        self
    }

    #[doc = "Set the field `post_analytics_processor_source_uri`.\n"]
    pub fn set_post_analytics_processor_source_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_analytics_processor_source_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `record_preprocessor_source_uri`.\n"]
    pub fn set_record_preprocessor_source_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_preprocessor_source_uri = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
    #[doc = ""]
    pub image_uri: PrimField<String>,
}

impl BuildSagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
        SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl {
            environment: core::default::Default::default(),
            image_uri: self.image_uri,
            post_analytics_processor_source_uri: core::default::Default::default(),
            record_preprocessor_source_uri: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef {
        SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityAppSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `post_analytics_processor_source_uri` after provisioning.\n"]
    pub fn post_analytics_processor_source_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.post_analytics_processor_source_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `record_preprocessor_source_uri` after provisioning.\n"]
    pub fn record_preprocessor_source_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_preprocessor_source_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl {
            s3_uri: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl {
            s3_uri: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElDynamic {
    constraints_resource: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl>,
    >,
    statistics_resource: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints_resource: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics_resource: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
    #[doc = "Set the field `constraints_resource`.\n"]
    pub fn set_constraints_resource(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.constraints_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.constraints_resource = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statistics_resource`.\n"]
    pub fn set_statistics_resource(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.statistics_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.statistics_resource = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl {
            constraints_resource: core::default::Default::default(),
            statistics_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef {
        SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints_resource` after provisioning.\n"]
    pub fn constraints_resource(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElConstraintsResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.constraints_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `statistics_resource` after provisioning.\n"]
    pub fn statistics_resource(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigElStatisticsResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.statistics_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<bool>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.header = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
    type O =
        BlockAssignable<
            SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
    pub fn build(
        self,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl {
            header: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<PrimField<bool>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
    #[doc = "Set the field `line`.\n"]
    pub fn set_line(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.line = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
    type O =
        BlockAssignable<
            SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
    pub fn build(
        self,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl {
            line: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `line` after provisioning.\n"]
    pub fn line(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.line", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElDynamic {
    csv: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl>,
    >,
    json: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
    type O =
        BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElCsvElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDynamic {
    dataset_format: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
    data_captured_destination_s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_distribution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_input_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_format: Option<
        Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl>,
    >,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_data_distribution_type = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_input_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `dataset_format`.\n"]
    pub fn set_dataset_format(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset_format = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_format = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
    #[doc = ""]
    pub data_captured_destination_s3_uri: PrimField<String>,
}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl {
            data_captured_destination_s3_uri: self.data_captured_destination_s3_uri,
            local_path: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
            dataset_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_captured_destination_s3_uri` after provisioning.\n"]
    pub fn data_captured_destination_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_captured_destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_format` after provisioning.\n"]
    pub fn dataset_format(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElDatasetFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset_format", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
    endpoint_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_distribution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_input_mode: Option<PrimField<String>>,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_data_distribution_type = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_input_mode = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
    #[doc = ""]
    pub endpoint_name: PrimField<String>,
}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl {
            endpoint_name: self.endpoint_name,
            local_path: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityJobInputElDynamic {
    batch_transform_input: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl>,
    >,
    endpoint_input: Option<DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_transform_input: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_input: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityJobInputElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputEl {
    #[doc = "Set the field `batch_transform_input`.\n"]
    pub fn set_batch_transform_input(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.batch_transform_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.batch_transform_input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_input`.\n"]
    pub fn set_endpoint_input(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.endpoint_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_input = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobInputEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobInputEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobInputEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobInputEl {
        SagemakerDataQualityJobDefinitionDataQualityJobInputEl {
            batch_transform_input: core::default::Default::default(),
            endpoint_input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobInputElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionDataQualityJobInputElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_transform_input` after provisioning.\n"]
    pub fn batch_transform_input(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElBatchTransformInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.batch_transform_input", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_input` after provisioning.\n"]
    pub fn endpoint_input(&self) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobInputElEndpointInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_input", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_upload_mode: Option<PrimField<String>>,
    s3_uri: PrimField<String>,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_upload_mode`.\n"]
    pub fn set_s3_upload_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_upload_mode = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
    type O =
        BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl {
            local_path: core::default::Default::default(),
            s3_upload_mode: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_upload_mode` after provisioning.\n"]
    pub fn s3_upload_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_upload_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElDynamic {
    s3_output: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
    #[doc = "Set the field `s3_output`.\n"]
    pub fn set_s3_output(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl {
            s3_output: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output` after provisioning.\n"]
    pub fn s3_output(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElS3OutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_output", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElDynamic {
    monitoring_outputs: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_outputs: Option<Vec<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl>>,
    dynamic: SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElDynamic,
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_outputs`.\n"]
    pub fn set_monitoring_outputs(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monitoring_outputs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_outputs = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {}

impl BuildSagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl {
            kms_key_id: core::default::Default::default(),
            monitoring_outputs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef {
        SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_outputs` after provisioning.\n"]
    pub fn monitoring_outputs(
        &self,
    ) -> ListRef<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigElMonitoringOutputsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
    instance_count: PrimField<f64>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_kms_key_id: Option<PrimField<String>>,
    volume_size_in_gb: PrimField<f64>,
}

impl SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
    #[doc = ""]
    pub instance_count: PrimField<f64>,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub volume_size_in_gb: PrimField<f64>,
}

impl BuildSagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
        SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl {
            instance_count: self.instance_count,
            instance_type: self.instance_type,
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: self.volume_size_in_gb,
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef {
        SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionJobResourcesElDynamic {
    cluster_config: Option<DynamicBlock<SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionJobResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_config: Option<Vec<SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl>>,
    dynamic: SagemakerDataQualityJobDefinitionJobResourcesElDynamic,
}

impl SagemakerDataQualityJobDefinitionJobResourcesEl {
    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cluster_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionJobResourcesEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionJobResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionJobResourcesEl {}

impl BuildSagemakerDataQualityJobDefinitionJobResourcesEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionJobResourcesEl {
        SagemakerDataQualityJobDefinitionJobResourcesEl {
            cluster_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionJobResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionJobResourcesElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionJobResourcesElRef {
        SagemakerDataQualityJobDefinitionJobResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionJobResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<SagemakerDataQualityJobDefinitionJobResourcesElClusterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}

impl SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl { }

impl ToListMappable for SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildSagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
        SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef {
        SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionNetworkConfigElDynamic {
    vpc_config: Option<DynamicBlock<SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_inter_container_traffic_encryption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_isolation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl>>,
    dynamic: SagemakerDataQualityJobDefinitionNetworkConfigElDynamic,
}

impl SagemakerDataQualityJobDefinitionNetworkConfigEl {
    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_inter_container_traffic_encryption = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_network_isolation = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionNetworkConfigEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionNetworkConfigEl {}

impl BuildSagemakerDataQualityJobDefinitionNetworkConfigEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionNetworkConfigEl {
        SagemakerDataQualityJobDefinitionNetworkConfigEl {
            enable_inter_container_traffic_encryption: core::default::Default::default(),
            enable_network_isolation: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionNetworkConfigElRef {
        SagemakerDataQualityJobDefinitionNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_inter_container_traffic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<SagemakerDataQualityJobDefinitionNetworkConfigElVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinitionStoppingConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_runtime_in_seconds: Option<PrimField<f64>>,
}

impl SagemakerDataQualityJobDefinitionStoppingConditionEl {
    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDataQualityJobDefinitionStoppingConditionEl {
    type O = BlockAssignable<SagemakerDataQualityJobDefinitionStoppingConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinitionStoppingConditionEl {}

impl BuildSagemakerDataQualityJobDefinitionStoppingConditionEl {
    pub fn build(self) -> SagemakerDataQualityJobDefinitionStoppingConditionEl {
        SagemakerDataQualityJobDefinitionStoppingConditionEl {
            max_runtime_in_seconds: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDataQualityJobDefinitionStoppingConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDataQualityJobDefinitionStoppingConditionElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDataQualityJobDefinitionStoppingConditionElRef {
        SagemakerDataQualityJobDefinitionStoppingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinitionStoppingConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDynamic {
    data_quality_app_specification: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityAppSpecificationEl>,
    >,
    data_quality_baseline_config: Option<DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityBaselineConfigEl>>,
    data_quality_job_input: Option<DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobInputEl>>,
    data_quality_job_output_config: Option<
        DynamicBlock<SagemakerDataQualityJobDefinitionDataQualityJobOutputConfigEl>,
    >,
    job_resources: Option<DynamicBlock<SagemakerDataQualityJobDefinitionJobResourcesEl>>,
    network_config: Option<DynamicBlock<SagemakerDataQualityJobDefinitionNetworkConfigEl>>,
    stopping_condition: Option<DynamicBlock<SagemakerDataQualityJobDefinitionStoppingConditionEl>>,
}
