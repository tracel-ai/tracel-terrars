use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigDeliveryChannelData {
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
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_delivery_properties: Option<Vec<ConfigDeliveryChannelSnapshotDeliveryPropertiesEl>>,
    dynamic: ConfigDeliveryChannelDynamic,
}
struct ConfigDeliveryChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigDeliveryChannelData>,
}
#[derive(Clone)]
pub struct ConfigDeliveryChannel(Rc<ConfigDeliveryChannel_>);
impl ConfigDeliveryChannel {
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_key_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_kms_key_arn`.\n"]
    pub fn set_s3_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sns_topic_arn`.\n"]
    pub fn set_sns_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sns_topic_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_delivery_properties`.\n"]
    pub fn set_snapshot_delivery_properties(
        self,
        v: impl Into<BlockAssignable<ConfigDeliveryChannelSnapshotDeliveryPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_delivery_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .snapshot_delivery_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_kms_key_arn` after provisioning.\n"]
    pub fn s3_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_delivery_properties` after provisioning.\n"]
    pub fn snapshot_delivery_properties(
        &self,
    ) -> ListRef<ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_delivery_properties", self.extract_ref()),
        )
    }
}
impl Referable for ConfigDeliveryChannel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigDeliveryChannel {}
impl ToListMappable for ConfigDeliveryChannel {
    type O = ListRef<ConfigDeliveryChannelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigDeliveryChannel_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_delivery_channel".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigDeliveryChannel {
    pub tf_id: String,
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}
impl BuildConfigDeliveryChannel {
    pub fn build(self, stack: &mut Stack) -> ConfigDeliveryChannel {
        let out = ConfigDeliveryChannel(Rc::new(ConfigDeliveryChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigDeliveryChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                s3_bucket_name: self.s3_bucket_name,
                s3_key_prefix: core::default::Default::default(),
                s3_kms_key_arn: core::default::Default::default(),
                sns_topic_arn: core::default::Default::default(),
                snapshot_delivery_properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigDeliveryChannelRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigDeliveryChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigDeliveryChannelRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_kms_key_arn` after provisioning.\n"]
    pub fn s3_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_delivery_properties` after provisioning.\n"]
    pub fn snapshot_delivery_properties(
        &self,
    ) -> ListRef<ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_delivery_properties", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_frequency: Option<PrimField<String>>,
}
impl ConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
    #[doc = "Set the field `delivery_frequency`.\n"]
    pub fn set_delivery_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delivery_frequency = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
    type O = BlockAssignable<ConfigDeliveryChannelSnapshotDeliveryPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigDeliveryChannelSnapshotDeliveryPropertiesEl {}
impl BuildConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
    pub fn build(self) -> ConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
        ConfigDeliveryChannelSnapshotDeliveryPropertiesEl {
            delivery_frequency: core::default::Default::default(),
        }
    }
}
pub struct ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef {
        ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigDeliveryChannelSnapshotDeliveryPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delivery_frequency` after provisioning.\n"]
    pub fn delivery_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_frequency", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigDeliveryChannelDynamic {
    snapshot_delivery_properties:
        Option<DynamicBlock<ConfigDeliveryChannelSnapshotDeliveryPropertiesEl>>,
}
