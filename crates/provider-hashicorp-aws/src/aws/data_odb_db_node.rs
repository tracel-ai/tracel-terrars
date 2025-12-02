use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOdbDbNodeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_vm_cluster_id: PrimField<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataOdbDbNode_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbDbNodeData>,
}
#[derive(Clone)]
pub struct DataOdbDbNode(Rc<DataOdbDbNode_>);
impl DataOdbDbNode {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\nAdditional information about the planned maintenance."]
    pub fn additional_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `backup_ip_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the backup IP address that's associated with the DB node."]
    pub fn backup_ip_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_ip_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vnic2_id` after provisioning.\nThe OCID of the second backup VNIC."]
    pub fn backup_vnic2_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vnic2_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vnic_id` after provisioning.\nThe OCID of the backup VNIC."]
    pub fn backup_vnic_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vnic_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\n"]
    pub fn cloud_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of CPU cores enabled on the DB node."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the DB node was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_id` after provisioning.\nThe unique identifier of the DB server that is associated with the DB node."]
    pub fn db_server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), allocated on the DB node."]
    pub fn db_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\nThe OCID of the DB system."]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fault_domain` after provisioning.\nThe name of the fault domain the instance is contained in."]
    pub fn fault_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fault_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `floating_ip_address` after provisioning.\nThe floating IP address assigned to the DB node."]
    pub fn floating_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.floating_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_ip_id` after provisioning.\nThe OCID of the host IP address that's associated with the DB node."]
    pub fn host_ip_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_ip_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nThe host name for the DB node."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `maintenance_type` after provisioning.\nThe type of database node maintenance. Either VMDB_REBOOT_MIGRATION or EXADBXS_REBOOT_MIGRATION."]
    pub fn maintenance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the DB node."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the DB node."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the DB node."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\nThe private IP address assigned to the DB node."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_storage_size_in_gbs` after provisioning.\nThe size (in GB) of the block storage volume allocation for the DB system."]
    pub fn software_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.software_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the DB node."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the DB node."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_maintenance_window_end` after provisioning.\nEnd date and time of maintenance window."]
    pub fn time_maintenance_window_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_maintenance_window_end", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_maintenance_window_start` after provisioning.\nStart date and time of maintenance window."]
    pub fn time_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_maintenance_window_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_cpu_core_count` after provisioning.\nThe total number of CPU cores reserved on the DB node."]
    pub fn total_cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_core_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vnic2_id` after provisioning.\nThe OCID of the second VNIC."]
    pub fn vnic2_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vnic2_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vnic_id` after provisioning.\nThe OCID of the VNIC."]
    pub fn vnic_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vnic_id", self.extract_ref()),
        )
    }
}
impl Referable for DataOdbDbNode {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOdbDbNode {}
impl ToListMappable for DataOdbDbNode {
    type O = ListRef<DataOdbDbNodeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOdbDbNode_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_db_node".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOdbDbNode {
    pub tf_id: String,
    #[doc = ""]
    pub cloud_vm_cluster_id: PrimField<String>,
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildDataOdbDbNode {
    pub fn build(self, stack: &mut Stack) -> DataOdbDbNode {
        let out = DataOdbDbNode(Rc::new(DataOdbDbNode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbDbNodeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOdbDbNodeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbDbNodeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOdbDbNodeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\nAdditional information about the planned maintenance."]
    pub fn additional_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `backup_ip_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the backup IP address that's associated with the DB node."]
    pub fn backup_ip_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_ip_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vnic2_id` after provisioning.\nThe OCID of the second backup VNIC."]
    pub fn backup_vnic2_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vnic2_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vnic_id` after provisioning.\nThe OCID of the backup VNIC."]
    pub fn backup_vnic_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vnic_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\n"]
    pub fn cloud_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of CPU cores enabled on the DB node."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the DB node was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_id` after provisioning.\nThe unique identifier of the DB server that is associated with the DB node."]
    pub fn db_server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), allocated on the DB node."]
    pub fn db_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\nThe OCID of the DB system."]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fault_domain` after provisioning.\nThe name of the fault domain the instance is contained in."]
    pub fn fault_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fault_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `floating_ip_address` after provisioning.\nThe floating IP address assigned to the DB node."]
    pub fn floating_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.floating_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_ip_id` after provisioning.\nThe OCID of the host IP address that's associated with the DB node."]
    pub fn host_ip_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_ip_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nThe host name for the DB node."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `maintenance_type` after provisioning.\nThe type of database node maintenance. Either VMDB_REBOOT_MIGRATION or EXADBXS_REBOOT_MIGRATION."]
    pub fn maintenance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the DB node."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the DB node."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the DB node."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\nThe private IP address assigned to the DB node."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_storage_size_in_gbs` after provisioning.\nThe size (in GB) of the block storage volume allocation for the DB system."]
    pub fn software_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.software_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the DB node."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the DB node."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_maintenance_window_end` after provisioning.\nEnd date and time of maintenance window."]
    pub fn time_maintenance_window_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_maintenance_window_end", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_maintenance_window_start` after provisioning.\nStart date and time of maintenance window."]
    pub fn time_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_maintenance_window_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_cpu_core_count` after provisioning.\nThe total number of CPU cores reserved on the DB node."]
    pub fn total_cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_core_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vnic2_id` after provisioning.\nThe OCID of the second VNIC."]
    pub fn vnic2_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vnic2_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vnic_id` after provisioning.\nThe OCID of the VNIC."]
    pub fn vnic_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vnic_id", self.extract_ref()),
        )
    }
}
