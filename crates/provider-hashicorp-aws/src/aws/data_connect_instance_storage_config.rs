use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectInstanceStorageConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    association_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
}

struct DataConnectInstanceStorageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectInstanceStorageConfigData>,
}

#[derive(Clone)]
pub struct DataConnectInstanceStorageConfig(Rc<DataConnectInstanceStorageConfig_>);

impl DataConnectInstanceStorageConfig {
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

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

impl Referable for DataConnectInstanceStorageConfig {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataConnectInstanceStorageConfig { }

impl ToListMappable for DataConnectInstanceStorageConfig {
    type O = ListRef<DataConnectInstanceStorageConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataConnectInstanceStorageConfig_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_instance_storage_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectInstanceStorageConfig {
    pub tf_id: String,
    #[doc = ""]
    pub association_id: PrimField<String>,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}

impl BuildDataConnectInstanceStorageConfig {
    pub fn build(self, stack: &mut Stack) -> DataConnectInstanceStorageConfig {
        let out = DataConnectInstanceStorageConfig(Rc::new(DataConnectInstanceStorageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectInstanceStorageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                association_id: self.association_id,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
                resource_type: self.resource_type,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectInstanceStorageConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataConnectInstanceStorageConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_arn: Option<PrimField<String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[doc = "Set the field `firehose_arn`.\n"]
    pub fn set_firehose_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.firehose_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
            firehose_arn: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
            stream_arn: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }

    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    type O =
        BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    pub fn build(
        self,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
            encryption_type: core::default::Default::default(),
            key_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<
        ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period_hours: Option<PrimField<f64>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
                        >,
                    >,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_hours`.\n"]
    pub fn set_retention_period_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_period_hours = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
            encryption_config: core::default::Default::default(),
            prefix: core::default::Default::default(),
            retention_period_hours: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period_hours` after provisioning.\n"]
    pub fn retention_period_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period_hours", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }

    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
            encryption_type: core::default::Default::default(),
            key_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn new(shared: StackShared, base: String) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose_config: Option<ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_config: Option<ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_video_stream_config: Option<
        ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_config: Option<ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigEl {
    #[doc = "Set the field `kinesis_firehose_config`.\n"]
    pub fn set_kinesis_firehose_config(
        mut self,
        v: impl Into<ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    ) -> Self {
        self.kinesis_firehose_config = Some(v.into());
        self
    }

    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v: impl Into<ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    ) -> Self {
        self.kinesis_stream_config = Some(v.into());
        self
    }

    #[doc = "Set the field `kinesis_video_stream_config`.\n"]
    pub fn set_kinesis_video_stream_config(
        mut self,
        v: impl Into<ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    ) -> Self {
        self.kinesis_video_stream_config = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl Into<ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    ) -> Self {
        self.s3_config = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectInstanceStorageConfigStorageConfigEl {
    type O = BlockAssignable<DataConnectInstanceStorageConfigStorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigEl {
        DataConnectInstanceStorageConfigStorageConfigEl {
            kinesis_firehose_config: core::default::Default::default(),
            kinesis_stream_config: core::default::Default::default(),
            kinesis_video_stream_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            storage_type: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectInstanceStorageConfigStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> DataConnectInstanceStorageConfigStorageConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_config` after provisioning.\n"]
    pub fn kinesis_firehose_config(
        &self,
    ) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_video_stream_config` after provisioning.\n"]
    pub fn kinesis_video_stream_config(
        &self,
    ) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_video_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> ListRef<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }
}
