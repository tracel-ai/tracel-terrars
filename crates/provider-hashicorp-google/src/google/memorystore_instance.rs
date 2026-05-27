use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MemorystoreInstanceData {
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
    engine_configs: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
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
    automated_backup_config: Option<Vec<MemorystoreInstanceAutomatedBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_instance_replication_config:
        Option<Vec<MemorystoreInstanceCrossInstanceReplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_auto_created_endpoints: Option<Vec<MemorystoreInstanceDesiredAutoCreatedEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_psc_auto_connections: Option<Vec<MemorystoreInstanceDesiredPscAutoConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_source: Option<Vec<MemorystoreInstanceGcsSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<MemorystoreInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_backup_source: Option<Vec<MemorystoreInstanceManagedBackupSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence_config: Option<Vec<MemorystoreInstancePersistenceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MemorystoreInstanceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_distribution_config: Option<Vec<MemorystoreInstanceZoneDistributionConfigEl>>,
    dynamic: MemorystoreInstanceDynamic,
}
struct MemorystoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MemorystoreInstanceData>,
}
#[derive(Clone)]
pub struct MemorystoreInstance(Rc<MemorystoreInstance_>);
impl MemorystoreInstance {
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
    #[doc = "Set the field `authorization_mode`.\nOptional. Immutable. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH"]
    pub fn set_authorization_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection_enabled`.\nOptional. If set to true deletion of the instance will fail."]
    pub fn set_deletion_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_configs`.\nOptional. User-provided engine configurations for the instance."]
    pub fn set_engine_configs(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().engine_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_version`.\nOptional. Engine version of the instance."]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_version`.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn set_maintenance_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type`.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn set_node_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_count`.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn set_replica_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_mode`.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn set_server_ca_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_ca_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn set_server_ca_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_ca_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `transit_encryption_mode`.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn set_transit_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `automated_backup_config`.\n"]
    pub fn set_automated_backup_config(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceAutomatedBackupConfigEl>>,
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
    #[doc = "Set the field `cross_instance_replication_config`.\n"]
    pub fn set_cross_instance_replication_config(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_instance_replication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .cross_instance_replication_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `desired_auto_created_endpoints`.\n"]
    pub fn set_desired_auto_created_endpoints(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceDesiredAutoCreatedEndpointsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_auto_created_endpoints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .desired_auto_created_endpoints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `desired_psc_auto_connections`.\n"]
    pub fn set_desired_psc_auto_connections(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceDesiredPscAutoConnectionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_psc_auto_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .desired_psc_auto_connections = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_source`.\n"]
    pub fn set_gcs_source(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceGcsSourceEl>>,
    ) -> Self {
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
        v: impl Into<BlockAssignable<MemorystoreInstanceMaintenancePolicyEl>>,
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
        v: impl Into<BlockAssignable<MemorystoreInstanceManagedBackupSourceEl>>,
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
        v: impl Into<BlockAssignable<MemorystoreInstancePersistenceConfigEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MemorystoreInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `zone_distribution_config`.\n"]
    pub fn set_zone_distribution_config(
        self,
        v: impl Into<BlockAssignable<MemorystoreInstanceZoneDistributionConfigEl>>,
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
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Immutable. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH"]
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
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
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> ListRef<MemorystoreInstanceDiscoveryEndpointsElRef> {
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
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> ListRef<MemorystoreInstanceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.engine_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<MemorystoreInstanceMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<MemorystoreInstanceManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> ListRef<MemorystoreInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> ListRef<MemorystoreInstancePscAttachmentDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_attachment_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> ListRef<MemorystoreInstancePscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> ListRef<MemorystoreInstanceStateInfoElRef> {
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
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(
        &self,
    ) -> ListRef<MemorystoreInstanceAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\n"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\n"]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredAutoCreatedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n"]
    pub fn desired_psc_auto_connections(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> ListRef<MemorystoreInstanceGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<MemorystoreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> ListRef<MemorystoreInstanceManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<MemorystoreInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceTimeoutsElRef {
        MemorystoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(
        &self,
    ) -> ListRef<MemorystoreInstanceZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
impl Referable for MemorystoreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MemorystoreInstance {}
impl ToListMappable for MemorystoreInstance {
    type O = ListRef<MemorystoreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MemorystoreInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_memorystore_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMemorystoreInstance {
    pub tf_id: String,
    #[doc = "Required. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub instance_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub location: PrimField<String>,
    #[doc = "Required. Number of shards for the instance."]
    pub shard_count: PrimField<f64>,
}
impl BuildMemorystoreInstance {
    pub fn build(self, stack: &mut Stack) -> MemorystoreInstance {
        let out = MemorystoreInstance(Rc::new(MemorystoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MemorystoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorization_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                engine_configs: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                maintenance_version: core::default::Default::default(),
                mode: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                server_ca_mode: core::default::Default::default(),
                server_ca_pool: core::default::Default::default(),
                shard_count: self.shard_count,
                transit_encryption_mode: core::default::Default::default(),
                automated_backup_config: core::default::Default::default(),
                cross_instance_replication_config: core::default::Default::default(),
                desired_auto_created_endpoints: core::default::Default::default(),
                desired_psc_auto_connections: core::default::Default::default(),
                gcs_source: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                managed_backup_source: core::default::Default::default(),
                persistence_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                zone_distribution_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MemorystoreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MemorystoreInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Immutable. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH"]
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
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
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> ListRef<MemorystoreInstanceDiscoveryEndpointsElRef> {
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
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> ListRef<MemorystoreInstanceEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.engine_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<MemorystoreInstanceMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> ListRef<MemorystoreInstanceManagedServerCaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_server_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> ListRef<MemorystoreInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> ListRef<MemorystoreInstancePscAttachmentDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_attachment_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> ListRef<MemorystoreInstancePscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_ca_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shard_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> ListRef<MemorystoreInstanceStateInfoElRef> {
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
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(
        &self,
    ) -> ListRef<MemorystoreInstanceAutomatedBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\n"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\n"]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredAutoCreatedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n"]
    pub fn desired_psc_auto_connections(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_psc_auto_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> ListRef<MemorystoreInstanceGcsSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<MemorystoreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> ListRef<MemorystoreInstanceManagedBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<MemorystoreInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceTimeoutsElRef {
        MemorystoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(
        &self,
    ) -> ListRef<MemorystoreInstanceZoneDistributionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_distribution_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl MemorystoreInstanceDiscoveryEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceDiscoveryEndpointsEl {
    type O = BlockAssignable<MemorystoreInstanceDiscoveryEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDiscoveryEndpointsEl {}
impl BuildMemorystoreInstanceDiscoveryEndpointsEl {
    pub fn build(self) -> MemorystoreInstanceDiscoveryEndpointsEl {
        MemorystoreInstanceDiscoveryEndpointsEl {
            address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceDiscoveryEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDiscoveryEndpointsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceDiscoveryEndpointsElRef {
        MemorystoreInstanceDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceDiscoveryEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
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
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    type O = BlockAssignable<MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {}
impl BuildMemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
    pub fn build(self) -> MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
        MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
        MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef {
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
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
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
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceEndpointsElConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_auto_connection:
        Option<ListField<MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>>,
}
impl MemorystoreInstanceEndpointsElConnectionsEl {
    #[doc = "Set the field `psc_auto_connection`.\n"]
    pub fn set_psc_auto_connection(
        mut self,
        v: impl Into<ListField<MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionEl>>,
    ) -> Self {
        self.psc_auto_connection = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceEndpointsElConnectionsEl {
    type O = BlockAssignable<MemorystoreInstanceEndpointsElConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceEndpointsElConnectionsEl {}
impl BuildMemorystoreInstanceEndpointsElConnectionsEl {
    pub fn build(self) -> MemorystoreInstanceEndpointsElConnectionsEl {
        MemorystoreInstanceEndpointsElConnectionsEl {
            psc_auto_connection: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceEndpointsElConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceEndpointsElConnectionsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceEndpointsElConnectionsElRef {
        MemorystoreInstanceEndpointsElConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceEndpointsElConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `psc_auto_connection` after provisioning.\n"]
    pub fn psc_auto_connection(
        &self,
    ) -> ListRef<MemorystoreInstanceEndpointsElConnectionsElPscAutoConnectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connections: Option<ListField<MemorystoreInstanceEndpointsElConnectionsEl>>,
}
impl MemorystoreInstanceEndpointsEl {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v: impl Into<ListField<MemorystoreInstanceEndpointsElConnectionsEl>>,
    ) -> Self {
        self.connections = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceEndpointsEl {
    type O = BlockAssignable<MemorystoreInstanceEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceEndpointsEl {}
impl BuildMemorystoreInstanceEndpointsEl {
    pub fn build(self) -> MemorystoreInstanceEndpointsEl {
        MemorystoreInstanceEndpointsEl {
            connections: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceEndpointsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceEndpointsElRef {
        MemorystoreInstanceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> ListRef<MemorystoreInstanceEndpointsElConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl MemorystoreInstanceMaintenanceScheduleEl {
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
impl ToListMappable for MemorystoreInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<MemorystoreInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceMaintenanceScheduleEl {}
impl BuildMemorystoreInstanceMaintenanceScheduleEl {
    pub fn build(self) -> MemorystoreInstanceMaintenanceScheduleEl {
        MemorystoreInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceMaintenanceScheduleElRef {
        MemorystoreInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceMaintenanceScheduleElRef {
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
pub struct MemorystoreInstanceManagedServerCaElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<ListField<PrimField<String>>>,
}
impl MemorystoreInstanceManagedServerCaElCaCertsEl {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.certificates = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceManagedServerCaElCaCertsEl {
    type O = BlockAssignable<MemorystoreInstanceManagedServerCaElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceManagedServerCaElCaCertsEl {}
impl BuildMemorystoreInstanceManagedServerCaElCaCertsEl {
    pub fn build(self) -> MemorystoreInstanceManagedServerCaElCaCertsEl {
        MemorystoreInstanceManagedServerCaElCaCertsEl {
            certificates: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceManagedServerCaElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceManagedServerCaElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceManagedServerCaElCaCertsElRef {
        MemorystoreInstanceManagedServerCaElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceManagedServerCaElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceManagedServerCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<MemorystoreInstanceManagedServerCaElCaCertsEl>>,
}
impl MemorystoreInstanceManagedServerCaEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<MemorystoreInstanceManagedServerCaElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceManagedServerCaEl {
    type O = BlockAssignable<MemorystoreInstanceManagedServerCaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceManagedServerCaEl {}
impl BuildMemorystoreInstanceManagedServerCaEl {
    pub fn build(self) -> MemorystoreInstanceManagedServerCaEl {
        MemorystoreInstanceManagedServerCaEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceManagedServerCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceManagedServerCaElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceManagedServerCaElRef {
        MemorystoreInstanceManagedServerCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceManagedServerCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<MemorystoreInstanceManagedServerCaElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
}
impl MemorystoreInstanceNodeConfigEl {
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceNodeConfigEl {
    type O = BlockAssignable<MemorystoreInstanceNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceNodeConfigEl {}
impl BuildMemorystoreInstanceNodeConfigEl {
    pub fn build(self) -> MemorystoreInstanceNodeConfigEl {
        MemorystoreInstanceNodeConfigEl {
            size_gb: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceNodeConfigElRef {
        MemorystoreInstanceNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstancePscAttachmentDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl MemorystoreInstancePscAttachmentDetailsEl {
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
impl ToListMappable for MemorystoreInstancePscAttachmentDetailsEl {
    type O = BlockAssignable<MemorystoreInstancePscAttachmentDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstancePscAttachmentDetailsEl {}
impl BuildMemorystoreInstancePscAttachmentDetailsEl {
    pub fn build(self) -> MemorystoreInstancePscAttachmentDetailsEl {
        MemorystoreInstancePscAttachmentDetailsEl {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstancePscAttachmentDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstancePscAttachmentDetailsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstancePscAttachmentDetailsElRef {
        MemorystoreInstancePscAttachmentDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstancePscAttachmentDetailsElRef {
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
pub struct MemorystoreInstancePscAutoConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl MemorystoreInstancePscAutoConnectionsEl {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
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
    #[doc = "Set the field `psc_connection_status`.\n"]
    pub fn set_psc_connection_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_status = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstancePscAutoConnectionsEl {
    type O = BlockAssignable<MemorystoreInstancePscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstancePscAutoConnectionsEl {}
impl BuildMemorystoreInstancePscAutoConnectionsEl {
    pub fn build(self) -> MemorystoreInstancePscAutoConnectionsEl {
        MemorystoreInstancePscAutoConnectionsEl {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            psc_connection_status: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstancePscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstancePscAutoConnectionsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstancePscAutoConnectionsElRef {
        MemorystoreInstancePscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstancePscAutoConnectionsElRef {
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
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
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
    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\n"]
    pub fn psc_connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_status", self.base),
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
pub struct MemorystoreInstanceStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_shard_count: Option<PrimField<f64>>,
}
impl MemorystoreInstanceStateInfoElUpdateInfoEl {
    #[doc = "Set the field `target_engine_version`.\n"]
    pub fn set_target_engine_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `target_node_type`.\n"]
    pub fn set_target_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_node_type = Some(v.into());
        self
    }
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
impl ToListMappable for MemorystoreInstanceStateInfoElUpdateInfoEl {
    type O = BlockAssignable<MemorystoreInstanceStateInfoElUpdateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceStateInfoElUpdateInfoEl {}
impl BuildMemorystoreInstanceStateInfoElUpdateInfoEl {
    pub fn build(self) -> MemorystoreInstanceStateInfoElUpdateInfoEl {
        MemorystoreInstanceStateInfoElUpdateInfoEl {
            target_engine_version: core::default::Default::default(),
            target_node_type: core::default::Default::default(),
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceStateInfoElUpdateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceStateInfoElUpdateInfoElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceStateInfoElUpdateInfoElRef {
        MemorystoreInstanceStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceStateInfoElUpdateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_engine_version` after provisioning.\n"]
    pub fn target_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_engine_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_node_type` after provisioning.\n"]
    pub fn target_node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_node_type", self.base),
        )
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
pub struct MemorystoreInstanceStateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update_info: Option<ListField<MemorystoreInstanceStateInfoElUpdateInfoEl>>,
}
impl MemorystoreInstanceStateInfoEl {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl Into<ListField<MemorystoreInstanceStateInfoElUpdateInfoEl>>,
    ) -> Self {
        self.update_info = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceStateInfoEl {
    type O = BlockAssignable<MemorystoreInstanceStateInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceStateInfoEl {}
impl BuildMemorystoreInstanceStateInfoEl {
    pub fn build(self) -> MemorystoreInstanceStateInfoEl {
        MemorystoreInstanceStateInfoEl {
            update_info: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceStateInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceStateInfoElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceStateInfoElRef {
        MemorystoreInstanceStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceStateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> ListRef<MemorystoreInstanceStateInfoElUpdateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    hours: PrimField<f64>,
}
impl MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {}
impl ToListMappable
    for MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl
{
    type O = BlockAssignable<
        MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    #[doc = "Hours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: PrimField<f64>,
}
impl BuildMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
        MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl {
            hours: self.hours,
        }
    }
}
pub struct MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
        MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElDynamic {
    start_time: Option<
        DynamicBlock<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<Vec<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl>>,
    dynamic: MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElDynamic,
}
impl MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<
                MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeEl,
            >,
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
impl ToListMappable for MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    type O = BlockAssignable<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {}
impl BuildMemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
    pub fn build(self) -> MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
        MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl {
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
        MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceAutomatedBackupConfigElDynamic {
    fixed_frequency_schedule:
        Option<DynamicBlock<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
}
#[derive(Serialize)]
pub struct MemorystoreInstanceAutomatedBackupConfigEl {
    retention: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_frequency_schedule:
        Option<Vec<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>>,
    dynamic: MemorystoreInstanceAutomatedBackupConfigElDynamic,
}
impl MemorystoreInstanceAutomatedBackupConfigEl {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v: impl Into<
            BlockAssignable<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleEl>,
        >,
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
impl ToListMappable for MemorystoreInstanceAutomatedBackupConfigEl {
    type O = BlockAssignable<MemorystoreInstanceAutomatedBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceAutomatedBackupConfigEl {
    #[doc = "How long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\". The default_value is \"3024000s\""]
    pub retention: PrimField<String>,
}
impl BuildMemorystoreInstanceAutomatedBackupConfigEl {
    pub fn build(self) -> MemorystoreInstanceAutomatedBackupConfigEl {
        MemorystoreInstanceAutomatedBackupConfigEl {
            retention: self.retention,
            fixed_frequency_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceAutomatedBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceAutomatedBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceAutomatedBackupConfigElRef {
        MemorystoreInstanceAutomatedBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceAutomatedBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nHow long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\". The default_value is \"3024000s\""]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> ListRef<MemorystoreInstanceAutomatedBackupConfigElFixedFrequencyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_frequency_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl
{
    type O = BlockAssignable<
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable
    for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl
{
    type O = BlockAssignable<
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl
{}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_instance: Option<
        ListField<MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_instance: Option<
        ListField<
            MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl,
        >,
    >,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v: impl Into<
            ListField<
                MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceEl,
            >,
        >,
    ) -> Self {
        self.primary_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_instance`.\n"]
    pub fn set_secondary_instance(
        mut self,
        v: impl Into<
            ListField<
                MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceEl,
            >,
        >,
    ) -> Self {
        self.secondary_instance = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    type O = BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
    pub fn build(self) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipEl {
            primary_instance: core::default::Default::default(),
            secondary_instance: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElPrimaryInstanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_instance` after provisioning.\n"]
    pub fn secondary_instance(
        &self,
    ) -> ListRef<
        MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElSecondaryInstanceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    #[doc = "Set the field `instance`.\nThe full resource path of the primary instance in the format: projects/{project}/locations/{region}/instances/{instance-id}"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    type O = BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
    pub fn build(self) -> MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
        MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl {
            instance: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe full resource path of the primary instance in the format: projects/{project}/locations/{region}/instances/{instance-id}"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the primary instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    #[doc = "Set the field `instance`.\nThe full resource path of the Nth instance in the format: projects/{project}/locations/{region}/instance/{instance-id}"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    type O =
        BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
    pub fn build(self) -> MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
        MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl {
            instance: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe full resource path of the Nth instance in the format: projects/{project}/locations/{region}/instance/{instance-id}"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the Nth instance."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceCrossInstanceReplicationConfigElDynamic {
    primary_instance:
        Option<DynamicBlock<MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>>,
    secondary_instances: Option<
        DynamicBlock<MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl>,
    >,
}
#[derive(Serialize)]
pub struct MemorystoreInstanceCrossInstanceReplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_instance:
        Option<Vec<MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_instances:
        Option<Vec<MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl>>,
    dynamic: MemorystoreInstanceCrossInstanceReplicationConfigElDynamic,
}
impl MemorystoreInstanceCrossInstanceReplicationConfigEl {
    #[doc = "Set the field `instance_role`.\nThe instance role supports the following values:\n1. 'INSTANCE_ROLE_UNSPECIFIED': This is an independent instance that has never participated in cross instance replication. It allows both reads and writes.\n2. 'NONE': This is an independent instance that previously participated in cross instance replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n3. 'PRIMARY': This instance serves as the replication source for secondary instance that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n4. 'SECONDARY': This instance replicates data from the primary instance. It allows only reads. Possible values: [\"INSTANCE_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn set_instance_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_role = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v: impl Into<
            BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_instances`.\n"]
    pub fn set_secondary_instances(
        mut self,
        v: impl Into<
            BlockAssignable<
                MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secondary_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_instances = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MemorystoreInstanceCrossInstanceReplicationConfigEl {
    type O = BlockAssignable<MemorystoreInstanceCrossInstanceReplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceCrossInstanceReplicationConfigEl {}
impl BuildMemorystoreInstanceCrossInstanceReplicationConfigEl {
    pub fn build(self) -> MemorystoreInstanceCrossInstanceReplicationConfigEl {
        MemorystoreInstanceCrossInstanceReplicationConfigEl {
            instance_role: core::default::Default::default(),
            primary_instance: core::default::Default::default(),
            secondary_instances: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceCrossInstanceReplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceCrossInstanceReplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceCrossInstanceReplicationConfigElRef {
        MemorystoreInstanceCrossInstanceReplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceCrossInstanceReplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\nThe instance role supports the following values:\n1. 'INSTANCE_ROLE_UNSPECIFIED': This is an independent instance that has never participated in cross instance replication. It allows both reads and writes.\n2. 'NONE': This is an independent instance that previously participated in cross instance replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n3. 'PRIMARY': This instance serves as the replication source for secondary instance that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n4. 'SECONDARY': This instance replicates data from the primary instance. It allows only reads. Possible values: [\"INSTANCE_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn instance_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nAn output only view of all the member instance participating in cross instance replication. This field is populated for all the member clusters irrespective of their cluster role."]
    pub fn membership(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElMembershipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last time cross instance replication config was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElPrimaryInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_instances` after provisioning.\n"]
    pub fn secondary_instances(
        &self,
    ) -> ListRef<MemorystoreInstanceCrossInstanceReplicationConfigElSecondaryInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_instances", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    network: PrimField<String>,
    project_id: PrimField<String>,
}
impl MemorystoreInstanceDesiredAutoCreatedEndpointsEl {}
impl ToListMappable for MemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    type O = BlockAssignable<MemorystoreInstanceDesiredAutoCreatedEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    #[doc = "Required. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: PrimField<String>,
    #[doc = "Required. The consumer project_id where the forwarding rule is created from."]
    pub project_id: PrimField<String>,
}
impl BuildMemorystoreInstanceDesiredAutoCreatedEndpointsEl {
    pub fn build(self) -> MemorystoreInstanceDesiredAutoCreatedEndpointsEl {
        MemorystoreInstanceDesiredAutoCreatedEndpointsEl {
            network: self.network,
            project_id: self.project_id,
        }
    }
}
pub struct MemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
        MemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceDesiredAutoCreatedEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nRequired. The consumer project_id where the forwarding rule is created from."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredPscAutoConnectionsEl {
    network: PrimField<String>,
    project_id: PrimField<String>,
}
impl MemorystoreInstanceDesiredPscAutoConnectionsEl {}
impl ToListMappable for MemorystoreInstanceDesiredPscAutoConnectionsEl {
    type O = BlockAssignable<MemorystoreInstanceDesiredPscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDesiredPscAutoConnectionsEl {
    #[doc = "Required. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: PrimField<String>,
    #[doc = "Required. The consumer project_id where the forwarding rule is created from."]
    pub project_id: PrimField<String>,
}
impl BuildMemorystoreInstanceDesiredPscAutoConnectionsEl {
    pub fn build(self) -> MemorystoreInstanceDesiredPscAutoConnectionsEl {
        MemorystoreInstanceDesiredPscAutoConnectionsEl {
            network: self.network,
            project_id: self.project_id,
        }
    }
}
pub struct MemorystoreInstanceDesiredPscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredPscAutoConnectionsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceDesiredPscAutoConnectionsElRef {
        MemorystoreInstanceDesiredPscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceDesiredPscAutoConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nRequired. The consumer project_id where the forwarding rule is created from."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceGcsSourceEl {
    uris: SetField<PrimField<String>>,
}
impl MemorystoreInstanceGcsSourceEl {}
impl ToListMappable for MemorystoreInstanceGcsSourceEl {
    type O = BlockAssignable<MemorystoreInstanceGcsSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceGcsSourceEl {
    #[doc = "URIs of the GCS objects to import.\nExample: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub uris: SetField<PrimField<String>>,
}
impl BuildMemorystoreInstanceGcsSourceEl {
    pub fn build(self) -> MemorystoreInstanceGcsSourceEl {
        MemorystoreInstanceGcsSourceEl { uris: self.uris }
    }
}
pub struct MemorystoreInstanceGcsSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceGcsSourceElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceGcsSourceElRef {
        MemorystoreInstanceGcsSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceGcsSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs of the GCS objects to import.\nExample: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub fn uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
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
impl ToListMappable for MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O =
        BlockAssignable<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
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
struct MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic {
    start_time: Option<
        DynamicBlock<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    day: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<Vec<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    dynamic: MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic,
}
impl MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<
                MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl,
            >,
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
impl ToListMappable for MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: PrimField<String>,
}
impl BuildMemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nThe day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration of the maintenance window.\nThe current window is fixed at 1 hour.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceMaintenancePolicyElDynamic {
    weekly_maintenance_window:
        Option<DynamicBlock<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
#[derive(Serialize)]
pub struct MemorystoreInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<Vec<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    dynamic: MemorystoreInstanceMaintenancePolicyElDynamic,
}
impl MemorystoreInstanceMaintenancePolicyEl {
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
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
impl ToListMappable for MemorystoreInstanceMaintenancePolicyEl {
    type O = BlockAssignable<MemorystoreInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceMaintenancePolicyEl {}
impl BuildMemorystoreInstanceMaintenancePolicyEl {
    pub fn build(self) -> MemorystoreInstanceMaintenancePolicyEl {
        MemorystoreInstanceMaintenancePolicyEl {
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceMaintenancePolicyElRef {
        MemorystoreInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the policy was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<MemorystoreInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceManagedBackupSourceEl {
    backup: PrimField<String>,
}
impl MemorystoreInstanceManagedBackupSourceEl {}
impl ToListMappable for MemorystoreInstanceManagedBackupSourceEl {
    type O = BlockAssignable<MemorystoreInstanceManagedBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceManagedBackupSourceEl {
    #[doc = "Example: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub backup: PrimField<String>,
}
impl BuildMemorystoreInstanceManagedBackupSourceEl {
    pub fn build(self) -> MemorystoreInstanceManagedBackupSourceEl {
        MemorystoreInstanceManagedBackupSourceEl {
            backup: self.backup,
        }
    }
}
pub struct MemorystoreInstanceManagedBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceManagedBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceManagedBackupSourceElRef {
        MemorystoreInstanceManagedBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceManagedBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\nExample: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstancePersistenceConfigElAofConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    append_fsync: Option<PrimField<String>>,
}
impl MemorystoreInstancePersistenceConfigElAofConfigEl {
    #[doc = "Set the field `append_fsync`.\nOptional. The fsync mode. \n Possible values:\n NEVER\nEVERY_SEC\nALWAYS"]
    pub fn set_append_fsync(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.append_fsync = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstancePersistenceConfigElAofConfigEl {
    type O = BlockAssignable<MemorystoreInstancePersistenceConfigElAofConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstancePersistenceConfigElAofConfigEl {}
impl BuildMemorystoreInstancePersistenceConfigElAofConfigEl {
    pub fn build(self) -> MemorystoreInstancePersistenceConfigElAofConfigEl {
        MemorystoreInstancePersistenceConfigElAofConfigEl {
            append_fsync: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstancePersistenceConfigElAofConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstancePersistenceConfigElAofConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstancePersistenceConfigElAofConfigElRef {
        MemorystoreInstancePersistenceConfigElAofConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstancePersistenceConfigElAofConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_fsync` after provisioning.\nOptional. The fsync mode. \n Possible values:\n NEVER\nEVERY_SEC\nALWAYS"]
    pub fn append_fsync(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstancePersistenceConfigElRdbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl MemorystoreInstancePersistenceConfigElRdbConfigEl {
    #[doc = "Set the field `rdb_snapshot_period`.\nOptional. Period between RDB snapshots. \n Possible values:\n ONE_HOUR\nSIX_HOURS\nTWELVE_HOURS\nTWENTY_FOUR_HOURS"]
    pub fn set_rdb_snapshot_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_period = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_start_time`.\nOptional. Time that the first snapshot was/will be attempted, and to which future\nsnapshots will be aligned. If not provided, the current time will be\nused."]
    pub fn set_rdb_snapshot_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstancePersistenceConfigElRdbConfigEl {
    type O = BlockAssignable<MemorystoreInstancePersistenceConfigElRdbConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstancePersistenceConfigElRdbConfigEl {}
impl BuildMemorystoreInstancePersistenceConfigElRdbConfigEl {
    pub fn build(self) -> MemorystoreInstancePersistenceConfigElRdbConfigEl {
        MemorystoreInstancePersistenceConfigElRdbConfigEl {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstancePersistenceConfigElRdbConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstancePersistenceConfigElRdbConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstancePersistenceConfigElRdbConfigElRef {
        MemorystoreInstancePersistenceConfigElRdbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstancePersistenceConfigElRdbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\nOptional. Period between RDB snapshots. \n Possible values:\n ONE_HOUR\nSIX_HOURS\nTWELVE_HOURS\nTWENTY_FOUR_HOURS"]
    pub fn rdb_snapshot_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\nOptional. Time that the first snapshot was/will be attempted, and to which future\nsnapshots will be aligned. If not provided, the current time will be\nused."]
    pub fn rdb_snapshot_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_start_time", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstancePersistenceConfigElDynamic {
    aof_config: Option<DynamicBlock<MemorystoreInstancePersistenceConfigElAofConfigEl>>,
    rdb_config: Option<DynamicBlock<MemorystoreInstancePersistenceConfigElRdbConfigEl>>,
}
#[derive(Serialize)]
pub struct MemorystoreInstancePersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aof_config: Option<Vec<MemorystoreInstancePersistenceConfigElAofConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_config: Option<Vec<MemorystoreInstancePersistenceConfigElRdbConfigEl>>,
    dynamic: MemorystoreInstancePersistenceConfigElDynamic,
}
impl MemorystoreInstancePersistenceConfigEl {
    #[doc = "Set the field `mode`.\nOptional. Current persistence mode. \n Possible values:\nDISABLED\nRDB\nAOF Possible values: [\"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl Into<BlockAssignable<MemorystoreInstancePersistenceConfigElAofConfigEl>>,
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
        v: impl Into<BlockAssignable<MemorystoreInstancePersistenceConfigElRdbConfigEl>>,
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
impl ToListMappable for MemorystoreInstancePersistenceConfigEl {
    type O = BlockAssignable<MemorystoreInstancePersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstancePersistenceConfigEl {}
impl BuildMemorystoreInstancePersistenceConfigEl {
    pub fn build(self) -> MemorystoreInstancePersistenceConfigEl {
        MemorystoreInstancePersistenceConfigEl {
            mode: core::default::Default::default(),
            aof_config: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstancePersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstancePersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstancePersistenceConfigElRef {
        MemorystoreInstancePersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstancePersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. Current persistence mode. \n Possible values:\nDISABLED\nRDB\nAOF Possible values: [\"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> ListRef<MemorystoreInstancePersistenceConfigElAofConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }
    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> ListRef<MemorystoreInstancePersistenceConfigElRdbConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MemorystoreInstanceTimeoutsEl {
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
impl ToListMappable for MemorystoreInstanceTimeoutsEl {
    type O = BlockAssignable<MemorystoreInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceTimeoutsEl {}
impl BuildMemorystoreInstanceTimeoutsEl {
    pub fn build(self) -> MemorystoreInstanceTimeoutsEl {
        MemorystoreInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceTimeoutsElRef {
        MemorystoreInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceTimeoutsElRef {
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
pub struct MemorystoreInstanceZoneDistributionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl MemorystoreInstanceZoneDistributionConfigEl {
    #[doc = "Set the field `mode`.\nOptional. Current zone distribution mode. Defaults to MULTI_ZONE. \n Possible values:\n MULTI_ZONE\nSINGLE_ZONE Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nOptional. Defines zone where all resources will be allocated with SINGLE_ZONE mode.\nIgnored for MULTI_ZONE mode."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for MemorystoreInstanceZoneDistributionConfigEl {
    type O = BlockAssignable<MemorystoreInstanceZoneDistributionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceZoneDistributionConfigEl {}
impl BuildMemorystoreInstanceZoneDistributionConfigEl {
    pub fn build(self) -> MemorystoreInstanceZoneDistributionConfigEl {
        MemorystoreInstanceZoneDistributionConfigEl {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceZoneDistributionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceZoneDistributionConfigElRef {
    fn new(shared: StackShared, base: String) -> MemorystoreInstanceZoneDistributionConfigElRef {
        MemorystoreInstanceZoneDistributionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceZoneDistributionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. Current zone distribution mode. Defaults to MULTI_ZONE. \n Possible values:\n MULTI_ZONE\nSINGLE_ZONE Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nOptional. Defines zone where all resources will be allocated with SINGLE_ZONE mode.\nIgnored for MULTI_ZONE mode."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceDynamic {
    automated_backup_config: Option<DynamicBlock<MemorystoreInstanceAutomatedBackupConfigEl>>,
    cross_instance_replication_config:
        Option<DynamicBlock<MemorystoreInstanceCrossInstanceReplicationConfigEl>>,
    desired_auto_created_endpoints:
        Option<DynamicBlock<MemorystoreInstanceDesiredAutoCreatedEndpointsEl>>,
    desired_psc_auto_connections:
        Option<DynamicBlock<MemorystoreInstanceDesiredPscAutoConnectionsEl>>,
    gcs_source: Option<DynamicBlock<MemorystoreInstanceGcsSourceEl>>,
    maintenance_policy: Option<DynamicBlock<MemorystoreInstanceMaintenancePolicyEl>>,
    managed_backup_source: Option<DynamicBlock<MemorystoreInstanceManagedBackupSourceEl>>,
    persistence_config: Option<DynamicBlock<MemorystoreInstancePersistenceConfigEl>>,
    zone_distribution_config: Option<DynamicBlock<MemorystoreInstanceZoneDistributionConfigEl>>,
}
