use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbCloudAutonomousVmClusterData {
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

struct DataOdbCloudAutonomousVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbCloudAutonomousVmClusterData>,
}

#[derive(Clone)]
pub struct DataOdbCloudAutonomousVmCluster(Rc<DataOdbCloudAutonomousVmCluster_>);

impl DataOdbCloudAutonomousVmCluster {
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
        "Get a reference to the value of field `autonomous_data_storage_percentage` after provisioning.\nThe percentage of data storage currently in use for Autonomous Databases in the Autonomous VM cluster."]
    pub fn autonomous_data_storage_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.autonomous_data_storage_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_data_storage_size_in_tbs` after provisioning.\nThe data storage size allocated for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn autonomous_data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.autonomous_data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_autonomous_data_storage_size_in_tbs` after provisioning.\nThe available data storage space for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn available_autonomous_data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_container_databases` after provisioning.\nThe number of Autonomous CDBs that you can create with the currently available storage."]
    pub fn available_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_container_databases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_cpus` after provisioning.\nThe number of CPU cores available for allocation to Autonomous Databases."]
    pub fn available_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nCloud exadata infrastructure id associated with this cloud autonomous VM cluster."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the Autonomous VM cluster: ECPU or OCPU."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe total number of CPU cores in the Autonomous VM cluster."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count_per_node` after provisioning.\nThe number of CPU cores enabled per node in the Autonomous VM cluster."]
    pub fn cpu_core_count_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_percentage` after provisioning.\nhe percentage of total CPU cores currently in use in the Autonomous VM cluster."]
    pub fn cpu_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Autonomous VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in GB."]
    pub fn data_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in TB."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers associated with the Autonomous VM cluster."]
    pub fn db_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the Autonomous VM cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Autonomous VM cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the Autonomous VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_storage_in_tbs_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the Exadata storage, in TB."]
    pub fn exadata_storage_in_tbs_lowest_scaled_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_storage_in_tbs_lowest_scaled_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the Autonomous VM cluster."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique ID of the Autonomous VM cluster."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_enabled_vm_cluster` after provisioning.\n Indicates whether mutual TLS (mTLS) authentication is enabled for the Autonomous VM cluster."]
    pub fn is_mtls_enabled_vm_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_enabled_vm_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model that applies to the Autonomous VM cluster. Valid values are LICENSE_INCLUDED or BRING_YOUR_OWN_LICENSE ."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_window` after provisioning.\nThe maintenance window for the Autonomous VM cluster."]
    pub fn maintenance_window(&self) -> ListRef<DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_acds_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the maximum number of Autonomous CDBs."]
    pub fn max_acds_lowest_scaled_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_acds_lowest_scaled_value", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_per_oracle_compute_unit_in_gbs` after provisioning.\nThe amount of memory allocated per Oracle Compute Unit, in GB."]
    pub fn memory_per_oracle_compute_unit_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_per_oracle_compute_unit_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe total amount of memory allocated to the Autonomous VM cluster, in gigabytes (GB)."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of database server nodes in the Autonomous VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `non_provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can't be provisioned because of resource  constraints."]
    pub fn non_provisionable_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with this Autonomous VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe URL for accessing the OCI console page for this Autonomous VM cluster."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the Autonomous VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network associated with this Autonomous VM cluster."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_node_storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the Autonomous VM cluster, in gigabytes (GB)."]
    pub fn odb_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can be provisioned in the Autonomous VM cluster."]
    pub fn provisionable_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_cpus` after provisioning.\nThe number of CPU cores currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reclaimable_cpus` after provisioning.\nThe number of CPU cores that can be reclaimed from terminated or scaled-down Autonomous Databases."]
    pub fn reclaimable_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.reclaimable_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_cpus` after provisioning.\nThe number of CPU cores reserved for system operations and redundancy."]
    pub fn reserved_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.reserved_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_non_tls` after provisioning.\nThe SCAN listener port for non-TLS (TCP) protocol. The default is 1521."]
    pub fn scan_listener_port_non_tls(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_non_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tls` after provisioning.\nThe SCAN listener port for TLS (TCP) protocol. The default is 2484."]
    pub fn scan_listener_port_tls(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the Exadata infrastructure for the Autonomous VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the Autonomous VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the Autonomous VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_database_ssl_certificate_expires` after provisioning.\nThe expiration date and time of the database SSL certificate."]
    pub fn time_database_ssl_certificate_expires(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_database_ssl_certificate_expires", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_ords_certificate_expires` after provisioning.\nThe expiration date and time of the Oracle REST Data Services (ORDS)certificate ."]
    pub fn time_ords_certificate_expires(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_ords_certificate_expires", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the Autonomous VM cluster."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_container_databases` after provisioning.\nThe total number of Autonomous Container Databases that can be created with the allocated local storage."]
    pub fn total_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_container_databases", self.extract_ref()))
    }
}

impl Referable for DataOdbCloudAutonomousVmCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbCloudAutonomousVmCluster { }

impl ToListMappable for DataOdbCloudAutonomousVmCluster {
    type O = ListRef<DataOdbCloudAutonomousVmClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbCloudAutonomousVmCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_cloud_autonomous_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbCloudAutonomousVmCluster {
    pub tf_id: String,
    #[doc = "Unique ID of the Autonomous VM cluster."]
    pub id: PrimField<String>,
}

impl BuildDataOdbCloudAutonomousVmCluster {
    pub fn build(self, stack: &mut Stack) -> DataOdbCloudAutonomousVmCluster {
        let out = DataOdbCloudAutonomousVmCluster(Rc::new(DataOdbCloudAutonomousVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbCloudAutonomousVmClusterData {
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

pub struct DataOdbCloudAutonomousVmClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbCloudAutonomousVmClusterRef {
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
        "Get a reference to the value of field `autonomous_data_storage_percentage` after provisioning.\nThe percentage of data storage currently in use for Autonomous Databases in the Autonomous VM cluster."]
    pub fn autonomous_data_storage_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.autonomous_data_storage_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_data_storage_size_in_tbs` after provisioning.\nThe data storage size allocated for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn autonomous_data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.autonomous_data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_autonomous_data_storage_size_in_tbs` after provisioning.\nThe available data storage space for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn available_autonomous_data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_container_databases` after provisioning.\nThe number of Autonomous CDBs that you can create with the currently available storage."]
    pub fn available_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_container_databases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_cpus` after provisioning.\nThe number of CPU cores available for allocation to Autonomous Databases."]
    pub fn available_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nCloud exadata infrastructure id associated with this cloud autonomous VM cluster."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the Autonomous VM cluster: ECPU or OCPU."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe total number of CPU cores in the Autonomous VM cluster."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count_per_node` after provisioning.\nThe number of CPU cores enabled per node in the Autonomous VM cluster."]
    pub fn cpu_core_count_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_percentage` after provisioning.\nhe percentage of total CPU cores currently in use in the Autonomous VM cluster."]
    pub fn cpu_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Autonomous VM cluster was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in GB."]
    pub fn data_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in TB."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers associated with the Autonomous VM cluster."]
    pub fn db_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the Autonomous VM cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Autonomous VM cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the Autonomous VM cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_storage_in_tbs_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the Exadata storage, in TB."]
    pub fn exadata_storage_in_tbs_lowest_scaled_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_storage_in_tbs_lowest_scaled_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the Autonomous VM cluster."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique ID of the Autonomous VM cluster."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_enabled_vm_cluster` after provisioning.\n Indicates whether mutual TLS (mTLS) authentication is enabled for the Autonomous VM cluster."]
    pub fn is_mtls_enabled_vm_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_enabled_vm_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model that applies to the Autonomous VM cluster. Valid values are LICENSE_INCLUDED or BRING_YOUR_OWN_LICENSE ."]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_window` after provisioning.\nThe maintenance window for the Autonomous VM cluster."]
    pub fn maintenance_window(&self) -> ListRef<DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_acds_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the maximum number of Autonomous CDBs."]
    pub fn max_acds_lowest_scaled_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_acds_lowest_scaled_value", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_per_oracle_compute_unit_in_gbs` after provisioning.\nThe amount of memory allocated per Oracle Compute Unit, in GB."]
    pub fn memory_per_oracle_compute_unit_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_per_oracle_compute_unit_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe total amount of memory allocated to the Autonomous VM cluster, in gigabytes (GB)."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of database server nodes in the Autonomous VM cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `non_provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can't be provisioned because of resource  constraints."]
    pub fn non_provisionable_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with this Autonomous VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe URL for accessing the OCI console page for this Autonomous VM cluster."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the Autonomous VM cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network associated with this Autonomous VM cluster."]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_node_storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the Autonomous VM cluster, in gigabytes (GB)."]
    pub fn odb_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can be provisioned in the Autonomous VM cluster."]
    pub fn provisionable_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_autonomous_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_cpus` after provisioning.\nThe number of CPU cores currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reclaimable_cpus` after provisioning.\nThe number of CPU cores that can be reclaimed from terminated or scaled-down Autonomous Databases."]
    pub fn reclaimable_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.reclaimable_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_cpus` after provisioning.\nThe number of CPU cores reserved for system operations and redundancy."]
    pub fn reserved_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.reserved_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_non_tls` after provisioning.\nThe SCAN listener port for non-TLS (TCP) protocol. The default is 1521."]
    pub fn scan_listener_port_non_tls(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_non_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tls` after provisioning.\nThe SCAN listener port for TLS (TCP) protocol. The default is 2484."]
    pub fn scan_listener_port_tls(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the Exadata infrastructure for the Autonomous VM cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the Autonomous VM cluster."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the Autonomous VM cluster."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_database_ssl_certificate_expires` after provisioning.\nThe expiration date and time of the database SSL certificate."]
    pub fn time_database_ssl_certificate_expires(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_database_ssl_certificate_expires", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_ords_certificate_expires` after provisioning.\nThe expiration date and time of the Oracle REST Data Services (ORDS)certificate ."]
    pub fn time_ords_certificate_expires(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_ords_certificate_expires", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the Autonomous VM cluster."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_container_databases` after provisioning.\nThe total number of Autonomous Container Databases that can be created with the allocated local storage."]
    pub fn total_container_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_container_databases", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {
    type O = BlockAssignable<DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {}

impl BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {
    pub fn build(self) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl { name: core::default::Default::default() }
    }
}

pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {
    type O = BlockAssignable<DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {}

impl BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {
    pub fn build(self) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl { name: core::default::Default::default() }
    }
}

pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<SetField<DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_of_day: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lead_time_in_weeks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    months: Option<SetField<DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weeks_of_month: Option<SetField<PrimField<f64>>>,
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(
        mut self,
        v: impl Into<SetField<DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekEl>>,
    ) -> Self {
        self.days_of_week = Some(v.into());
        self
    }

    #[doc = "Set the field `hours_of_day`.\n"]
    pub fn set_hours_of_day(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.hours_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `lead_time_in_weeks`.\n"]
    pub fn set_lead_time_in_weeks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lead_time_in_weeks = Some(v.into());
        self
    }

    #[doc = "Set the field `months`.\n"]
    pub fn set_months(
        mut self,
        v: impl Into<SetField<DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsEl>>,
    ) -> Self {
        self.months = Some(v.into());
        self
    }

    #[doc = "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preference = Some(v.into());
        self
    }

    #[doc = "Set the field `weeks_of_month`.\n"]
    pub fn set_weeks_of_month(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.weeks_of_month = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
    type O = BlockAssignable<DataOdbCloudAutonomousVmClusterMaintenanceWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowEl {}

impl BuildDataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
    pub fn build(self) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowEl {
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            lead_time_in_weeks: core::default::Default::default(),
            months: core::default::Default::default(),
            preference: core::default::Default::default(),
            weeks_of_month: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef {
        DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudAutonomousVmClusterMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> SetRef<DataOdbCloudAutonomousVmClusterMaintenanceWindowElDaysOfWeekElRef> {
        SetRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"]
    pub fn hours_of_day(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `lead_time_in_weeks` after provisioning.\n"]
    pub fn lead_time_in_weeks(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lead_time_in_weeks", self.base))
    }

    #[doc = "Get a reference to the value of field `months` after provisioning.\n"]
    pub fn months(&self) -> SetRef<DataOdbCloudAutonomousVmClusterMaintenanceWindowElMonthsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"]
    pub fn weeks_of_month(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.weeks_of_month", self.base))
    }
}
