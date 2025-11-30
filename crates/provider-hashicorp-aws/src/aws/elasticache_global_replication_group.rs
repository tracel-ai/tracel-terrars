use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ElasticacheGlobalReplicationGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_failover_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_replication_group_description: Option<PrimField<String>>,
    global_replication_group_id_suffix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_node_groups: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_group_name: Option<PrimField<String>>,
    primary_replication_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElasticacheGlobalReplicationGroupTimeoutsEl>,
}

struct ElasticacheGlobalReplicationGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticacheGlobalReplicationGroupData>,
}

#[derive(Clone)]
pub struct ElasticacheGlobalReplicationGroup(Rc<ElasticacheGlobalReplicationGroup_>);

impl ElasticacheGlobalReplicationGroup {
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

    #[doc = "Set the field `automatic_failover_enabled`.\n"]
    pub fn set_automatic_failover_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_failover_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_node_type`.\n"]
    pub fn set_cache_node_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_node_type = Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }

    #[doc = "Set the field `global_replication_group_description`.\n"]
    pub fn set_global_replication_group_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .global_replication_group_description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `num_node_groups`.\n"]
    pub fn set_num_node_groups(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_node_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_group_name`.\n"]
    pub fn set_parameter_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElasticacheGlobalReplicationGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.at_rest_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_failover_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_node_type` after provisioning.\n"]
    pub fn cache_node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_node_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version_actual", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_node_groups` after provisioning.\n"]
    pub fn global_node_groups(
        &self,
    ) -> SetRef<ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.global_node_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_description` after provisioning.\n"]
    pub fn global_replication_group_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.global_replication_group_description",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id_suffix` after provisioning.\n"]
    pub fn global_replication_group_id_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id_suffix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_node_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_replication_group_id` after provisioning.\n"]
    pub fn primary_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_replication_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheGlobalReplicationGroupTimeoutsElRef {
        ElasticacheGlobalReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ElasticacheGlobalReplicationGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ElasticacheGlobalReplicationGroup {}

impl ToListMappable for ElasticacheGlobalReplicationGroup {
    type O = ListRef<ElasticacheGlobalReplicationGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ElasticacheGlobalReplicationGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_elasticache_global_replication_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticacheGlobalReplicationGroup {
    pub tf_id: String,
    #[doc = ""]
    pub global_replication_group_id_suffix: PrimField<String>,
    #[doc = ""]
    pub primary_replication_group_id: PrimField<String>,
}

impl BuildElasticacheGlobalReplicationGroup {
    pub fn build(self, stack: &mut Stack) -> ElasticacheGlobalReplicationGroup {
        let out = ElasticacheGlobalReplicationGroup(Rc::new(ElasticacheGlobalReplicationGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticacheGlobalReplicationGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                automatic_failover_enabled: core::default::Default::default(),
                cache_node_type: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                global_replication_group_description: core::default::Default::default(),
                global_replication_group_id_suffix: self.global_replication_group_id_suffix,
                id: core::default::Default::default(),
                num_node_groups: core::default::Default::default(),
                parameter_group_name: core::default::Default::default(),
                primary_replication_group_id: self.primary_replication_group_id,
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticacheGlobalReplicationGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheGlobalReplicationGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ElasticacheGlobalReplicationGroupRef {
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

    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.at_rest_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_failover_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_node_type` after provisioning.\n"]
    pub fn cache_node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_node_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version_actual", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_node_groups` after provisioning.\n"]
    pub fn global_node_groups(
        &self,
    ) -> SetRef<ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.global_node_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_description` after provisioning.\n"]
    pub fn global_replication_group_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.global_replication_group_description",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id_suffix` after provisioning.\n"]
    pub fn global_replication_group_id_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id_suffix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_node_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `primary_replication_group_id` after provisioning.\n"]
    pub fn primary_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_replication_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheGlobalReplicationGroupTimeoutsElRef {
        ElasticacheGlobalReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    global_node_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slots: Option<PrimField<String>>,
}

impl ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
    #[doc = "Set the field `global_node_group_id`.\n"]
    pub fn set_global_node_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.global_node_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `slots`.\n"]
    pub fn set_slots(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slots = Some(v.into());
        self
    }
}

impl ToListMappable for ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
    type O = BlockAssignable<ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {}

impl BuildElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
    pub fn build(self) -> ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
        ElasticacheGlobalReplicationGroupGlobalNodeGroupsEl {
            global_node_group_id: core::default::Default::default(),
            slots: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef {
        ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheGlobalReplicationGroupGlobalNodeGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `global_node_group_id` after provisioning.\n"]
    pub fn global_node_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_node_group_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `slots` after provisioning.\n"]
    pub fn slots(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slots", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticacheGlobalReplicationGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ElasticacheGlobalReplicationGroupTimeoutsEl {
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

impl ToListMappable for ElasticacheGlobalReplicationGroupTimeoutsEl {
    type O = BlockAssignable<ElasticacheGlobalReplicationGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheGlobalReplicationGroupTimeoutsEl {}

impl BuildElasticacheGlobalReplicationGroupTimeoutsEl {
    pub fn build(self) -> ElasticacheGlobalReplicationGroupTimeoutsEl {
        ElasticacheGlobalReplicationGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheGlobalReplicationGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheGlobalReplicationGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheGlobalReplicationGroupTimeoutsElRef {
        ElasticacheGlobalReplicationGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheGlobalReplicationGroupTimeoutsElRef {
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
