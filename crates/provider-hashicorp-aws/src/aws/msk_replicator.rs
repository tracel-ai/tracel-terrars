use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MskReplicatorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replicator_name: PrimField<String>,
    service_execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_cluster: Option<Vec<MskReplicatorKafkaClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_info_list: Option<Vec<MskReplicatorReplicationInfoListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MskReplicatorTimeoutsEl>,
    dynamic: MskReplicatorDynamic,
}
struct MskReplicator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MskReplicatorData>,
}
#[derive(Clone)]
pub struct MskReplicator(Rc<MskReplicator_>);
impl MskReplicator {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `kafka_cluster`.\n"]
    pub fn set_kafka_cluster(
        self,
        v: impl Into<BlockAssignable<MskReplicatorKafkaClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replication_info_list`.\n"]
    pub fn set_replication_info_list(
        self,
        v: impl Into<BlockAssignable<MskReplicatorReplicationInfoListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_info_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_info_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MskReplicatorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicator_name` after provisioning.\n"]
    pub fn replicator_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicator_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> ListRef<MskReplicatorKafkaClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_info_list` after provisioning.\n"]
    pub fn replication_info_list(&self) -> ListRef<MskReplicatorReplicationInfoListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_info_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskReplicatorTimeoutsElRef {
        MskReplicatorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MskReplicator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MskReplicator {}
impl ToListMappable for MskReplicator {
    type O = ListRef<MskReplicatorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MskReplicator_ {
    fn extract_resource_type(&self) -> String {
        "aws_msk_replicator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMskReplicator {
    pub tf_id: String,
    #[doc = ""]
    pub replicator_name: PrimField<String>,
    #[doc = ""]
    pub service_execution_role_arn: PrimField<String>,
}
impl BuildMskReplicator {
    pub fn build(self, stack: &mut Stack) -> MskReplicator {
        let out = MskReplicator(Rc::new(MskReplicator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MskReplicatorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replicator_name: self.replicator_name,
                service_execution_role_arn: self.service_execution_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                kafka_cluster: core::default::Default::default(),
                replication_info_list: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MskReplicatorRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MskReplicatorRef {
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
    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicator_name` after provisioning.\n"]
    pub fn replicator_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicator_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> ListRef<MskReplicatorKafkaClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_info_list` after provisioning.\n"]
    pub fn replication_info_list(&self) -> ListRef<MskReplicatorReplicationInfoListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_info_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskReplicatorTimeoutsElRef {
        MskReplicatorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MskReplicatorKafkaClusterElAmazonMskClusterEl {
    msk_cluster_arn: PrimField<String>,
}
impl MskReplicatorKafkaClusterElAmazonMskClusterEl {}
impl ToListMappable for MskReplicatorKafkaClusterElAmazonMskClusterEl {
    type O = BlockAssignable<MskReplicatorKafkaClusterElAmazonMskClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorKafkaClusterElAmazonMskClusterEl {
    #[doc = ""]
    pub msk_cluster_arn: PrimField<String>,
}
impl BuildMskReplicatorKafkaClusterElAmazonMskClusterEl {
    pub fn build(self) -> MskReplicatorKafkaClusterElAmazonMskClusterEl {
        MskReplicatorKafkaClusterElAmazonMskClusterEl {
            msk_cluster_arn: self.msk_cluster_arn,
        }
    }
}
pub struct MskReplicatorKafkaClusterElAmazonMskClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorKafkaClusterElAmazonMskClusterElRef {
    fn new(shared: StackShared, base: String) -> MskReplicatorKafkaClusterElAmazonMskClusterElRef {
        MskReplicatorKafkaClusterElAmazonMskClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorKafkaClusterElAmazonMskClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `msk_cluster_arn` after provisioning.\n"]
    pub fn msk_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.msk_cluster_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskReplicatorKafkaClusterElVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl MskReplicatorKafkaClusterElVpcConfigEl {
    #[doc = "Set the field `security_groups_ids`.\n"]
    pub fn set_security_groups_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups_ids = Some(v.into());
        self
    }
}
impl ToListMappable for MskReplicatorKafkaClusterElVpcConfigEl {
    type O = BlockAssignable<MskReplicatorKafkaClusterElVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorKafkaClusterElVpcConfigEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildMskReplicatorKafkaClusterElVpcConfigEl {
    pub fn build(self) -> MskReplicatorKafkaClusterElVpcConfigEl {
        MskReplicatorKafkaClusterElVpcConfigEl {
            security_groups_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct MskReplicatorKafkaClusterElVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorKafkaClusterElVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> MskReplicatorKafkaClusterElVpcConfigElRef {
        MskReplicatorKafkaClusterElVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorKafkaClusterElVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_groups_ids` after provisioning.\n"]
    pub fn security_groups_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskReplicatorKafkaClusterElDynamic {
    amazon_msk_cluster: Option<DynamicBlock<MskReplicatorKafkaClusterElAmazonMskClusterEl>>,
    vpc_config: Option<DynamicBlock<MskReplicatorKafkaClusterElVpcConfigEl>>,
}
#[derive(Serialize)]
pub struct MskReplicatorKafkaClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_msk_cluster: Option<Vec<MskReplicatorKafkaClusterElAmazonMskClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<MskReplicatorKafkaClusterElVpcConfigEl>>,
    dynamic: MskReplicatorKafkaClusterElDynamic,
}
impl MskReplicatorKafkaClusterEl {
    #[doc = "Set the field `amazon_msk_cluster`.\n"]
    pub fn set_amazon_msk_cluster(
        mut self,
        v: impl Into<BlockAssignable<MskReplicatorKafkaClusterElAmazonMskClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_msk_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_msk_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl Into<BlockAssignable<MskReplicatorKafkaClusterElVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskReplicatorKafkaClusterEl {
    type O = BlockAssignable<MskReplicatorKafkaClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorKafkaClusterEl {}
impl BuildMskReplicatorKafkaClusterEl {
    pub fn build(self) -> MskReplicatorKafkaClusterEl {
        MskReplicatorKafkaClusterEl {
            amazon_msk_cluster: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskReplicatorKafkaClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorKafkaClusterElRef {
    fn new(shared: StackShared, base: String) -> MskReplicatorKafkaClusterElRef {
        MskReplicatorKafkaClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorKafkaClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amazon_msk_cluster` after provisioning.\n"]
    pub fn amazon_msk_cluster(&self) -> ListRef<MskReplicatorKafkaClusterElAmazonMskClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_msk_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<MskReplicatorKafkaClusterElVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct MskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_groups_to_exclude: Option<SetField<PrimField<String>>>,
    consumer_groups_to_replicate: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detect_and_copy_new_consumer_groups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synchronise_consumer_group_offsets: Option<PrimField<bool>>,
}
impl MskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
    #[doc = "Set the field `consumer_groups_to_exclude`.\n"]
    pub fn set_consumer_groups_to_exclude(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.consumer_groups_to_exclude = Some(v.into());
        self
    }
    #[doc = "Set the field `detect_and_copy_new_consumer_groups`.\n"]
    pub fn set_detect_and_copy_new_consumer_groups(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.detect_and_copy_new_consumer_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `synchronise_consumer_group_offsets`.\n"]
    pub fn set_synchronise_consumer_group_offsets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.synchronise_consumer_group_offsets = Some(v.into());
        self
    }
}
impl ToListMappable for MskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
    type O = BlockAssignable<MskReplicatorReplicationInfoListElConsumerGroupReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
    #[doc = ""]
    pub consumer_groups_to_replicate: SetField<PrimField<String>>,
}
impl BuildMskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
    pub fn build(self) -> MskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
        MskReplicatorReplicationInfoListElConsumerGroupReplicationEl {
            consumer_groups_to_exclude: core::default::Default::default(),
            consumer_groups_to_replicate: self.consumer_groups_to_replicate,
            detect_and_copy_new_consumer_groups: core::default::Default::default(),
            synchronise_consumer_group_offsets: core::default::Default::default(),
        }
    }
}
pub struct MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef {
        MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_groups_to_exclude` after provisioning.\n"]
    pub fn consumer_groups_to_exclude(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.consumer_groups_to_exclude", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_groups_to_replicate` after provisioning.\n"]
    pub fn consumer_groups_to_replicate(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.consumer_groups_to_replicate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `detect_and_copy_new_consumer_groups` after provisioning.\n"]
    pub fn detect_and_copy_new_consumer_groups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_and_copy_new_consumer_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `synchronise_consumer_group_offsets` after provisioning.\n"]
    pub fn synchronise_consumer_group_offsets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.synchronise_consumer_group_offsets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
    type O =
        BlockAssignable<MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {}
impl BuildMskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
    pub fn build(self) -> MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
        MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef {
        MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl
{
    type O = BlockAssignable<
        MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {}
impl BuildMskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {
    pub fn build(
        self,
    ) -> MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {
        MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef {
        MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskReplicatorReplicationInfoListElTopicReplicationElDynamic {
    starting_position: Option<
        DynamicBlock<MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl>,
    >,
    topic_name_configuration: Option<
        DynamicBlock<MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl>,
    >,
}
#[derive(Serialize)]
pub struct MskReplicatorReplicationInfoListElTopicReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_access_control_lists_for_topics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_topic_configurations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detect_and_copy_new_topics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics_to_exclude: Option<SetField<PrimField<String>>>,
    topics_to_replicate: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position:
        Option<Vec<MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_name_configuration:
        Option<Vec<MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl>>,
    dynamic: MskReplicatorReplicationInfoListElTopicReplicationElDynamic,
}
impl MskReplicatorReplicationInfoListElTopicReplicationEl {
    #[doc = "Set the field `copy_access_control_lists_for_topics`.\n"]
    pub fn set_copy_access_control_lists_for_topics(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.copy_access_control_lists_for_topics = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_topic_configurations`.\n"]
    pub fn set_copy_topic_configurations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.copy_topic_configurations = Some(v.into());
        self
    }
    #[doc = "Set the field `detect_and_copy_new_topics`.\n"]
    pub fn set_detect_and_copy_new_topics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.detect_and_copy_new_topics = Some(v.into());
        self
    }
    #[doc = "Set the field `topics_to_exclude`.\n"]
    pub fn set_topics_to_exclude(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.topics_to_exclude = Some(v.into());
        self
    }
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(
        mut self,
        v: impl Into<
            BlockAssignable<MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.starting_position = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.starting_position = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `topic_name_configuration`.\n"]
    pub fn set_topic_name_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.topic_name_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.topic_name_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskReplicatorReplicationInfoListElTopicReplicationEl {
    type O = BlockAssignable<MskReplicatorReplicationInfoListElTopicReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorReplicationInfoListElTopicReplicationEl {
    #[doc = ""]
    pub topics_to_replicate: SetField<PrimField<String>>,
}
impl BuildMskReplicatorReplicationInfoListElTopicReplicationEl {
    pub fn build(self) -> MskReplicatorReplicationInfoListElTopicReplicationEl {
        MskReplicatorReplicationInfoListElTopicReplicationEl {
            copy_access_control_lists_for_topics: core::default::Default::default(),
            copy_topic_configurations: core::default::Default::default(),
            detect_and_copy_new_topics: core::default::Default::default(),
            topics_to_exclude: core::default::Default::default(),
            topics_to_replicate: self.topics_to_replicate,
            starting_position: core::default::Default::default(),
            topic_name_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskReplicatorReplicationInfoListElTopicReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorReplicationInfoListElTopicReplicationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskReplicatorReplicationInfoListElTopicReplicationElRef {
        MskReplicatorReplicationInfoListElTopicReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorReplicationInfoListElTopicReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `copy_access_control_lists_for_topics` after provisioning.\n"]
    pub fn copy_access_control_lists_for_topics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_access_control_lists_for_topics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `copy_topic_configurations` after provisioning.\n"]
    pub fn copy_topic_configurations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_topic_configurations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `detect_and_copy_new_topics` after provisioning.\n"]
    pub fn detect_and_copy_new_topics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_and_copy_new_topics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topics_to_exclude` after provisioning.\n"]
    pub fn topics_to_exclude(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.topics_to_exclude", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topics_to_replicate` after provisioning.\n"]
    pub fn topics_to_replicate(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.topics_to_replicate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(
        &self,
    ) -> ListRef<MskReplicatorReplicationInfoListElTopicReplicationElStartingPositionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic_name_configuration` after provisioning.\n"]
    pub fn topic_name_configuration(
        &self,
    ) -> ListRef<MskReplicatorReplicationInfoListElTopicReplicationElTopicNameConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topic_name_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MskReplicatorReplicationInfoListElDynamic {
    consumer_group_replication:
        Option<DynamicBlock<MskReplicatorReplicationInfoListElConsumerGroupReplicationEl>>,
    topic_replication: Option<DynamicBlock<MskReplicatorReplicationInfoListElTopicReplicationEl>>,
}
#[derive(Serialize)]
pub struct MskReplicatorReplicationInfoListEl {
    source_kafka_cluster_arn: PrimField<String>,
    target_compression_type: PrimField<String>,
    target_kafka_cluster_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group_replication:
        Option<Vec<MskReplicatorReplicationInfoListElConsumerGroupReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_replication: Option<Vec<MskReplicatorReplicationInfoListElTopicReplicationEl>>,
    dynamic: MskReplicatorReplicationInfoListElDynamic,
}
impl MskReplicatorReplicationInfoListEl {
    #[doc = "Set the field `consumer_group_replication`.\n"]
    pub fn set_consumer_group_replication(
        mut self,
        v: impl Into<BlockAssignable<MskReplicatorReplicationInfoListElConsumerGroupReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.consumer_group_replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.consumer_group_replication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `topic_replication`.\n"]
    pub fn set_topic_replication(
        mut self,
        v: impl Into<BlockAssignable<MskReplicatorReplicationInfoListElTopicReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.topic_replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.topic_replication = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskReplicatorReplicationInfoListEl {
    type O = BlockAssignable<MskReplicatorReplicationInfoListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorReplicationInfoListEl {
    #[doc = ""]
    pub source_kafka_cluster_arn: PrimField<String>,
    #[doc = ""]
    pub target_compression_type: PrimField<String>,
    #[doc = ""]
    pub target_kafka_cluster_arn: PrimField<String>,
}
impl BuildMskReplicatorReplicationInfoListEl {
    pub fn build(self) -> MskReplicatorReplicationInfoListEl {
        MskReplicatorReplicationInfoListEl {
            source_kafka_cluster_arn: self.source_kafka_cluster_arn,
            target_compression_type: self.target_compression_type,
            target_kafka_cluster_arn: self.target_kafka_cluster_arn,
            consumer_group_replication: core::default::Default::default(),
            topic_replication: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskReplicatorReplicationInfoListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorReplicationInfoListElRef {
    fn new(shared: StackShared, base: String) -> MskReplicatorReplicationInfoListElRef {
        MskReplicatorReplicationInfoListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorReplicationInfoListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_kafka_cluster_alias` after provisioning.\n"]
    pub fn source_kafka_cluster_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_kafka_cluster_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_kafka_cluster_arn` after provisioning.\n"]
    pub fn source_kafka_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_kafka_cluster_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_compression_type` after provisioning.\n"]
    pub fn target_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_kafka_cluster_alias` after provisioning.\n"]
    pub fn target_kafka_cluster_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_kafka_cluster_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_kafka_cluster_arn` after provisioning.\n"]
    pub fn target_kafka_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_kafka_cluster_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_group_replication` after provisioning.\n"]
    pub fn consumer_group_replication(
        &self,
    ) -> ListRef<MskReplicatorReplicationInfoListElConsumerGroupReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_group_replication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic_replication` after provisioning.\n"]
    pub fn topic_replication(
        &self,
    ) -> ListRef<MskReplicatorReplicationInfoListElTopicReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topic_replication", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskReplicatorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MskReplicatorTimeoutsEl {
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
impl ToListMappable for MskReplicatorTimeoutsEl {
    type O = BlockAssignable<MskReplicatorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskReplicatorTimeoutsEl {}
impl BuildMskReplicatorTimeoutsEl {
    pub fn build(self) -> MskReplicatorTimeoutsEl {
        MskReplicatorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MskReplicatorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskReplicatorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MskReplicatorTimeoutsElRef {
        MskReplicatorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskReplicatorTimeoutsElRef {
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
struct MskReplicatorDynamic {
    kafka_cluster: Option<DynamicBlock<MskReplicatorKafkaClusterEl>>,
    replication_info_list: Option<DynamicBlock<MskReplicatorReplicationInfoListEl>>,
}
