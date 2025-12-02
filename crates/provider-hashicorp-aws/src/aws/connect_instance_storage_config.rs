use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConnectInstanceStorageConfigData {
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
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_config: Option<Vec<ConnectInstanceStorageConfigStorageConfigEl>>,
    dynamic: ConnectInstanceStorageConfigDynamic,
}
struct ConnectInstanceStorageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectInstanceStorageConfigData>,
}
#[derive(Clone)]
pub struct ConnectInstanceStorageConfig(Rc<ConnectInstanceStorageConfig_>);
impl ConnectInstanceStorageConfig {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl Into<BlockAssignable<ConnectInstanceStorageConfigStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<ConnectInstanceStorageConfigStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
}
impl Referable for ConnectInstanceStorageConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConnectInstanceStorageConfig {}
impl ToListMappable for ConnectInstanceStorageConfig {
    type O = ListRef<ConnectInstanceStorageConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConnectInstanceStorageConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_instance_storage_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConnectInstanceStorageConfig {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}
impl BuildConnectInstanceStorageConfig {
    pub fn build(self, stack: &mut Stack) -> ConnectInstanceStorageConfig {
        let out = ConnectInstanceStorageConfig(Rc::new(ConnectInstanceStorageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectInstanceStorageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                storage_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConnectInstanceStorageConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConnectInstanceStorageConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<ConnectInstanceStorageConfigStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    firehose_arn: PrimField<String>,
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    type O = BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[doc = ""]
    pub firehose_arn: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
            firehose_arn: self.firehose_arn,
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    stream_arn: PrimField<String>,
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    type O = BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[doc = ""]
    pub stream_arn: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
            stream_arn: self.stream_arn,
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    encryption_type: PrimField<String>,
    key_id: PrimField<String>,
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {}
impl ToListMappable
    for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl
{
    type O = BlockAssignable<
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl
{
    #[doc = ""]
    pub encryption_type: PrimField<String>,
    #[doc = ""]
    pub key_id: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    pub fn build(
        self,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl
    {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
            encryption_type: self.encryption_type,
            key_id: self.key_id,
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef
    {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElDynamic {
    encryption_config: Option<
        DynamicBlock<
            ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    prefix: PrimField<String>,
    retention_period_hours: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<
        Vec<
            ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
        >,
    >,
    dynamic: ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElDynamic,
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v : impl Into < BlockAssignable < ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    type O = BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = ""]
    pub prefix: PrimField<String>,
    #[doc = ""]
    pub retention_period_hours: PrimField<f64>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
            prefix: self.prefix,
            retention_period_hours: self.retention_period_hours,
            encryption_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `retention_period_hours` after provisioning.\n"]
    pub fn retention_period_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_hours", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    encryption_type: PrimField<String>,
    key_id: PrimField<String>,
}
impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    type O =
        BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[doc = ""]
    pub encryption_type: PrimField<String>,
    #[doc = ""]
    pub key_id: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
            encryption_type: self.encryption_type,
            key_id: self.key_id,
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElDynamic {
    encryption_config: Option<
        DynamicBlock<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    bucket_name: PrimField<String>,
    bucket_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config:
        Option<Vec<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
    dynamic: ConnectInstanceStorageConfigStorageConfigElS3ConfigElDynamic,
}
impl ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    type O = BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub bucket_prefix: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            encryption_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElDynamic {
    kinesis_firehose_config:
        Option<DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    kinesis_stream_config:
        Option<DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    kinesis_video_stream_config:
        Option<DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    s3_config: Option<DynamicBlock<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
}
#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigEl {
    storage_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose_config:
        Option<Vec<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_config:
        Option<Vec<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_video_stream_config:
        Option<Vec<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_config: Option<Vec<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    dynamic: ConnectInstanceStorageConfigStorageConfigElDynamic,
}
impl ConnectInstanceStorageConfigStorageConfigEl {
    #[doc = "Set the field `kinesis_firehose_config`.\n"]
    pub fn set_kinesis_firehose_config(
        mut self,
        v: impl Into<
            BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v: impl Into<BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_video_stream_config`.\n"]
    pub fn set_kinesis_video_stream_config(
        mut self,
        v: impl Into<
            BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_video_stream_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_video_stream_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl Into<BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConnectInstanceStorageConfigStorageConfigEl {
    type O = BlockAssignable<ConnectInstanceStorageConfigStorageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectInstanceStorageConfigStorageConfigEl {
    #[doc = ""]
    pub storage_type: PrimField<String>,
}
impl BuildConnectInstanceStorageConfigStorageConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigEl {
        ConnectInstanceStorageConfigStorageConfigEl {
            storage_type: self.storage_type,
            kinesis_firehose_config: core::default::Default::default(),
            kinesis_stream_config: core::default::Default::default(),
            kinesis_video_stream_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConnectInstanceStorageConfigStorageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectInstanceStorageConfigStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectInstanceStorageConfigStorageConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectInstanceStorageConfigStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kinesis_firehose_config` after provisioning.\n"]
    pub fn kinesis_firehose_config(
        &self,
    ) -> ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_firehose_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_video_stream_config` after provisioning.\n"]
    pub fn kinesis_video_stream_config(
        &self,
    ) -> ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_video_stream_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> ListRef<ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigDynamic {
    storage_config: Option<DynamicBlock<ConnectInstanceStorageConfigStorageConfigEl>>,
}
