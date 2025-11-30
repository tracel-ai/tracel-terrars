use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct OdbCloudVmClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_exadata_infrastructure_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_exadata_infrastructure_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_name: Option<PrimField<String>>,
    cpu_core_count: PrimField<f64>,
    data_storage_size_in_tbs: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_in_gbs: Option<PrimField<f64>>,
    db_servers: SetField<PrimField<String>>,
    display_name: PrimField<String>,
    gi_version: PrimField<String>,
    hostname_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_local_backup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_sparse_diskgroup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_in_gbs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_listener_port_tcp: Option<PrimField<f64>>,
    ssh_public_keys: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_collection_options: Option<Vec<OdbCloudVmClusterDataCollectionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OdbCloudVmClusterTimeoutsEl>,
    dynamic: OdbCloudVmClusterDynamic,
}

struct OdbCloudVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OdbCloudVmClusterData>,
}

#[derive(Clone)]
pub struct OdbCloudVmCluster(Rc<OdbCloudVmCluster_>);

impl OdbCloudVmCluster {
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

    #[doc = "Set the field `cloud_exadata_infrastructure_arn`.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn set_cloud_exadata_infrastructure_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_exadata_infrastructure_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_exadata_infrastructure_id`.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn set_cloud_exadata_infrastructure_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_exadata_infrastructure_id = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_name`.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn set_cluster_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_name = Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_in_gbs`.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn set_db_node_storage_size_in_gbs(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().db_node_storage_size_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `is_local_backup_enabled`.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn set_is_local_backup_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_local_backup_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `is_sparse_diskgroup_enabled`.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn set_is_sparse_diskgroup_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_sparse_diskgroup_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `license_model`.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn set_license_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().license_model = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_in_gbs`.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn set_memory_size_in_gbs(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_size_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `odb_network_arn`.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn set_odb_network_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().odb_network_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `odb_network_id`.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn set_odb_network_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().odb_network_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp`.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn set_scan_listener_port_tcp(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scan_listener_port_tcp = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn set_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `data_collection_options`.\n"]
    pub fn set_data_collection_options(
        self,
        v: impl Into<BlockAssignable<OdbCloudVmClusterDataCollectionOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_collection_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_collection_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OdbCloudVmClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model used when the instance is created or cloned — either ECPU or OCPU. ECPU is a virtualized compute unit; OCPU is a physical processor core with hyper-threading."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster. Changing this will create a new resource."]
    pub fn db_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy for the VM cluster: NORMAL (2-way) or HIGH (3-way)."]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_redundancy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name associated with the VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\nA valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gi_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gi_version_computed` after provisioning.\nA complete software version of Oracle Grid Infrastructure (GI)."]
    pub fn gi_version_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gi_version_computed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\nThe host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe host name for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix_computed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe Exadata IORM (I/O Resource Manager) configuration cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> ListRef<OdbCloudVmClusterIormConfigCacheElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iorm_config_cache", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn is_local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_local_backup_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_sparse_diskgroup_enabled` after provisioning.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn is_sparse_diskgroup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_sparse_diskgroup_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe OCID of the most recent maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_history_entry_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `listener_port` after provisioning.\nThe listener port number configured on the VM cluster."]
    pub fn listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe total number of nodes in the VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with the VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster resource in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID (Oracle Cloud Identifier) of the VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe percentage of progress made on the current operation for the VM cluster."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.percent_progress", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe fully qualified domain name (FQDN) for the SCAN IP addresses associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IPs linked to the VM cluster."]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe list of OCIDs for SCAN IP addresses associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scan_ip_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn scan_listener_port_tcp(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure running the VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn ssh_public_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current lifecycle status of the VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information regarding the current status of the VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the VM cluster, in gigabytes (GB)."]
    pub fn storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_version", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timezone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses assigned to the VM cluster. CRS assigns one VIP per node for failover support."]
    pub fn vip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vip_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(&self) -> ListRef<OdbCloudVmClusterDataCollectionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_collection_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudVmClusterTimeoutsElRef {
        OdbCloudVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for OdbCloudVmCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for OdbCloudVmCluster {}

impl ToListMappable for OdbCloudVmCluster {
    type O = ListRef<OdbCloudVmClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OdbCloudVmCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_odb_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbCloudVmCluster {
    pub tf_id: String,
    #[doc = "The number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub cpu_core_count: PrimField<f64>,
    #[doc = "The size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub data_storage_size_in_tbs: PrimField<f64>,
    #[doc = "The list of database servers for the VM cluster. Changing this will create a new resource."]
    pub db_servers: SetField<PrimField<String>>,
    #[doc = "A user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub display_name: PrimField<String>,
    #[doc = "A valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub gi_version: PrimField<String>,
    #[doc = "The host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub hostname_prefix: PrimField<String>,
    #[doc = "The public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub ssh_public_keys: SetField<PrimField<String>>,
}

impl BuildOdbCloudVmCluster {
    pub fn build(self, stack: &mut Stack) -> OdbCloudVmCluster {
        let out = OdbCloudVmCluster(Rc::new(OdbCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OdbCloudVmClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cloud_exadata_infrastructure_arn: core::default::Default::default(),
                cloud_exadata_infrastructure_id: core::default::Default::default(),
                cluster_name: core::default::Default::default(),
                cpu_core_count: self.cpu_core_count,
                data_storage_size_in_tbs: self.data_storage_size_in_tbs,
                db_node_storage_size_in_gbs: core::default::Default::default(),
                db_servers: self.db_servers,
                display_name: self.display_name,
                gi_version: self.gi_version,
                hostname_prefix: self.hostname_prefix,
                is_local_backup_enabled: core::default::Default::default(),
                is_sparse_diskgroup_enabled: core::default::Default::default(),
                license_model: core::default::Default::default(),
                memory_size_in_gbs: core::default::Default::default(),
                odb_network_arn: core::default::Default::default(),
                odb_network_id: core::default::Default::default(),
                region: core::default::Default::default(),
                scan_listener_port_tcp: core::default::Default::default(),
                ssh_public_keys: self.ssh_public_keys,
                tags: core::default::Default::default(),
                timezone: core::default::Default::default(),
                data_collection_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbCloudVmClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbCloudVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl OdbCloudVmClusterRef {
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

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model used when the instance is created or cloned — either ECPU or OCPU. ECPU is a virtualized compute unit; OCPU is a physical processor core with hyper-threading."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster. Changing this will create a new resource."]
    pub fn db_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy for the VM cluster: NORMAL (2-way) or HIGH (3-way)."]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_redundancy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name associated with the VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\nA valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gi_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gi_version_computed` after provisioning.\nA complete software version of Oracle Grid Infrastructure (GI)."]
    pub fn gi_version_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gi_version_computed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\nThe host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe host name for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix_computed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe Exadata IORM (I/O Resource Manager) configuration cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> ListRef<OdbCloudVmClusterIormConfigCacheElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iorm_config_cache", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn is_local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_local_backup_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_sparse_diskgroup_enabled` after provisioning.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn is_sparse_diskgroup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_sparse_diskgroup_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe OCID of the most recent maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_history_entry_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `listener_port` after provisioning.\nThe listener port number configured on the VM cluster."]
    pub fn listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe total number of nodes in the VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with the VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster resource in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID (Oracle Cloud Identifier) of the VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe percentage of progress made on the current operation for the VM cluster."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.percent_progress", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe fully qualified domain name (FQDN) for the SCAN IP addresses associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IPs linked to the VM cluster."]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe list of OCIDs for SCAN IP addresses associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scan_ip_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn scan_listener_port_tcp(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure running the VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn ssh_public_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current lifecycle status of the VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information regarding the current status of the VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the VM cluster, in gigabytes (GB)."]
    pub fn storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_version", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timezone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses assigned to the VM cluster. CRS assigns one VIP per node for failover support."]
    pub fn vip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vip_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(&self) -> ListRef<OdbCloudVmClusterDataCollectionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_collection_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudVmClusterTimeoutsElRef {
        OdbCloudVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmClusterIormConfigCacheElDbPlansEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flash_cache_limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share: Option<PrimField<f64>>,
}

impl OdbCloudVmClusterIormConfigCacheElDbPlansEl {
    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_name = Some(v.into());
        self
    }

    #[doc = "Set the field `flash_cache_limit`.\n"]
    pub fn set_flash_cache_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flash_cache_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `share`.\n"]
    pub fn set_share(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.share = Some(v.into());
        self
    }
}

impl ToListMappable for OdbCloudVmClusterIormConfigCacheElDbPlansEl {
    type O = BlockAssignable<OdbCloudVmClusterIormConfigCacheElDbPlansEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmClusterIormConfigCacheElDbPlansEl {}

impl BuildOdbCloudVmClusterIormConfigCacheElDbPlansEl {
    pub fn build(self) -> OdbCloudVmClusterIormConfigCacheElDbPlansEl {
        OdbCloudVmClusterIormConfigCacheElDbPlansEl {
            db_name: core::default::Default::default(),
            flash_cache_limit: core::default::Default::default(),
            share: core::default::Default::default(),
        }
    }
}

pub struct OdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    fn new(shared: StackShared, base: String) -> OdbCloudVmClusterIormConfigCacheElDbPlansElRef {
        OdbCloudVmClusterIormConfigCacheElDbPlansElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc = "Get a reference to the value of field `flash_cache_limit` after provisioning.\n"]
    pub fn flash_cache_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flash_cache_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `share` after provisioning.\n"]
    pub fn share(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.share", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmClusterIormConfigCacheEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_plans: Option<ListField<OdbCloudVmClusterIormConfigCacheElDbPlansEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objective: Option<PrimField<String>>,
}

impl OdbCloudVmClusterIormConfigCacheEl {
    #[doc = "Set the field `db_plans`.\n"]
    pub fn set_db_plans(
        mut self,
        v: impl Into<ListField<OdbCloudVmClusterIormConfigCacheElDbPlansEl>>,
    ) -> Self {
        self.db_plans = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_details`.\n"]
    pub fn set_lifecycle_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_details = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_state`.\n"]
    pub fn set_lifecycle_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_state = Some(v.into());
        self
    }

    #[doc = "Set the field `objective`.\n"]
    pub fn set_objective(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.objective = Some(v.into());
        self
    }
}

impl ToListMappable for OdbCloudVmClusterIormConfigCacheEl {
    type O = BlockAssignable<OdbCloudVmClusterIormConfigCacheEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmClusterIormConfigCacheEl {}

impl BuildOdbCloudVmClusterIormConfigCacheEl {
    pub fn build(self) -> OdbCloudVmClusterIormConfigCacheEl {
        OdbCloudVmClusterIormConfigCacheEl {
            db_plans: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            objective: core::default::Default::default(),
        }
    }
}

pub struct OdbCloudVmClusterIormConfigCacheElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbCloudVmClusterIormConfigCacheElRef {
    fn new(shared: StackShared, base: String) -> OdbCloudVmClusterIormConfigCacheElRef {
        OdbCloudVmClusterIormConfigCacheElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmClusterIormConfigCacheElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_plans` after provisioning.\n"]
    pub fn db_plans(&self) -> ListRef<OdbCloudVmClusterIormConfigCacheElDbPlansElRef> {
        ListRef::new(self.shared().clone(), format!("{}.db_plans", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\n"]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_state", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `objective` after provisioning.\n"]
    pub fn objective(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.objective", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmClusterDataCollectionOptionsEl {
    is_diagnostics_events_enabled: PrimField<bool>,
    is_health_monitoring_enabled: PrimField<bool>,
    is_incident_logs_enabled: PrimField<bool>,
}

impl OdbCloudVmClusterDataCollectionOptionsEl {}

impl ToListMappable for OdbCloudVmClusterDataCollectionOptionsEl {
    type O = BlockAssignable<OdbCloudVmClusterDataCollectionOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmClusterDataCollectionOptionsEl {
    #[doc = ""]
    pub is_diagnostics_events_enabled: PrimField<bool>,
    #[doc = ""]
    pub is_health_monitoring_enabled: PrimField<bool>,
    #[doc = ""]
    pub is_incident_logs_enabled: PrimField<bool>,
}

impl BuildOdbCloudVmClusterDataCollectionOptionsEl {
    pub fn build(self) -> OdbCloudVmClusterDataCollectionOptionsEl {
        OdbCloudVmClusterDataCollectionOptionsEl {
            is_diagnostics_events_enabled: self.is_diagnostics_events_enabled,
            is_health_monitoring_enabled: self.is_health_monitoring_enabled,
            is_incident_logs_enabled: self.is_incident_logs_enabled,
        }
    }
}

pub struct OdbCloudVmClusterDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbCloudVmClusterDataCollectionOptionsElRef {
    fn new(shared: StackShared, base: String) -> OdbCloudVmClusterDataCollectionOptionsElRef {
        OdbCloudVmClusterDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmClusterDataCollectionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_diagnostics_events_enabled` after provisioning.\n"]
    pub fn is_diagnostics_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_diagnostics_events_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `is_health_monitoring_enabled` after provisioning.\n"]
    pub fn is_health_monitoring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_health_monitoring_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\n"]
    pub fn is_incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_incident_logs_enabled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OdbCloudVmClusterTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for OdbCloudVmClusterTimeoutsEl {
    type O = BlockAssignable<OdbCloudVmClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmClusterTimeoutsEl {}

impl BuildOdbCloudVmClusterTimeoutsEl {
    pub fn build(self) -> OdbCloudVmClusterTimeoutsEl {
        OdbCloudVmClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OdbCloudVmClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbCloudVmClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OdbCloudVmClusterTimeoutsElRef {
        OdbCloudVmClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct OdbCloudVmClusterDynamic {
    data_collection_options: Option<DynamicBlock<OdbCloudVmClusterDataCollectionOptionsEl>>,
}
