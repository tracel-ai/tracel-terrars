use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PubsubLiteTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_config: Option<Vec<PubsubLiteTopicPartitionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_config: Option<Vec<PubsubLiteTopicReservationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_config: Option<Vec<PubsubLiteTopicRetentionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PubsubLiteTopicTimeoutsEl>,
    dynamic: PubsubLiteTopicDynamic,
}
struct PubsubLiteTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PubsubLiteTopicData>,
}
#[derive(Clone)]
pub struct PubsubLiteTopic(Rc<PubsubLiteTopic_>);
impl PubsubLiteTopic {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the pubsub lite topic."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe zone of the pubsub lite topic."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `partition_config`.\n"]
    pub fn set_partition_config(
        self,
        v: impl Into<BlockAssignable<PubsubLiteTopicPartitionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_config`.\n"]
    pub fn set_reservation_config(
        self,
        v: impl Into<BlockAssignable<PubsubLiteTopicReservationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retention_config`.\n"]
    pub fn set_retention_config(
        self,
        v: impl Into<BlockAssignable<PubsubLiteTopicRetentionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PubsubLiteTopicTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_config` after provisioning.\n"]
    pub fn partition_config(&self) -> ListRef<PubsubLiteTopicPartitionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_config` after provisioning.\n"]
    pub fn reservation_config(&self) -> ListRef<PubsubLiteTopicReservationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_config` after provisioning.\n"]
    pub fn retention_config(&self) -> ListRef<PubsubLiteTopicRetentionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteTopicTimeoutsElRef {
        PubsubLiteTopicTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PubsubLiteTopic {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PubsubLiteTopic {}
impl ToListMappable for PubsubLiteTopic {
    type O = ListRef<PubsubLiteTopicRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PubsubLiteTopic_ {
    fn extract_resource_type(&self) -> String {
        "google_pubsub_lite_topic".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPubsubLiteTopic {
    pub tf_id: String,
    #[doc = "Name of the topic."]
    pub name: PrimField<String>,
}
impl BuildPubsubLiteTopic {
    pub fn build(self, stack: &mut Stack) -> PubsubLiteTopic {
        let out = PubsubLiteTopic(Rc::new(PubsubLiteTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PubsubLiteTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                zone: core::default::Default::default(),
                partition_config: core::default::Default::default(),
                reservation_config: core::default::Default::default(),
                retention_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PubsubLiteTopicRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PubsubLiteTopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_config` after provisioning.\n"]
    pub fn partition_config(&self) -> ListRef<PubsubLiteTopicPartitionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_config` after provisioning.\n"]
    pub fn reservation_config(&self) -> ListRef<PubsubLiteTopicReservationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_config` after provisioning.\n"]
    pub fn retention_config(&self) -> ListRef<PubsubLiteTopicRetentionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteTopicTimeoutsElRef {
        PubsubLiteTopicTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubLiteTopicPartitionConfigElCapacityEl {
    publish_mib_per_sec: PrimField<f64>,
    subscribe_mib_per_sec: PrimField<f64>,
}
impl PubsubLiteTopicPartitionConfigElCapacityEl {}
impl ToListMappable for PubsubLiteTopicPartitionConfigElCapacityEl {
    type O = BlockAssignable<PubsubLiteTopicPartitionConfigElCapacityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubLiteTopicPartitionConfigElCapacityEl {
    #[doc = "Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub publish_mib_per_sec: PrimField<f64>,
    #[doc = "Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub subscribe_mib_per_sec: PrimField<f64>,
}
impl BuildPubsubLiteTopicPartitionConfigElCapacityEl {
    pub fn build(self) -> PubsubLiteTopicPartitionConfigElCapacityEl {
        PubsubLiteTopicPartitionConfigElCapacityEl {
            publish_mib_per_sec: self.publish_mib_per_sec,
            subscribe_mib_per_sec: self.subscribe_mib_per_sec,
        }
    }
}
pub struct PubsubLiteTopicPartitionConfigElCapacityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicPartitionConfigElCapacityElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteTopicPartitionConfigElCapacityElRef {
        PubsubLiteTopicPartitionConfigElCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubLiteTopicPartitionConfigElCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `publish_mib_per_sec` after provisioning.\nSubscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub fn publish_mib_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publish_mib_per_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscribe_mib_per_sec` after provisioning.\nPublish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub fn subscribe_mib_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscribe_mib_per_sec", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PubsubLiteTopicPartitionConfigElDynamic {
    capacity: Option<DynamicBlock<PubsubLiteTopicPartitionConfigElCapacityEl>>,
}
#[derive(Serialize)]
pub struct PubsubLiteTopicPartitionConfigEl {
    count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<Vec<PubsubLiteTopicPartitionConfigElCapacityEl>>,
    dynamic: PubsubLiteTopicPartitionConfigElDynamic,
}
impl PubsubLiteTopicPartitionConfigEl {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(
        mut self,
        v: impl Into<BlockAssignable<PubsubLiteTopicPartitionConfigElCapacityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PubsubLiteTopicPartitionConfigEl {
    type O = BlockAssignable<PubsubLiteTopicPartitionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubLiteTopicPartitionConfigEl {
    #[doc = "The number of partitions in the topic. Must be at least 1."]
    pub count: PrimField<f64>,
}
impl BuildPubsubLiteTopicPartitionConfigEl {
    pub fn build(self) -> PubsubLiteTopicPartitionConfigEl {
        PubsubLiteTopicPartitionConfigEl {
            count: self.count,
            capacity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PubsubLiteTopicPartitionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicPartitionConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteTopicPartitionConfigElRef {
        PubsubLiteTopicPartitionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubLiteTopicPartitionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of partitions in the topic. Must be at least 1."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> ListRef<PubsubLiteTopicPartitionConfigElCapacityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubLiteTopicReservationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_reservation: Option<PrimField<String>>,
}
impl PubsubLiteTopicReservationConfigEl {
    #[doc = "Set the field `throughput_reservation`.\nThe Reservation to use for this topic's throughput capacity."]
    pub fn set_throughput_reservation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.throughput_reservation = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubLiteTopicReservationConfigEl {
    type O = BlockAssignable<PubsubLiteTopicReservationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubLiteTopicReservationConfigEl {}
impl BuildPubsubLiteTopicReservationConfigEl {
    pub fn build(self) -> PubsubLiteTopicReservationConfigEl {
        PubsubLiteTopicReservationConfigEl {
            throughput_reservation: core::default::Default::default(),
        }
    }
}
pub struct PubsubLiteTopicReservationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicReservationConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteTopicReservationConfigElRef {
        PubsubLiteTopicReservationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubLiteTopicReservationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `throughput_reservation` after provisioning.\nThe Reservation to use for this topic's throughput capacity."]
    pub fn throughput_reservation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_reservation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubLiteTopicRetentionConfigEl {
    per_partition_bytes: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<String>>,
}
impl PubsubLiteTopicRetentionConfigEl {
    #[doc = "Set the field `period`.\nHow long a published message is retained. If unset, messages will be retained as\nlong as the bytes retained for each partition is below perPartitionBytes. A\nduration in seconds with up to nine fractional digits, terminated by 's'.\nExample: \"3.5s\"."]
    pub fn set_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.period = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubLiteTopicRetentionConfigEl {
    type O = BlockAssignable<PubsubLiteTopicRetentionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubLiteTopicRetentionConfigEl {
    #[doc = "The provisioned storage, in bytes, per partition. If the number of bytes stored\nin any of the topic's partitions grows beyond this value, older messages will be\ndropped to make room for newer ones, regardless of the value of period."]
    pub per_partition_bytes: PrimField<String>,
}
impl BuildPubsubLiteTopicRetentionConfigEl {
    pub fn build(self) -> PubsubLiteTopicRetentionConfigEl {
        PubsubLiteTopicRetentionConfigEl {
            per_partition_bytes: self.per_partition_bytes,
            period: core::default::Default::default(),
        }
    }
}
pub struct PubsubLiteTopicRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicRetentionConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteTopicRetentionConfigElRef {
        PubsubLiteTopicRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubLiteTopicRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `per_partition_bytes` after provisioning.\nThe provisioned storage, in bytes, per partition. If the number of bytes stored\nin any of the topic's partitions grows beyond this value, older messages will be\ndropped to make room for newer ones, regardless of the value of period."]
    pub fn per_partition_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_partition_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\nHow long a published message is retained. If unset, messages will be retained as\nlong as the bytes retained for each partition is below perPartitionBytes. A\nduration in seconds with up to nine fractional digits, terminated by 's'.\nExample: \"3.5s\"."]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubLiteTopicTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PubsubLiteTopicTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubLiteTopicTimeoutsEl {
    type O = BlockAssignable<PubsubLiteTopicTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubLiteTopicTimeoutsEl {}
impl BuildPubsubLiteTopicTimeoutsEl {
    pub fn build(self) -> PubsubLiteTopicTimeoutsEl {
        PubsubLiteTopicTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PubsubLiteTopicTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubLiteTopicTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteTopicTimeoutsElRef {
        PubsubLiteTopicTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubLiteTopicTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct PubsubLiteTopicDynamic {
    partition_config: Option<DynamicBlock<PubsubLiteTopicPartitionConfigEl>>,
    reservation_config: Option<DynamicBlock<PubsubLiteTopicReservationConfigEl>>,
    retention_config: Option<DynamicBlock<PubsubLiteTopicRetentionConfigEl>>,
}
