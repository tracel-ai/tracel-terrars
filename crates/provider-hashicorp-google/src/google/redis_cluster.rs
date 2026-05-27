use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RedisClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_configs: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ca_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ca_pool: Option<PrimField<String>>,
    shard_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automated_backup_config: Option<Vec<RedisClusterAutomatedBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_cluster_replication_config: Option<Vec<RedisClusterCrossClusterReplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_source: Option<Vec<RedisClusterGcsSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<RedisClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_backup_source: Option<Vec<RedisClusterManagedBackupSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence_config: Option<Vec<RedisClusterPersistenceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_configs: Option<Vec<RedisClusterPscConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RedisClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_distribution_config: Option<Vec<RedisClusterZoneDistributionConfigEl>>,
    dynamic: RedisClusterDynamic,
}
struct RedisCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedisClusterData>,
}
#[derive(Clone)]
pub struct RedisCluster(Rc<RedisCluster_>);
impl RedisCluster {
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
    #[doc = "Set the field `authorization_mode`.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn set_authorization_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection_enabled`.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn set_deletion_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_version`.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn set_maintenance_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type`.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn set_node_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_configs`.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn set_redis_configs(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().redis_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe name of the region of the Redis cluster."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_count`.\nOptional. The number of replica nodes per shard."]
    pub fn set_replica_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_mode`.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn set_server_ca_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_ca_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn set_server_ca_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_ca_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `transit_encryption_mode`.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn set_transit_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `automated_backup_config`.\n"]
    pub fn set_automated_backup_config(
        self,
        v: impl Into<BlockAssignable<RedisClusterAutomatedBackupConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automated_backup_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automated_backup_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cross_cluster_replication_config`.\n"]
    pub fn set_cross_cluster_replication_config(
        self,
        v: impl Into<BlockAssignable<RedisClusterCrossClusterReplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_cluster_replication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .cross_cluster_replication_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_source`.\n"]
    pub fn set_gcs_source(self, v: impl Into<BlockAssignable<RedisClusterGcsSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<RedisClusterMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `managed_backup_source`.\n"]
    pub fn set_managed_backup_source(
        self,
        v: impl Into<BlockAssignable<RedisClusterManagedBackupSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_backup_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_backup_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `persistence_config`.\n"]
    pub fn set_persistence_config(
        self,
        v: impl Into<BlockAssignable<RedisClusterPersistenceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistence_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistence_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_configs`.\n"]
    pub fn set_psc_configs(self, v: impl Into<BlockAssignable<RedisClusterPscConfigsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RedisClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `zone_distribution_config`.\n"]
    pub fn set_zone_distribution_config(
        self,
        v: impl Into<BlockAssignable<RedisClusterZoneDistributionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zone_distribution_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zone_distribution_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<RedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<RedisClusterMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<RedisClusterManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.precise_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<RedisClusterPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> ListRef<RedisClusterPscServiceAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<RedisClusterStateInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> ListRef<RedisClusterAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\n"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> ListRef<RedisClusterGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<RedisClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> ListRef<RedisClusterManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<RedisClusterPersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> ListRef<RedisClusterPscConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> ListRef<RedisClusterZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
impl Referable for RedisCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RedisCluster {}
impl ToListMappable for RedisCluster {
    type O = ListRef<RedisClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RedisCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_redis_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRedisCluster {
    pub tf_id: String,
    #[doc = "Required. Number of shards for the Redis cluster."]
    pub shard_count: PrimField<f64>,
}
impl BuildRedisCluster {
    pub fn build(self, stack: &mut Stack) -> RedisCluster {
        let out = RedisCluster(Rc::new(RedisCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedisClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorization_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                name: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                redis_configs: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                server_ca_mode: core::default::Default::default(),
                server_ca_pool: core::default::Default::default(),
                shard_count: self.shard_count,
                transit_encryption_mode: core::default::Default::default(),
                automated_backup_config: core::default::Default::default(),
                cross_cluster_replication_config: core::default::Default::default(),
                gcs_source: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                managed_backup_source: core::default::Default::default(),
                persistence_config: core::default::Default::default(),
                psc_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                zone_distribution_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RedisClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RedisClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<RedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<RedisClusterMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<RedisClusterManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.precise_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<RedisClusterPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> ListRef<RedisClusterPscServiceAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<RedisClusterStateInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> ListRef<RedisClusterAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\n"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> ListRef<RedisClusterGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<RedisClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> ListRef<RedisClusterManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<RedisClusterPersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> ListRef<RedisClusterPscConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> ListRef<RedisClusterZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}
impl RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterDiscoveryEndpointsElPscConfigEl {
    type O = BlockAssignable<RedisClusterDiscoveryEndpointsElPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterDiscoveryEndpointsElPscConfigEl {}
impl BuildRedisClusterDiscoveryEndpointsElPscConfigEl {
    pub fn build(self) -> RedisClusterDiscoveryEndpointsElPscConfigEl {
        RedisClusterDiscoveryEndpointsElPscConfigEl {
            network: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterDiscoveryEndpointsElPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterDiscoveryEndpointsElPscConfigElRef {
        RedisClusterDiscoveryEndpointsElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>,
}
impl RedisClusterDiscoveryEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>,
    ) -> Self {
        self.psc_config = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterDiscoveryEndpointsEl {
    type O = BlockAssignable<RedisClusterDiscoveryEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterDiscoveryEndpointsEl {}
impl BuildRedisClusterDiscoveryEndpointsEl {
    pub fn build(self) -> RedisClusterDiscoveryEndpointsEl {
        RedisClusterDiscoveryEndpointsEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            psc_config: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterDiscoveryEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterDiscoveryEndpointsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterDiscoveryEndpointsElRef {
        RedisClusterDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterDiscoveryEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<RedisClusterDiscoveryEndpointsElPscConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl RedisClusterMaintenanceScheduleEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_deadline_time`.\n"]
    pub fn set_schedule_deadline_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_deadline_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterMaintenanceScheduleEl {
    type O = BlockAssignable<RedisClusterMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterMaintenanceScheduleEl {}
impl BuildRedisClusterMaintenanceScheduleEl {
    pub fn build(self) -> RedisClusterMaintenanceScheduleEl {
        RedisClusterMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterMaintenanceScheduleElRef {
        RedisClusterMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterMaintenanceScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule_deadline_time` after provisioning.\n"]
    pub fn schedule_deadline_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_deadline_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterManagedServerCaElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<ListField<PrimField<String>>>,
}
impl RedisClusterManagedServerCaElCaCertsEl {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.certificates = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterManagedServerCaElCaCertsEl {
    type O = BlockAssignable<RedisClusterManagedServerCaElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterManagedServerCaElCaCertsEl {}
impl BuildRedisClusterManagedServerCaElCaCertsEl {
    pub fn build(self) -> RedisClusterManagedServerCaElCaCertsEl {
        RedisClusterManagedServerCaElCaCertsEl {
            certificates: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterManagedServerCaElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterManagedServerCaElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterManagedServerCaElCaCertsElRef {
        RedisClusterManagedServerCaElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterManagedServerCaElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterManagedServerCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<RedisClusterManagedServerCaElCaCertsEl>>,
}
impl RedisClusterManagedServerCaEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<RedisClusterManagedServerCaElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterManagedServerCaEl {
    type O = BlockAssignable<RedisClusterManagedServerCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterManagedServerCaEl {}
impl BuildRedisClusterManagedServerCaEl {
    pub fn build(self) -> RedisClusterManagedServerCaEl {
        RedisClusterManagedServerCaEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterManagedServerCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterManagedServerCaElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterManagedServerCaElRef {
        RedisClusterManagedServerCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterManagedServerCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<RedisClusterManagedServerCaElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterPscConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
}
impl RedisClusterPscConnectionsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterPscConnectionsEl {
    type O = BlockAssignable<RedisClusterPscConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPscConnectionsEl {}
impl BuildRedisClusterPscConnectionsEl {
    pub fn build(self) -> RedisClusterPscConnectionsEl {
        RedisClusterPscConnectionsEl {
            address: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterPscConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPscConnectionsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPscConnectionsElRef {
        RedisClusterPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPscConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterPscServiceAttachmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl RedisClusterPscServiceAttachmentsEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterPscServiceAttachmentsEl {
    type O = BlockAssignable<RedisClusterPscServiceAttachmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPscServiceAttachmentsEl {}
impl BuildRedisClusterPscServiceAttachmentsEl {
    pub fn build(self) -> RedisClusterPscServiceAttachmentsEl {
        RedisClusterPscServiceAttachmentsEl {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterPscServiceAttachmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPscServiceAttachmentsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPscServiceAttachmentsElRef {
        RedisClusterPscServiceAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPscServiceAttachmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_shard_count: Option<PrimField<f64>>,
}
impl RedisClusterStateInfoElUpdateInfoEl {
    #[doc = "Set the field `target_replica_count`.\n"]
    pub fn set_target_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `target_shard_count`.\n"]
    pub fn set_target_shard_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_shard_count = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterStateInfoElUpdateInfoEl {
    type O = BlockAssignable<RedisClusterStateInfoElUpdateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterStateInfoElUpdateInfoEl {}
impl BuildRedisClusterStateInfoElUpdateInfoEl {
    pub fn build(self) -> RedisClusterStateInfoElUpdateInfoEl {
        RedisClusterStateInfoElUpdateInfoEl {
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterStateInfoElUpdateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterStateInfoElUpdateInfoElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterStateInfoElUpdateInfoElRef {
        RedisClusterStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterStateInfoElUpdateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_replica_count` after provisioning.\n"]
    pub fn target_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_shard_count` after provisioning.\n"]
    pub fn target_shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_shard_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterStateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update_info: Option<ListField<RedisClusterStateInfoElUpdateInfoEl>>,
}
impl RedisClusterStateInfoEl {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl Into<ListField<RedisClusterStateInfoElUpdateInfoEl>>,
    ) -> Self {
        self.update_info = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterStateInfoEl {
    type O = BlockAssignable<RedisClusterStateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterStateInfoEl {}
impl BuildRedisClusterStateInfoEl {
    pub fn build(self) -> RedisClusterStateInfoEl {
        RedisClusterStateInfoEl {
            update_info: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterStateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterStateInfoElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterStateInfoElRef {
        RedisClusterStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterStateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> ListRef<RedisClusterStateInfoElUpdateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    hours: PrimField<f64>,
}
impl RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {}
impl ToListMappable for RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    type O =
        BlockAssignable<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[doc = "Hours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: PrimField<f64>,
}
impl BuildRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    pub fn build(self) -> RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
        RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl { hours: self.hours }
    }
}
pub struct RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
        RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElDynamic {
    start_time: Option<
        DynamicBlock<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<Vec<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>>,
    dynamic: RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElDynamic,
}
impl RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    type O = BlockAssignable<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {}
impl BuildRedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
    pub fn build(self) -> RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
        RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl {
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
        RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterAutomatedBackupConfigElDynamic {
    fixed_frequency_schedule:
        Option<DynamicBlock<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
}
#[derive(Serialize)]
pub struct RedisClusterAutomatedBackupConfigEl {
    retention: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_frequency_schedule:
        Option<Vec<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    dynamic: RedisClusterAutomatedBackupConfigElDynamic,
}
impl RedisClusterAutomatedBackupConfigEl {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_frequency_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_frequency_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterAutomatedBackupConfigEl {
    type O = BlockAssignable<RedisClusterAutomatedBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterAutomatedBackupConfigEl {
    #[doc = "How long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub retention: PrimField<String>,
}
impl BuildRedisClusterAutomatedBackupConfigEl {
    pub fn build(self) -> RedisClusterAutomatedBackupConfigEl {
        RedisClusterAutomatedBackupConfigEl {
            retention: self.retention,
            fixed_frequency_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterAutomatedBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterAutomatedBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterAutomatedBackupConfigElRef {
        RedisClusterAutomatedBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterAutomatedBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nHow long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> ListRef<RedisClusterAutomatedBackupConfigElFixedFrequencyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_frequency_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    type O =
        BlockAssignable<RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {}
impl BuildRedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
    pub fn build(self) -> RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
        RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
        RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    type O =
        BlockAssignable<RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {}
impl BuildRedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
    pub fn build(
        self,
    ) -> RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
        RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
        RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigElMembershipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_cluster:
        Option<ListField<RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_clusters: Option<
        ListField<RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl>,
    >,
}
impl RedisClusterCrossClusterReplicationConfigElMembershipEl {
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v: impl Into<ListField<RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterEl>>,
    ) -> Self {
        self.primary_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v: impl Into<
            ListField<RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersEl>,
        >,
    ) -> Self {
        self.secondary_clusters = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigElMembershipEl {
    type O = BlockAssignable<RedisClusterCrossClusterReplicationConfigElMembershipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigElMembershipEl {}
impl BuildRedisClusterCrossClusterReplicationConfigElMembershipEl {
    pub fn build(self) -> RedisClusterCrossClusterReplicationConfigElMembershipEl {
        RedisClusterCrossClusterReplicationConfigElMembershipEl {
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElMembershipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElMembershipElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterCrossClusterReplicationConfigElMembershipElRef {
        RedisClusterCrossClusterReplicationConfigElMembershipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElMembershipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElMembershipElPrimaryClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElMembershipElSecondaryClustersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_clusters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
}
impl RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    #[doc = "Set the field `cluster`.\nThe full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    type O = BlockAssignable<RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {}
impl BuildRedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
    pub fn build(self) -> RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
        RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl {
            cluster: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
        RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the primary cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
}
impl RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    #[doc = "Set the field `cluster`.\nThe full resource path of the secondary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    type O = BlockAssignable<RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {}
impl BuildRedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
    pub fn build(self) -> RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
        RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl {
            cluster: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
        RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe full resource path of the secondary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the secondary cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterCrossClusterReplicationConfigElDynamic {
    primary_cluster:
        Option<DynamicBlock<RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>>,
    secondary_clusters:
        Option<DynamicBlock<RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>>,
}
#[derive(Serialize)]
pub struct RedisClusterCrossClusterReplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_cluster: Option<Vec<RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_clusters: Option<Vec<RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>>,
    dynamic: RedisClusterCrossClusterReplicationConfigElDynamic,
}
impl RedisClusterCrossClusterReplicationConfigEl {
    #[doc = "Set the field `cluster_role`.\nThe role of the cluster in cross cluster replication. Supported values are:\n\n1. 'CLUSTER_ROLE_UNSPECIFIED': This is an independent cluster that has never participated in cross cluster replication. It allows both reads and writes.\n\n1. 'NONE': This is an independent cluster that previously participated in cross cluster replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n\n1. 'PRIMARY': This cluster serves as the replication source for secondary clusters that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n\n1. 'SECONDARY': This cluster replicates data from the primary cluster. It allows only reads. Possible values: [\"CLUSTER_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn set_cluster_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_role = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterCrossClusterReplicationConfigElPrimaryClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterCrossClusterReplicationConfigElSecondaryClustersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secondary_clusters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_clusters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterCrossClusterReplicationConfigEl {
    type O = BlockAssignable<RedisClusterCrossClusterReplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterCrossClusterReplicationConfigEl {}
impl BuildRedisClusterCrossClusterReplicationConfigEl {
    pub fn build(self) -> RedisClusterCrossClusterReplicationConfigEl {
        RedisClusterCrossClusterReplicationConfigEl {
            cluster_role: core::default::Default::default(),
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterCrossClusterReplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterCrossClusterReplicationConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterCrossClusterReplicationConfigElRef {
        RedisClusterCrossClusterReplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterCrossClusterReplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_role` after provisioning.\nThe role of the cluster in cross cluster replication. Supported values are:\n\n1. 'CLUSTER_ROLE_UNSPECIFIED': This is an independent cluster that has never participated in cross cluster replication. It allows both reads and writes.\n\n1. 'NONE': This is an independent cluster that previously participated in cross cluster replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n\n1. 'PRIMARY': This cluster serves as the replication source for secondary clusters that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n\n1. 'SECONDARY': This cluster replicates data from the primary cluster. It allows only reads. Possible values: [\"CLUSTER_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_role", self.base))
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nAn output only view of all the member clusters participating in cross cluster replication. This field is populated for all the member clusters irrespective of their cluster role."]
    pub fn membership(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElMembershipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last time cross cluster replication config was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElPrimaryClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> ListRef<RedisClusterCrossClusterReplicationConfigElSecondaryClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_clusters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterGcsSourceEl {
    uris: SetField<PrimField<String>>,
}
impl RedisClusterGcsSourceEl {}
impl ToListMappable for RedisClusterGcsSourceEl {
    type O = BlockAssignable<RedisClusterGcsSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterGcsSourceEl {
    #[doc = "URIs of the GCS objects to import. Example: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub uris: SetField<PrimField<String>>,
}
impl BuildRedisClusterGcsSourceEl {
    pub fn build(self) -> RedisClusterGcsSourceEl {
        RedisClusterGcsSourceEl { uris: self.uris }
    }
}
pub struct RedisClusterGcsSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterGcsSourceElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterGcsSourceElRef {
        RedisClusterGcsSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterGcsSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs of the GCS objects to import. Example: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub fn uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O = BlockAssignable<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElDynamic {
    start_time:
        Option<DynamicBlock<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}
#[derive(Serialize)]
pub struct RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    day: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<Vec<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    dynamic: RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElDynamic,
}
impl RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Required. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: PrimField<String>,
}
impl BuildRedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
        RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nRequired. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nOutput only. Duration of the maintenance window.\nThe current window is fixed at 1 hour.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterMaintenancePolicyElDynamic {
    weekly_maintenance_window:
        Option<DynamicBlock<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
#[derive(Serialize)]
pub struct RedisClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<Vec<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    dynamic: RedisClusterMaintenancePolicyElDynamic,
}
impl RedisClusterMaintenancePolicyEl {
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterMaintenancePolicyEl {
    type O = BlockAssignable<RedisClusterMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterMaintenancePolicyEl {}
impl BuildRedisClusterMaintenancePolicyEl {
    pub fn build(self) -> RedisClusterMaintenancePolicyEl {
        RedisClusterMaintenancePolicyEl {
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterMaintenancePolicyElRef {
        RedisClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<RedisClusterMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisClusterManagedBackupSourceEl {
    backup: PrimField<String>,
}
impl RedisClusterManagedBackupSourceEl {}
impl ToListMappable for RedisClusterManagedBackupSourceEl {
    type O = BlockAssignable<RedisClusterManagedBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterManagedBackupSourceEl {
    #[doc = "Example: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub backup: PrimField<String>,
}
impl BuildRedisClusterManagedBackupSourceEl {
    pub fn build(self) -> RedisClusterManagedBackupSourceEl {
        RedisClusterManagedBackupSourceEl {
            backup: self.backup,
        }
    }
}
pub struct RedisClusterManagedBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterManagedBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterManagedBackupSourceElRef {
        RedisClusterManagedBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterManagedBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\nExample: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterPersistenceConfigElAofConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    append_fsync: Option<PrimField<String>>,
}
impl RedisClusterPersistenceConfigElAofConfigEl {
    #[doc = "Set the field `append_fsync`.\nOptional. Available fsync modes.\n\n- NO - Do not explicitly call fsync(). Rely on OS defaults.\n- EVERYSEC - Call fsync() once per second in a background thread. A balance between performance and durability.\n- ALWAYS - Call fsync() for earch write command. Possible values: [\"APPEND_FSYNC_UNSPECIFIED\", \"NO\", \"EVERYSEC\", \"ALWAYS\"]"]
    pub fn set_append_fsync(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.append_fsync = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterPersistenceConfigElAofConfigEl {
    type O = BlockAssignable<RedisClusterPersistenceConfigElAofConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPersistenceConfigElAofConfigEl {}
impl BuildRedisClusterPersistenceConfigElAofConfigEl {
    pub fn build(self) -> RedisClusterPersistenceConfigElAofConfigEl {
        RedisClusterPersistenceConfigElAofConfigEl {
            append_fsync: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterPersistenceConfigElAofConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPersistenceConfigElAofConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPersistenceConfigElAofConfigElRef {
        RedisClusterPersistenceConfigElAofConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPersistenceConfigElAofConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_fsync` after provisioning.\nOptional. Available fsync modes.\n\n- NO - Do not explicitly call fsync(). Rely on OS defaults.\n- EVERYSEC - Call fsync() once per second in a background thread. A balance between performance and durability.\n- ALWAYS - Call fsync() for earch write command. Possible values: [\"APPEND_FSYNC_UNSPECIFIED\", \"NO\", \"EVERYSEC\", \"ALWAYS\"]"]
    pub fn append_fsync(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterPersistenceConfigElRdbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl RedisClusterPersistenceConfigElRdbConfigEl {
    #[doc = "Set the field `rdb_snapshot_period`.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"SNAPSHOT_PERIOD_UNSPECIFIED\", \"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn set_rdb_snapshot_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_period = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_start_time`.\nThe time that the first snapshot was/will be attempted, and to which\nfuture snapshots will be aligned.\nIf not provided, the current time will be used."]
    pub fn set_rdb_snapshot_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterPersistenceConfigElRdbConfigEl {
    type O = BlockAssignable<RedisClusterPersistenceConfigElRdbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPersistenceConfigElRdbConfigEl {}
impl BuildRedisClusterPersistenceConfigElRdbConfigEl {
    pub fn build(self) -> RedisClusterPersistenceConfigElRdbConfigEl {
        RedisClusterPersistenceConfigElRdbConfigEl {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterPersistenceConfigElRdbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPersistenceConfigElRdbConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPersistenceConfigElRdbConfigElRef {
        RedisClusterPersistenceConfigElRdbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPersistenceConfigElRdbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"SNAPSHOT_PERIOD_UNSPECIFIED\", \"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn rdb_snapshot_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\nThe time that the first snapshot was/will be attempted, and to which\nfuture snapshots will be aligned.\nIf not provided, the current time will be used."]
    pub fn rdb_snapshot_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_start_time", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct RedisClusterPersistenceConfigElDynamic {
    aof_config: Option<DynamicBlock<RedisClusterPersistenceConfigElAofConfigEl>>,
    rdb_config: Option<DynamicBlock<RedisClusterPersistenceConfigElRdbConfigEl>>,
}
#[derive(Serialize)]
pub struct RedisClusterPersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aof_config: Option<Vec<RedisClusterPersistenceConfigElAofConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_config: Option<Vec<RedisClusterPersistenceConfigElRdbConfigEl>>,
    dynamic: RedisClusterPersistenceConfigElDynamic,
}
impl RedisClusterPersistenceConfigEl {
    #[doc = "Set the field `mode`.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence (both backup and restore) is disabled for the cluster.\n- RDB: RDB based Persistence is enabled.\n- AOF: AOF based Persistence is enabled. Possible values: [\"PERSISTENCE_MODE_UNSPECIFIED\", \"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterPersistenceConfigElAofConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aof_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aof_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rdb_config`.\n"]
    pub fn set_rdb_config(
        mut self,
        v: impl Into<BlockAssignable<RedisClusterPersistenceConfigElRdbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rdb_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rdb_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RedisClusterPersistenceConfigEl {
    type O = BlockAssignable<RedisClusterPersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPersistenceConfigEl {}
impl BuildRedisClusterPersistenceConfigEl {
    pub fn build(self) -> RedisClusterPersistenceConfigEl {
        RedisClusterPersistenceConfigEl {
            mode: core::default::Default::default(),
            aof_config: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisClusterPersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPersistenceConfigElRef {
        RedisClusterPersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence (both backup and restore) is disabled for the cluster.\n- RDB: RDB based Persistence is enabled.\n- AOF: AOF based Persistence is enabled. Possible values: [\"PERSISTENCE_MODE_UNSPECIFIED\", \"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> ListRef<RedisClusterPersistenceConfigElAofConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }
    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> ListRef<RedisClusterPersistenceConfigElRdbConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterPscConfigsEl {
    network: PrimField<String>,
}
impl RedisClusterPscConfigsEl {}
impl ToListMappable for RedisClusterPscConfigsEl {
    type O = BlockAssignable<RedisClusterPscConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterPscConfigsEl {
    #[doc = "Required. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub network: PrimField<String>,
}
impl BuildRedisClusterPscConfigsEl {
    pub fn build(self) -> RedisClusterPscConfigsEl {
        RedisClusterPscConfigsEl {
            network: self.network,
        }
    }
}
pub struct RedisClusterPscConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterPscConfigsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPscConfigsElRef {
        RedisClusterPscConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterPscConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct RedisClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl RedisClusterTimeoutsEl {
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
impl ToListMappable for RedisClusterTimeoutsEl {
    type O = BlockAssignable<RedisClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterTimeoutsEl {}
impl BuildRedisClusterTimeoutsEl {
    pub fn build(self) -> RedisClusterTimeoutsEl {
        RedisClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterTimeoutsElRef {
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
#[derive(Serialize)]
pub struct RedisClusterZoneDistributionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl RedisClusterZoneDistributionConfigEl {
    #[doc = "Set the field `mode`.\nImmutable. The mode for zone distribution for Memorystore Redis cluster.\nIf not provided, MULTI_ZONE will be used as default Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nImmutable. The zone for single zone Memorystore Redis cluster."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for RedisClusterZoneDistributionConfigEl {
    type O = BlockAssignable<RedisClusterZoneDistributionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisClusterZoneDistributionConfigEl {}
impl BuildRedisClusterZoneDistributionConfigEl {
    pub fn build(self) -> RedisClusterZoneDistributionConfigEl {
        RedisClusterZoneDistributionConfigEl {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct RedisClusterZoneDistributionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisClusterZoneDistributionConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterZoneDistributionConfigElRef {
        RedisClusterZoneDistributionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisClusterZoneDistributionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nImmutable. The mode for zone distribution for Memorystore Redis cluster.\nIf not provided, MULTI_ZONE will be used as default Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nImmutable. The zone for single zone Memorystore Redis cluster."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisClusterDynamic {
    automated_backup_config: Option<DynamicBlock<RedisClusterAutomatedBackupConfigEl>>,
    cross_cluster_replication_config:
        Option<DynamicBlock<RedisClusterCrossClusterReplicationConfigEl>>,
    gcs_source: Option<DynamicBlock<RedisClusterGcsSourceEl>>,
    maintenance_policy: Option<DynamicBlock<RedisClusterMaintenancePolicyEl>>,
    managed_backup_source: Option<DynamicBlock<RedisClusterManagedBackupSourceEl>>,
    persistence_config: Option<DynamicBlock<RedisClusterPersistenceConfigEl>>,
    psc_configs: Option<DynamicBlock<RedisClusterPscConfigsEl>>,
    zone_distribution_config: Option<DynamicBlock<RedisClusterZoneDistributionConfigEl>>,
}
