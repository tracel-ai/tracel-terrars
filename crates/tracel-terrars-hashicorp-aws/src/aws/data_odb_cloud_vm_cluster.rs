use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbCloudVmClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbCloudVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbCloudVmClusterData>,
}

#[derive(Clone)]
pub struct DataOdbCloudVmCluster(Rc<DataOdbCloudVmCluster_>);

impl DataOdbCloudVmCluster {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the VM cluster."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_collection_options` after provisioning.\nThe set of diagnostic collection options enabled for the VM cluster."]
    pub fn data_collection_options(&self) -> ListRef<DataOdbCloudVmClusterDataCollectionOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TB), that's allocated for the VM cluster."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster."]
    pub fn db_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy configured for the VM cluster. NORMAL is 2-way redundancy. HIGH is 3-way redundancy."]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the VM cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nhe software version of the Oracle Grid Infrastructure (GI) for the VM cluster."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe computed hostname prefix for the VM cluster."]
    pub fn hostname_prefix_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the VM cluster."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe ExadataIormConfig cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> ListRef<DataOdbCloudVmClusterIormConfigCacheElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nIndicates whether database backups to local Exadata storage is enabled for the VM cluster."]
    pub fn is_local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_disk_group_enabled` after provisioning.\nIndicates whether the VM cluster is configured with a sparse disk group."]
    pub fn is_sparse_disk_group_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_disk_group_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the last maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model applied to the VM cluster."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe port number configured for the listener on the VM cluster."]
    pub fn listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI Resource Anchor."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe ID of the ODB network."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the VM cluster,expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe FQDN of the DNS record for the Single Client Access Name (SCAN) IP\n addresses that are associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe OCID of the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure that's running the VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nhe public key portion of one or more key pairs used for SSH access to the VM cluster."]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated to the VM cluster."]
    pub fn storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe time zone of the VM cluster."]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses that are associated with the VM cluster.\nOracle's Cluster Ready Services (CRS) creates and maintains one VIP address for\neach node in the VM cluster to enable failover. If one node fails, the VIP is\nreassigned to another active node in the cluster."]
    pub fn vip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }
}

impl Referable for DataOdbCloudVmCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbCloudVmCluster { }

impl ToListMappable for DataOdbCloudVmCluster {
    type O = ListRef<DataOdbCloudVmClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbCloudVmCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbCloudVmCluster {
    pub tf_id: String,
    #[doc = "The unique identifier of the VM cluster."]
    pub id: PrimField<String>,
}

impl BuildDataOdbCloudVmCluster {
    pub fn build(self, stack: &mut Stack) -> DataOdbCloudVmCluster {
        let out = DataOdbCloudVmCluster(Rc::new(DataOdbCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbCloudVmClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbCloudVmClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbCloudVmClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the VM cluster."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_collection_options` after provisioning.\nThe set of diagnostic collection options enabled for the VM cluster."]
    pub fn data_collection_options(&self) -> ListRef<DataOdbCloudVmClusterDataCollectionOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TB), that's allocated for the VM cluster."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster."]
    pub fn db_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy configured for the VM cluster. NORMAL is 2-way redundancy. HIGH is 3-way redundancy."]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the VM cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nhe software version of the Oracle Grid Infrastructure (GI) for the VM cluster."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe computed hostname prefix for the VM cluster."]
    pub fn hostname_prefix_computed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the VM cluster."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe ExadataIormConfig cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> ListRef<DataOdbCloudVmClusterIormConfigCacheElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nIndicates whether database backups to local Exadata storage is enabled for the VM cluster."]
    pub fn is_local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_disk_group_enabled` after provisioning.\nIndicates whether the VM cluster is configured with a sparse disk group."]
    pub fn is_sparse_disk_group_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_disk_group_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the last maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model applied to the VM cluster."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe port number configured for the listener on the VM cluster."]
    pub fn listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI Resource Anchor."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe ID of the ODB network."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the VM cluster,expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe FQDN of the DNS record for the Single Client Access Name (SCAN) IP\n addresses that are associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe OCID of the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure that's running the VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nhe public key portion of one or more key pairs used for SSH access to the VM cluster."]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated to the VM cluster."]
    pub fn storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe time zone of the VM cluster."]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses that are associated with the VM cluster.\nOracle's Cluster Ready Services (CRS) creates and maintains one VIP address for\neach node in the VM cluster to enable failover. If one node fails, the VIP is\nreassigned to another active node in the cluster."]
    pub fn vip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudVmClusterDataCollectionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_diagnostics_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_health_monitoring_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_incident_logs_enabled: Option<PrimField<bool>>,
}

impl DataOdbCloudVmClusterDataCollectionOptionsEl {
    #[doc = "Set the field `is_diagnostics_events_enabled`.\n"]
    pub fn set_is_diagnostics_events_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_diagnostics_events_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `is_health_monitoring_enabled`.\n"]
    pub fn set_is_health_monitoring_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_health_monitoring_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `is_incident_logs_enabled`.\n"]
    pub fn set_is_incident_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_incident_logs_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudVmClusterDataCollectionOptionsEl {
    type O = BlockAssignable<DataOdbCloudVmClusterDataCollectionOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmClusterDataCollectionOptionsEl {}

impl BuildDataOdbCloudVmClusterDataCollectionOptionsEl {
    pub fn build(self) -> DataOdbCloudVmClusterDataCollectionOptionsEl {
        DataOdbCloudVmClusterDataCollectionOptionsEl {
            is_diagnostics_events_enabled: core::default::Default::default(),
            is_health_monitoring_enabled: core::default::Default::default(),
            is_incident_logs_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudVmClusterDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudVmClusterDataCollectionOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudVmClusterDataCollectionOptionsElRef {
        DataOdbCloudVmClusterDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmClusterDataCollectionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_diagnostics_events_enabled` after provisioning.\n"]
    pub fn is_diagnostics_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_diagnostics_events_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `is_health_monitoring_enabled` after provisioning.\n"]
    pub fn is_health_monitoring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_health_monitoring_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\n"]
    pub fn is_incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flash_cache_limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share: Option<PrimField<f64>>,
}

impl DataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
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

impl ToListMappable for DataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
    type O = BlockAssignable<DataOdbCloudVmClusterIormConfigCacheElDbPlansEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmClusterIormConfigCacheElDbPlansEl {}

impl BuildDataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
    pub fn build(self) -> DataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
        DataOdbCloudVmClusterIormConfigCacheElDbPlansEl {
            db_name: core::default::Default::default(),
            flash_cache_limit: core::default::Default::default(),
            share: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef {
        DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc = "Get a reference to the value of field `flash_cache_limit` after provisioning.\n"]
    pub fn flash_cache_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flash_cache_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `share` after provisioning.\n"]
    pub fn share(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.share", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudVmClusterIormConfigCacheEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_plans: Option<ListField<DataOdbCloudVmClusterIormConfigCacheElDbPlansEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objective: Option<PrimField<String>>,
}

impl DataOdbCloudVmClusterIormConfigCacheEl {
    #[doc = "Set the field `db_plans`.\n"]
    pub fn set_db_plans(mut self, v: impl Into<ListField<DataOdbCloudVmClusterIormConfigCacheElDbPlansEl>>) -> Self {
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

impl ToListMappable for DataOdbCloudVmClusterIormConfigCacheEl {
    type O = BlockAssignable<DataOdbCloudVmClusterIormConfigCacheEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmClusterIormConfigCacheEl {}

impl BuildDataOdbCloudVmClusterIormConfigCacheEl {
    pub fn build(self) -> DataOdbCloudVmClusterIormConfigCacheEl {
        DataOdbCloudVmClusterIormConfigCacheEl {
            db_plans: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            objective: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudVmClusterIormConfigCacheElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudVmClusterIormConfigCacheElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudVmClusterIormConfigCacheElRef {
        DataOdbCloudVmClusterIormConfigCacheElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmClusterIormConfigCacheElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_plans` after provisioning.\n"]
    pub fn db_plans(&self) -> ListRef<DataOdbCloudVmClusterIormConfigCacheElDbPlansElRef> {
        ListRef::new(self.shared().clone(), format!("{}.db_plans", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\n"]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.base))
    }

    #[doc = "Get a reference to the value of field `objective` after provisioning.\n"]
    pub fn objective(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.objective", self.base))
    }
}
