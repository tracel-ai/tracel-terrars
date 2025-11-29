use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockModelInvocationLoggingConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<BedrockModelInvocationLoggingConfigurationLoggingConfigEl>>,
    dynamic: BedrockModelInvocationLoggingConfigurationDynamic,
}

struct BedrockModelInvocationLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockModelInvocationLoggingConfigurationData>,
}

#[derive(Clone)]
pub struct BedrockModelInvocationLoggingConfiguration(Rc<BedrockModelInvocationLoggingConfiguration_>);

impl BedrockModelInvocationLoggingConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<BedrockModelInvocationLoggingConfigurationLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }
}

impl Referable for BedrockModelInvocationLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockModelInvocationLoggingConfiguration { }

impl ToListMappable for BedrockModelInvocationLoggingConfiguration {
    type O = ListRef<BedrockModelInvocationLoggingConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockModelInvocationLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrock_model_invocation_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration {
    pub tf_id: String,
}

impl BuildBedrockModelInvocationLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> BedrockModelInvocationLoggingConfiguration {
        let out = BedrockModelInvocationLoggingConfiguration(Rc::new(BedrockModelInvocationLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockModelInvocationLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockModelInvocationLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockModelInvocationLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<BedrockModelInvocationLoggingConfigurationLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
    type O =
        BlockAssignable<
            BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {}

impl BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
    pub fn build(
        self,
    ) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl {
            bucket_name: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
        }
    }
}

pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElDynamic {
    large_data_delivery_s3_config: Option<
        DynamicBlock<
            BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_data_delivery_s3_config: Option<
        Vec<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl>,
    >,
    dynamic: BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElDynamic,
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `large_data_delivery_s3_config`.\n"]
    pub fn set_large_data_delivery_s3_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.large_data_delivery_s3_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.large_data_delivery_s3_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
    type O = BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {}

impl BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
    pub fn build(self) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl {
            log_group_name: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            large_data_delivery_s3_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `large_data_delivery_s3_config` after provisioning.\n"]
    pub fn large_data_delivery_s3_config(
        &self,
    ) -> ListRef<
        BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElLargeDataDeliveryS3ConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.large_data_delivery_s3_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
    type O = BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {}

impl BuildBedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
    pub fn build(self) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl {
            bucket_name: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
        }
    }
}

pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfigurationLoggingConfigElDynamic {
    cloudwatch_config: Option<
        DynamicBlock<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl>,
    >,
    s3_config: Option<DynamicBlock<BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    embedding_data_delivery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_data_delivery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_data_delivery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_data_delivery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_config: Option<Vec<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_config: Option<Vec<BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl>>,
    dynamic: BedrockModelInvocationLoggingConfigurationLoggingConfigElDynamic,
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigEl {
    #[doc = "Set the field `embedding_data_delivery_enabled`.\n"]
    pub fn set_embedding_data_delivery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.embedding_data_delivery_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `image_data_delivery_enabled`.\n"]
    pub fn set_image_data_delivery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_data_delivery_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `text_data_delivery_enabled`.\n"]
    pub fn set_text_data_delivery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.text_data_delivery_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `video_data_delivery_enabled`.\n"]
    pub fn set_video_data_delivery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.video_data_delivery_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_config`.\n"]
    pub fn set_cloudwatch_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockModelInvocationLoggingConfigurationLoggingConfigEl {
    type O = BlockAssignable<BedrockModelInvocationLoggingConfigurationLoggingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfigurationLoggingConfigEl {}

impl BuildBedrockModelInvocationLoggingConfigurationLoggingConfigEl {
    pub fn build(self) -> BedrockModelInvocationLoggingConfigurationLoggingConfigEl {
        BedrockModelInvocationLoggingConfigurationLoggingConfigEl {
            embedding_data_delivery_enabled: core::default::Default::default(),
            image_data_delivery_enabled: core::default::Default::default(),
            text_data_delivery_enabled: core::default::Default::default(),
            video_data_delivery_enabled: core::default::Default::default(),
            cloudwatch_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockModelInvocationLoggingConfigurationLoggingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockModelInvocationLoggingConfigurationLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockModelInvocationLoggingConfigurationLoggingConfigElRef {
        BedrockModelInvocationLoggingConfigurationLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `embedding_data_delivery_enabled` after provisioning.\n"]
    pub fn embedding_data_delivery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.embedding_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `image_data_delivery_enabled` after provisioning.\n"]
    pub fn image_data_delivery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text_data_delivery_enabled` after provisioning.\n"]
    pub fn text_data_delivery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `video_data_delivery_enabled` after provisioning.\n"]
    pub fn video_data_delivery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.video_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_config` after provisioning.\n"]
    pub fn cloudwatch_config(
        &self,
    ) -> ListRef<BedrockModelInvocationLoggingConfigurationLoggingConfigElCloudwatchConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> ListRef<BedrockModelInvocationLoggingConfigurationLoggingConfigElS3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfigurationDynamic {
    logging_config: Option<DynamicBlock<BedrockModelInvocationLoggingConfigurationLoggingConfigEl>>,
}
