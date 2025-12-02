use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElasticacheReplicationGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    at_rest_encryption_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token_update_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_minor_version_upgrade: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_failover_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_tiering_enabled: Option<PrimField<bool>>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_snapshot_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_replication_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_discovery: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_az_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_cache_clusters: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_node_groups: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_cache_cluster_azs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas_per_node_group: Option<PrimField<f64>>,
    replication_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_retention_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_delivery_configuration: Option<Vec<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_configuration: Option<Vec<ElasticacheReplicationGroupNodeGroupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElasticacheReplicationGroupTimeoutsEl>,
    dynamic: ElasticacheReplicationGroupDynamic,
}
struct ElasticacheReplicationGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticacheReplicationGroupData>,
}
#[derive(Clone)]
pub struct ElasticacheReplicationGroup(Rc<ElasticacheReplicationGroup_>);
impl ElasticacheReplicationGroup {
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
    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = Some(v.into());
        self
    }
    #[doc = "Set the field `at_rest_encryption_enabled`.\n"]
    pub fn set_at_rest_encryption_enabled(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().at_rest_encryption_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auth_token = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_token_update_strategy`.\n"]
    pub fn set_auth_token_update_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auth_token_update_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `automatic_failover_enabled`.\n"]
    pub fn set_automatic_failover_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_failover_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_mode`.\n"]
    pub fn set_cluster_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `data_tiering_enabled`.\n"]
    pub fn set_data_tiering_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().data_tiering_enabled = Some(v.into());
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
    #[doc = "Set the field `final_snapshot_identifier`.\n"]
    pub fn set_final_snapshot_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().final_snapshot_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `global_replication_group_id`.\n"]
    pub fn set_global_replication_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().global_replication_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_discovery`.\n"]
    pub fn set_ip_discovery(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_discovery = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_az_enabled`.\n"]
    pub fn set_multi_az_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_type = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_topic_arn`.\n"]
    pub fn set_notification_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notification_topic_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `num_cache_clusters`.\n"]
    pub fn set_num_cache_clusters(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_cache_clusters = Some(v.into());
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
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_cache_cluster_azs`.\n"]
    pub fn set_preferred_cache_cluster_azs(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_cache_cluster_azs = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replicas_per_node_group`.\n"]
    pub fn set_replicas_per_node_group(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replicas_per_node_group = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_names`.\n"]
    pub fn set_security_group_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_names = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_arns`.\n"]
    pub fn set_snapshot_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().snapshot_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_name`.\n"]
    pub fn set_snapshot_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot_name = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_retention_limit`.\n"]
    pub fn set_snapshot_retention_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().snapshot_retention_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_window`.\n"]
    pub fn set_snapshot_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot_window = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_group_name`.\n"]
    pub fn set_subnet_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_group_name = Some(v.into());
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
    #[doc = "Set the field `transit_encryption_enabled`.\n"]
    pub fn set_transit_encryption_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `transit_encryption_mode`.\n"]
    pub fn set_transit_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `user_group_ids`.\n"]
    pub fn set_user_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().user_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `log_delivery_configuration`.\n"]
    pub fn set_log_delivery_configuration(
        self,
        v: impl Into<BlockAssignable<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_group_configuration`.\n"]
    pub fn set_node_group_configuration(
        self,
        v: impl Into<BlockAssignable<ElasticacheReplicationGroupNodeGroupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_group_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_group_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElasticacheReplicationGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_immediately", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.at_rest_encryption_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token_update_strategy` after provisioning.\n"]
    pub fn auth_token_update_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_update_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_failover_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_tiering_enabled` after provisioning.\n"]
    pub fn data_tiering_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_tiering_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_discovery` after provisioning.\n"]
    pub fn ip_discovery(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.member_clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_cache_clusters", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_cache_cluster_azs` after provisioning.\n"]
    pub fn preferred_cache_cluster_azs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_cache_cluster_azs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reader_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicas_per_node_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_arns` after provisioning.\n"]
    pub fn snapshot_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.snapshot_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_retention_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_group_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\n"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_group_ids` after provisioning.\n"]
    pub fn user_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ElasticacheReplicationGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ElasticacheReplicationGroup {}
impl ToListMappable for ElasticacheReplicationGroup {
    type O = ListRef<ElasticacheReplicationGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ElasticacheReplicationGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_elasticache_replication_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElasticacheReplicationGroup {
    pub tf_id: String,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub replication_group_id: PrimField<String>,
}
impl BuildElasticacheReplicationGroup {
    pub fn build(self, stack: &mut Stack) -> ElasticacheReplicationGroup {
        let out = ElasticacheReplicationGroup(Rc::new(ElasticacheReplicationGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticacheReplicationGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_immediately: core::default::Default::default(),
                at_rest_encryption_enabled: core::default::Default::default(),
                auth_token: core::default::Default::default(),
                auth_token_update_strategy: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                automatic_failover_enabled: core::default::Default::default(),
                cluster_mode: core::default::Default::default(),
                data_tiering_enabled: core::default::Default::default(),
                description: self.description,
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                global_replication_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_discovery: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                multi_az_enabled: core::default::Default::default(),
                network_type: core::default::Default::default(),
                node_type: core::default::Default::default(),
                notification_topic_arn: core::default::Default::default(),
                num_cache_clusters: core::default::Default::default(),
                num_node_groups: core::default::Default::default(),
                parameter_group_name: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_cache_cluster_azs: core::default::Default::default(),
                region: core::default::Default::default(),
                replicas_per_node_group: core::default::Default::default(),
                replication_group_id: self.replication_group_id,
                security_group_ids: core::default::Default::default(),
                security_group_names: core::default::Default::default(),
                snapshot_arns: core::default::Default::default(),
                snapshot_name: core::default::Default::default(),
                snapshot_retention_limit: core::default::Default::default(),
                snapshot_window: core::default::Default::default(),
                subnet_group_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_encryption_enabled: core::default::Default::default(),
                transit_encryption_mode: core::default::Default::default(),
                user_group_ids: core::default::Default::default(),
                log_delivery_configuration: core::default::Default::default(),
                node_group_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElasticacheReplicationGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticacheReplicationGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElasticacheReplicationGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_immediately", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.at_rest_encryption_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token_update_strategy` after provisioning.\n"]
    pub fn auth_token_update_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_update_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_failover_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_tiering_enabled` after provisioning.\n"]
    pub fn data_tiering_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_tiering_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.final_snapshot_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_replication_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_discovery` after provisioning.\n"]
    pub fn ip_discovery(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.member_clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_cache_clusters", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_cache_cluster_azs` after provisioning.\n"]
    pub fn preferred_cache_cluster_azs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_cache_cluster_azs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reader_endpoint_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replicas_per_node_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_arns` after provisioning.\n"]
    pub fn snapshot_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.snapshot_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_retention_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_group_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\n"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_group_ids` after provisioning.\n"]
    pub fn user_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElasticacheReplicationGroupLogDeliveryConfigurationEl {
    destination: PrimField<String>,
    destination_type: PrimField<String>,
    log_format: PrimField<String>,
    log_type: PrimField<String>,
}
impl ElasticacheReplicationGroupLogDeliveryConfigurationEl {}
impl ToListMappable for ElasticacheReplicationGroupLogDeliveryConfigurationEl {
    type O = BlockAssignable<ElasticacheReplicationGroupLogDeliveryConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[doc = ""]
    pub destination: PrimField<String>,
    #[doc = ""]
    pub destination_type: PrimField<String>,
    #[doc = ""]
    pub log_format: PrimField<String>,
    #[doc = ""]
    pub log_type: PrimField<String>,
}
impl BuildElasticacheReplicationGroupLogDeliveryConfigurationEl {
    pub fn build(self) -> ElasticacheReplicationGroupLogDeliveryConfigurationEl {
        ElasticacheReplicationGroupLogDeliveryConfigurationEl {
            destination: self.destination,
            destination_type: self.destination_type,
            log_format: self.log_format,
            log_type: self.log_type,
        }
    }
}
pub struct ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
        ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ElasticacheReplicationGroupNodeGroupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_outpost_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_availability_zones: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_outpost_arns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slots: Option<PrimField<String>>,
}
impl ElasticacheReplicationGroupNodeGroupConfigurationEl {
    #[doc = "Set the field `node_group_id`.\n"]
    pub fn set_node_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_availability_zone`.\n"]
    pub fn set_primary_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_outpost_arn`.\n"]
    pub fn set_primary_outpost_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_outpost_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_availability_zones`.\n"]
    pub fn set_replica_availability_zones(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.replica_availability_zones = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_count`.\n"]
    pub fn set_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_outpost_arns`.\n"]
    pub fn set_replica_outpost_arns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.replica_outpost_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `slots`.\n"]
    pub fn set_slots(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slots = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticacheReplicationGroupNodeGroupConfigurationEl {
    type O = BlockAssignable<ElasticacheReplicationGroupNodeGroupConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticacheReplicationGroupNodeGroupConfigurationEl {}
impl BuildElasticacheReplicationGroupNodeGroupConfigurationEl {
    pub fn build(self) -> ElasticacheReplicationGroupNodeGroupConfigurationEl {
        ElasticacheReplicationGroupNodeGroupConfigurationEl {
            node_group_id: core::default::Default::default(),
            primary_availability_zone: core::default::Default::default(),
            primary_outpost_arn: core::default::Default::default(),
            replica_availability_zones: core::default::Default::default(),
            replica_count: core::default::Default::default(),
            replica_outpost_arns: core::default::Default::default(),
            slots: core::default::Default::default(),
        }
    }
}
pub struct ElasticacheReplicationGroupNodeGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticacheReplicationGroupNodeGroupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticacheReplicationGroupNodeGroupConfigurationElRef {
        ElasticacheReplicationGroupNodeGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticacheReplicationGroupNodeGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_group_id` after provisioning.\n"]
    pub fn node_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_availability_zone` after provisioning.\n"]
    pub fn primary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_outpost_arn` after provisioning.\n"]
    pub fn primary_outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_outpost_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replica_availability_zones` after provisioning.\n"]
    pub fn replica_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_availability_zones", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\n"]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replica_outpost_arns` after provisioning.\n"]
    pub fn replica_outpost_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_outpost_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slots` after provisioning.\n"]
    pub fn slots(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slots", self.base))
    }
}
#[derive(Serialize)]
pub struct ElasticacheReplicationGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ElasticacheReplicationGroupTimeoutsEl {
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
impl ToListMappable for ElasticacheReplicationGroupTimeoutsEl {
    type O = BlockAssignable<ElasticacheReplicationGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticacheReplicationGroupTimeoutsEl {}
impl BuildElasticacheReplicationGroupTimeoutsEl {
    pub fn build(self) -> ElasticacheReplicationGroupTimeoutsEl {
        ElasticacheReplicationGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ElasticacheReplicationGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticacheReplicationGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticacheReplicationGroupTimeoutsElRef {
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
struct ElasticacheReplicationGroupDynamic {
    log_delivery_configuration:
        Option<DynamicBlock<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
    node_group_configuration:
        Option<DynamicBlock<ElasticacheReplicationGroupNodeGroupConfigurationEl>>,
}
