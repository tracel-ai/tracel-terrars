use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerEndpointConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    async_inference_config: Option<Vec<SagemakerEndpointConfigurationAsyncInferenceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_capture_config: Option<Vec<SagemakerEndpointConfigurationDataCaptureConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production_variants: Option<Vec<SagemakerEndpointConfigurationProductionVariantsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_production_variants: Option<Vec<SagemakerEndpointConfigurationShadowProductionVariantsEl>>,
    dynamic: SagemakerEndpointConfigurationDynamic,
}

struct SagemakerEndpointConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerEndpointConfigurationData>,
}

#[derive(Clone)]
pub struct SagemakerEndpointConfiguration(Rc<SagemakerEndpointConfiguration_>);

impl SagemakerEndpointConfiguration {
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

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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

    #[doc = "Set the field `async_inference_config`.\n"]
    pub fn set_async_inference_config(
        self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().async_inference_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.async_inference_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_capture_config`.\n"]
    pub fn set_data_capture_config(
        self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_capture_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_capture_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `production_variants`.\n"]
    pub fn set_production_variants(
        self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationProductionVariantsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().production_variants = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.production_variants = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shadow_production_variants`.\n"]
    pub fn set_shadow_production_variants(
        self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shadow_production_variants = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shadow_production_variants = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `async_inference_config` after provisioning.\n"]
    pub fn async_inference_config(&self) -> ListRef<SagemakerEndpointConfigurationAsyncInferenceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.async_inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_capture_config` after provisioning.\n"]
    pub fn data_capture_config(&self) -> ListRef<SagemakerEndpointConfigurationDataCaptureConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_capture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_variants` after provisioning.\n"]
    pub fn production_variants(&self) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.production_variants", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shadow_production_variants` after provisioning.\n"]
    pub fn shadow_production_variants(&self) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shadow_production_variants", self.extract_ref()))
    }
}

impl Referable for SagemakerEndpointConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerEndpointConfiguration { }

impl ToListMappable for SagemakerEndpointConfiguration {
    type O = ListRef<SagemakerEndpointConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerEndpointConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_endpoint_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerEndpointConfiguration {
    pub tf_id: String,
}

impl BuildSagemakerEndpointConfiguration {
    pub fn build(self, stack: &mut Stack) -> SagemakerEndpointConfiguration {
        let out = SagemakerEndpointConfiguration(Rc::new(SagemakerEndpointConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerEndpointConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                execution_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                async_inference_config: core::default::Default::default(),
                data_capture_config: core::default::Default::default(),
                production_variants: core::default::Default::default(),
                shadow_production_variants: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerEndpointConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerEndpointConfigurationRef {
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

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `async_inference_config` after provisioning.\n"]
    pub fn async_inference_config(&self) -> ListRef<SagemakerEndpointConfigurationAsyncInferenceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.async_inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_capture_config` after provisioning.\n"]
    pub fn data_capture_config(&self) -> ListRef<SagemakerEndpointConfigurationDataCaptureConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_capture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_variants` after provisioning.\n"]
    pub fn production_variants(&self) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.production_variants", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shadow_production_variants` after provisioning.\n"]
    pub fn shadow_production_variants(&self) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shadow_production_variants", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_invocations_per_instance: Option<PrimField<f64>>,
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
    #[doc = "Set the field `max_concurrent_invocations_per_instance`.\n"]
    pub fn set_max_concurrent_invocations_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_invocations_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {}

impl BuildSagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
        SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl {
            max_concurrent_invocations_per_instance: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef {
        SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_invocations_per_instance` after provisioning.\n"]
    pub fn max_concurrent_invocations_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_invocations_per_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_inference_response_in: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_topic: Option<PrimField<String>>,
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
    #[doc = "Set the field `error_topic`.\n"]
    pub fn set_error_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_topic = Some(v.into());
        self
    }

    #[doc = "Set the field `include_inference_response_in`.\n"]
    pub fn set_include_inference_response_in(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include_inference_response_in = Some(v.into());
        self
    }

    #[doc = "Set the field `success_topic`.\n"]
    pub fn set_success_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success_topic = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {}

impl BuildSagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
        SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl {
            error_topic: core::default::Default::default(),
            include_inference_response_in: core::default::Default::default(),
            success_topic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef {
        SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_topic` after provisioning.\n"]
    pub fn error_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_topic", self.base))
    }

    #[doc = "Get a reference to the value of field `include_inference_response_in` after provisioning.\n"]
    pub fn include_inference_response_in(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include_inference_response_in", self.base))
    }

    #[doc = "Get a reference to the value of field `success_topic` after provisioning.\n"]
    pub fn success_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElDynamic {
    notification_config: Option<
        DynamicBlock<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_failure_path: Option<PrimField<String>>,
    s3_output_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<
        Vec<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl>,
    >,
    dynamic: SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElDynamic,
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_failure_path`.\n"]
    pub fn set_s3_failure_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_failure_path = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
    #[doc = ""]
    pub s3_output_path: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
        SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl {
            kms_key_id: core::default::Default::default(),
            s3_failure_path: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
            notification_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef {
        SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_failure_path` after provisioning.\n"]
    pub fn s3_failure_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_failure_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationAsyncInferenceConfigElDynamic {
    client_config: Option<DynamicBlock<SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl>>,
    output_config: Option<DynamicBlock<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationAsyncInferenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_config: Option<Vec<SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_config: Option<Vec<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl>>,
    dynamic: SagemakerEndpointConfigurationAsyncInferenceConfigElDynamic,
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigEl {
    #[doc = "Set the field `client_config`.\n"]
    pub fn set_client_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationAsyncInferenceConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationAsyncInferenceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationAsyncInferenceConfigEl {}

impl BuildSagemakerEndpointConfigurationAsyncInferenceConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationAsyncInferenceConfigEl {
        SagemakerEndpointConfigurationAsyncInferenceConfigEl {
            client_config: core::default::Default::default(),
            output_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationAsyncInferenceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationAsyncInferenceConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointConfigurationAsyncInferenceConfigElRef {
        SagemakerEndpointConfigurationAsyncInferenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationAsyncInferenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_config` after provisioning.\n"]
    pub fn client_config(&self) -> ListRef<SagemakerEndpointConfigurationAsyncInferenceConfigElClientConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> ListRef<SagemakerEndpointConfigurationAsyncInferenceConfigElOutputConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_content_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_content_types: Option<SetField<PrimField<String>>>,
}

impl SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
    #[doc = "Set the field `csv_content_types`.\n"]
    pub fn set_csv_content_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.csv_content_types = Some(v.into());
        self
    }

    #[doc = "Set the field `json_content_types`.\n"]
    pub fn set_json_content_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.json_content_types = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {}

impl BuildSagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
    pub fn build(self) -> SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
        SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl {
            csv_content_types: core::default::Default::default(),
            json_content_types: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef {
        SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_content_types` after provisioning.\n"]
    pub fn csv_content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.csv_content_types", self.base))
    }

    #[doc = "Get a reference to the value of field `json_content_types` after provisioning.\n"]
    pub fn json_content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.json_content_types", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl {
    capture_mode: PrimField<String>,
}

impl SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl { }

impl ToListMappable for SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl {
    #[doc = ""]
    pub capture_mode: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl {
    pub fn build(self) -> SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl {
        SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl { capture_mode: self.capture_mode }
    }
}

pub struct SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef {
        SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capture_mode` after provisioning.\n"]
    pub fn capture_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capture_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationDataCaptureConfigElDynamic {
    capture_content_type_header: Option<
        DynamicBlock<SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl>,
    >,
    capture_options: Option<DynamicBlock<SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationDataCaptureConfigEl {
    destination_s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_capture: Option<PrimField<bool>>,
    initial_sampling_percentage: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_content_type_header: Option<Vec<SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_options: Option<Vec<SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl>>,
    dynamic: SagemakerEndpointConfigurationDataCaptureConfigElDynamic,
}

impl SagemakerEndpointConfigurationDataCaptureConfigEl {
    #[doc = "Set the field `enable_capture`.\n"]
    pub fn set_enable_capture(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_capture = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `capture_content_type_header`.\n"]
    pub fn set_capture_content_type_header(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capture_content_type_header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capture_content_type_header = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capture_options`.\n"]
    pub fn set_capture_options(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capture_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capture_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationDataCaptureConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationDataCaptureConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationDataCaptureConfigEl {
    #[doc = ""]
    pub destination_s3_uri: PrimField<String>,
    #[doc = ""]
    pub initial_sampling_percentage: PrimField<f64>,
}

impl BuildSagemakerEndpointConfigurationDataCaptureConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationDataCaptureConfigEl {
        SagemakerEndpointConfigurationDataCaptureConfigEl {
            destination_s3_uri: self.destination_s3_uri,
            enable_capture: core::default::Default::default(),
            initial_sampling_percentage: self.initial_sampling_percentage,
            kms_key_id: core::default::Default::default(),
            capture_content_type_header: core::default::Default::default(),
            capture_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationDataCaptureConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationDataCaptureConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointConfigurationDataCaptureConfigElRef {
        SagemakerEndpointConfigurationDataCaptureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationDataCaptureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_capture` after provisioning.\n"]
    pub fn enable_capture(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_capture", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_sampling_percentage` after provisioning.\n"]
    pub fn initial_sampling_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_sampling_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_content_type_header` after provisioning.\n"]
    pub fn capture_content_type_header(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationDataCaptureConfigElCaptureContentTypeHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capture_content_type_header", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_options` after provisioning.\n"]
    pub fn capture_options(&self) -> ListRef<SagemakerEndpointConfigurationDataCaptureConfigElCaptureOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capture_options", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
    destination_s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
    #[doc = ""]
    pub destination_s3_uri: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
        SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl {
            destination_s3_uri: self.destination_s3_uri,
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef {
        SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {}

impl BuildSagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
    pub fn build(self) -> SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
        SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef {
        SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl {
    routing_strategy: PrimField<String>,
}

impl SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl { }

impl ToListMappable for SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl {
    #[doc = ""]
    pub routing_strategy: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl {
        SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl { routing_strategy: self.routing_strategy }
    }
}

pub struct SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef {
        SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routing_strategy` after provisioning.\n"]
    pub fn routing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
    max_concurrency: PrimField<f64>,
    memory_size_in_mb: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_concurrency: Option<PrimField<f64>>,
}

impl SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
    #[doc = "Set the field `provisioned_concurrency`.\n"]
    pub fn set_provisioned_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_concurrency = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
    #[doc = ""]
    pub max_concurrency: PrimField<f64>,
    #[doc = ""]
    pub memory_size_in_mb: PrimField<f64>,
}

impl BuildSagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
        SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl {
            max_concurrency: self.max_concurrency,
            memory_size_in_mb: self.memory_size_in_mb,
            provisioned_concurrency: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef {
        SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_in_mb` after provisioning.\n"]
    pub fn memory_size_in_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_concurrency` after provisioning.\n"]
    pub fn provisioned_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_concurrency", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationProductionVariantsElDynamic {
    core_dump_config: Option<DynamicBlock<SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl>>,
    managed_instance_scaling: Option<
        DynamicBlock<SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl>,
    >,
    routing_config: Option<DynamicBlock<SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl>>,
    serverless_config: Option<DynamicBlock<SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationProductionVariantsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_startup_health_check_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssm_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_ami_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_variant_weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_download_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size_in_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_dump_config: Option<Vec<SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_instance_scaling: Option<Vec<SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_config: Option<Vec<SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serverless_config: Option<Vec<SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl>>,
    dynamic: SagemakerEndpointConfigurationProductionVariantsElDynamic,
}

impl SagemakerEndpointConfigurationProductionVariantsEl {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }

    #[doc = "Set the field `container_startup_health_check_timeout_in_seconds`.\n"]
    pub fn set_container_startup_health_check_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_startup_health_check_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ssm_access`.\n"]
    pub fn set_enable_ssm_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ssm_access = Some(v.into());
        self
    }

    #[doc = "Set the field `inference_ami_version`.\n"]
    pub fn set_inference_ami_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_ami_version = Some(v.into());
        self
    }

    #[doc = "Set the field `initial_instance_count`.\n"]
    pub fn set_initial_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `initial_variant_weight`.\n"]
    pub fn set_initial_variant_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_variant_weight = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_download_timeout_in_seconds`.\n"]
    pub fn set_model_data_download_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.model_data_download_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_name = Some(v.into());
        self
    }

    #[doc = "Set the field `variant_name`.\n"]
    pub fn set_variant_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.variant_name = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size_in_gb = Some(v.into());
        self
    }

    #[doc = "Set the field `core_dump_config`.\n"]
    pub fn set_core_dump_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.core_dump_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.core_dump_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_instance_scaling`.\n"]
    pub fn set_managed_instance_scaling(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_instance_scaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_instance_scaling = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElRoutingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routing_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_config`.\n"]
    pub fn set_serverless_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationProductionVariantsElServerlessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.serverless_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationProductionVariantsEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationProductionVariantsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationProductionVariantsEl {}

impl BuildSagemakerEndpointConfigurationProductionVariantsEl {
    pub fn build(self) -> SagemakerEndpointConfigurationProductionVariantsEl {
        SagemakerEndpointConfigurationProductionVariantsEl {
            accelerator_type: core::default::Default::default(),
            container_startup_health_check_timeout_in_seconds: core::default::Default::default(),
            enable_ssm_access: core::default::Default::default(),
            inference_ami_version: core::default::Default::default(),
            initial_instance_count: core::default::Default::default(),
            initial_variant_weight: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_data_download_timeout_in_seconds: core::default::Default::default(),
            model_name: core::default::Default::default(),
            variant_name: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            core_dump_config: core::default::Default::default(),
            managed_instance_scaling: core::default::Default::default(),
            routing_config: core::default::Default::default(),
            serverless_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationProductionVariantsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationProductionVariantsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointConfigurationProductionVariantsElRef {
        SagemakerEndpointConfigurationProductionVariantsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationProductionVariantsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_startup_health_check_timeout_in_seconds` after provisioning.\n"]
    pub fn container_startup_health_check_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_startup_health_check_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ssm_access` after provisioning.\n"]
    pub fn enable_ssm_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssm_access", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_ami_version` after provisioning.\n"]
    pub fn inference_ami_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.inference_ami_version", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_instance_count` after provisioning.\n"]
    pub fn initial_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_variant_weight` after provisioning.\n"]
    pub fn initial_variant_weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_variant_weight", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_download_timeout_in_seconds` after provisioning.\n"]
    pub fn model_data_download_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_data_download_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `core_dump_config` after provisioning.\n"]
    pub fn core_dump_config(&self) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElCoreDumpConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_dump_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_instance_scaling` after provisioning.\n"]
    pub fn managed_instance_scaling(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElManagedInstanceScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_instance_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElRoutingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_config` after provisioning.\n"]
    pub fn serverless_config(&self) -> ListRef<SagemakerEndpointConfigurationProductionVariantsElServerlessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serverless_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
    destination_s3_uri: PrimField<String>,
    kms_key_id: PrimField<String>,
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl { }

impl ToListMappable for SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
    #[doc = ""]
    pub destination_s3_uri: PrimField<String>,
    #[doc = ""]
    pub kms_key_id: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
        SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl {
            destination_s3_uri: self.destination_s3_uri,
            kms_key_id: self.kms_key_id,
        }
    }
}

pub struct SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef {
        SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {}

impl BuildSagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
    pub fn build(self) -> SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
        SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef {
        SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
    routing_strategy: PrimField<String>,
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl { }

impl ToListMappable for SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
    #[doc = ""]
    pub routing_strategy: PrimField<String>,
}

impl BuildSagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
        SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl {
            routing_strategy: self.routing_strategy,
        }
    }
}

pub struct SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef {
        SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routing_strategy` after provisioning.\n"]
    pub fn routing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
    max_concurrency: PrimField<f64>,
    memory_size_in_mb: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_concurrency: Option<PrimField<f64>>,
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
    #[doc = "Set the field `provisioned_concurrency`.\n"]
    pub fn set_provisioned_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_concurrency = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
    #[doc = ""]
    pub max_concurrency: PrimField<f64>,
    #[doc = ""]
    pub memory_size_in_mb: PrimField<f64>,
}

impl BuildSagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
    pub fn build(self) -> SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
        SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl {
            max_concurrency: self.max_concurrency,
            memory_size_in_mb: self.memory_size_in_mb,
            provisioned_concurrency: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef {
        SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_in_mb` after provisioning.\n"]
    pub fn memory_size_in_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_concurrency` after provisioning.\n"]
    pub fn provisioned_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_concurrency", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationShadowProductionVariantsElDynamic {
    core_dump_config: Option<DynamicBlock<SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl>>,
    managed_instance_scaling: Option<
        DynamicBlock<SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl>,
    >,
    routing_config: Option<DynamicBlock<SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl>>,
    serverless_config: Option<
        DynamicBlock<SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfigurationShadowProductionVariantsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_startup_health_check_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssm_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_ami_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_variant_weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_download_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size_in_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_dump_config: Option<Vec<SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_instance_scaling: Option<
        Vec<SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_config: Option<Vec<SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serverless_config: Option<Vec<SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl>>,
    dynamic: SagemakerEndpointConfigurationShadowProductionVariantsElDynamic,
}

impl SagemakerEndpointConfigurationShadowProductionVariantsEl {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }

    #[doc = "Set the field `container_startup_health_check_timeout_in_seconds`.\n"]
    pub fn set_container_startup_health_check_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_startup_health_check_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ssm_access`.\n"]
    pub fn set_enable_ssm_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ssm_access = Some(v.into());
        self
    }

    #[doc = "Set the field `inference_ami_version`.\n"]
    pub fn set_inference_ami_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_ami_version = Some(v.into());
        self
    }

    #[doc = "Set the field `initial_instance_count`.\n"]
    pub fn set_initial_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `initial_variant_weight`.\n"]
    pub fn set_initial_variant_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_variant_weight = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_download_timeout_in_seconds`.\n"]
    pub fn set_model_data_download_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.model_data_download_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_name = Some(v.into());
        self
    }

    #[doc = "Set the field `variant_name`.\n"]
    pub fn set_variant_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.variant_name = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size_in_gb = Some(v.into());
        self
    }

    #[doc = "Set the field `core_dump_config`.\n"]
    pub fn set_core_dump_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.core_dump_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.core_dump_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_instance_scaling`.\n"]
    pub fn set_managed_instance_scaling(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_instance_scaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_instance_scaling = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routing_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_config`.\n"]
    pub fn set_serverless_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.serverless_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointConfigurationShadowProductionVariantsEl {
    type O = BlockAssignable<SagemakerEndpointConfigurationShadowProductionVariantsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfigurationShadowProductionVariantsEl {}

impl BuildSagemakerEndpointConfigurationShadowProductionVariantsEl {
    pub fn build(self) -> SagemakerEndpointConfigurationShadowProductionVariantsEl {
        SagemakerEndpointConfigurationShadowProductionVariantsEl {
            accelerator_type: core::default::Default::default(),
            container_startup_health_check_timeout_in_seconds: core::default::Default::default(),
            enable_ssm_access: core::default::Default::default(),
            inference_ami_version: core::default::Default::default(),
            initial_instance_count: core::default::Default::default(),
            initial_variant_weight: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_data_download_timeout_in_seconds: core::default::Default::default(),
            model_name: core::default::Default::default(),
            variant_name: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            core_dump_config: core::default::Default::default(),
            managed_instance_scaling: core::default::Default::default(),
            routing_config: core::default::Default::default(),
            serverless_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointConfigurationShadowProductionVariantsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointConfigurationShadowProductionVariantsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointConfigurationShadowProductionVariantsElRef {
        SagemakerEndpointConfigurationShadowProductionVariantsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfigurationShadowProductionVariantsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_startup_health_check_timeout_in_seconds` after provisioning.\n"]
    pub fn container_startup_health_check_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_startup_health_check_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ssm_access` after provisioning.\n"]
    pub fn enable_ssm_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssm_access", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_ami_version` after provisioning.\n"]
    pub fn inference_ami_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.inference_ami_version", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_instance_count` after provisioning.\n"]
    pub fn initial_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_variant_weight` after provisioning.\n"]
    pub fn initial_variant_weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_variant_weight", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_download_timeout_in_seconds` after provisioning.\n"]
    pub fn model_data_download_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_data_download_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `core_dump_config` after provisioning.\n"]
    pub fn core_dump_config(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElCoreDumpConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_dump_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_instance_scaling` after provisioning.\n"]
    pub fn managed_instance_scaling(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElManagedInstanceScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_instance_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElRoutingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_config` after provisioning.\n"]
    pub fn serverless_config(
        &self,
    ) -> ListRef<SagemakerEndpointConfigurationShadowProductionVariantsElServerlessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serverless_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationDynamic {
    async_inference_config: Option<DynamicBlock<SagemakerEndpointConfigurationAsyncInferenceConfigEl>>,
    data_capture_config: Option<DynamicBlock<SagemakerEndpointConfigurationDataCaptureConfigEl>>,
    production_variants: Option<DynamicBlock<SagemakerEndpointConfigurationProductionVariantsEl>>,
    shadow_production_variants: Option<DynamicBlock<SagemakerEndpointConfigurationShadowProductionVariantsEl>>,
}
