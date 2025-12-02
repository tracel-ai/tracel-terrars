use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOdbCloudExadataInfrastructureData {
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
struct DataOdbCloudExadataInfrastructure_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbCloudExadataInfrastructureData>,
}
#[derive(Clone)]
pub struct DataOdbCloudExadataInfrastructure(Rc<DataOdbCloudExadataInfrastructure_>);
impl DataOdbCloudExadataInfrastructure {
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
    #[doc = "Get a reference to the value of field `activated_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure."]
    pub fn activated_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activated_storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `additional_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure."]
    pub fn additional_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) for the Exadata infrastructure."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nhe name of the Availability Zone (AZ) where the Exadata infrastructure is located."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the Exadata infrastructure is located."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_storage_size_in_gbs` after provisioning.\nThe amount of available storage, in gigabytes (GB), for the Exadata infrastructure."]
    pub fn available_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\nThe number of database servers for the Exadata infrastructure."]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nThe total number of CPU cores that are allocated to the Exadata infrastructure."]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe time when the Exadata infrastructure was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nThe email addresses of contacts to receive notification from Oracle about maintenance updates for the Exadata infrastructure."]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> SetRef<DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the Exadata infrastructure's data disk group, in terabytes (TB)."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_in_tbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_server_type` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation."]
    pub fn database_server_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_server_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of\nvalid model names, use the ListDbSystemShapes operation."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_version` after provisioning.\nThe version of the Exadata infrastructure."]
    pub fn db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Exadata infrastructure."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the Exadata infrastructure."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_maintenance_run_id` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the last maintenance run for the Exadata infrastructure."]
    pub fn last_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_maintenance_run_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n The scheduling details for the maintenance window. Patching and system updates take place during the maintenance window "]
    pub fn maintenance_window(
        &self,
    ) -> ListRef<DataOdbCloudExadataInfrastructureMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available on the Exadata infrastructure."]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cpu_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_data_storage_in_tbs` after provisioning.\nThe total amount of data disk group storage, in terabytes (TB), that's available on the Exadata infrastructure."]
    pub fn max_data_storage_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_data_storage_in_tbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_db_node_storage_size_in_gbs` after provisioning.\nThe total amount of local node storage, in gigabytes (GB), that's available on the Exadata infrastructure."]
    pub fn max_db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total amount of memory, in gigabytes (GB), that's available on the Exadata infrastructure."]
    pub fn max_memory_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated on the Exadata infrastructure."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_db_server_version` after provisioning.\nThe monthly software version of the database servers installed on the Exadata infrastructure."]
    pub fn monthly_db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_db_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\nThe monthly software version of the storage servers installed on the Exadata infrastructure."]
    pub fn monthly_storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\nThe OCID of the next maintenance run for the Exadata infrastructure."]
    pub fn next_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the Exadata infrastructure."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the Exadata infrastructure in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the Exadata infrastructure in OCI."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the Exadata infrastructure expressed as a percentage."]
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
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe model name of the Exadata infrastructure."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the Exadata infrastructure."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the Exadata infrastructure."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_count` after provisioning.\nhe number of storage servers that are activated for the Exadata infrastructure."]
    pub fn storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_type` after provisioning.\nThe storage server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation."]
    pub fn storage_server_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_version` after provisioning.\nThe software version of the storage servers on the Exadata infrastructure."]
    pub fn storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_storage_size_in_gbs` after provisioning.\nThe total amount of storage, in gigabytes (GB), on the the Exadata infrastructure."]
    pub fn total_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_storage_size_in_gbs", self.extract_ref()),
        )
    }
}
impl Referable for DataOdbCloudExadataInfrastructure {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOdbCloudExadataInfrastructure {}
impl ToListMappable for DataOdbCloudExadataInfrastructure {
    type O = ListRef<DataOdbCloudExadataInfrastructureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOdbCloudExadataInfrastructure_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_cloud_exadata_infrastructure".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOdbCloudExadataInfrastructure {
    pub tf_id: String,
    #[doc = "The unique identifier of the Exadata infrastructure."]
    pub id: PrimField<String>,
}
impl BuildDataOdbCloudExadataInfrastructure {
    pub fn build(self, stack: &mut Stack) -> DataOdbCloudExadataInfrastructure {
        let out = DataOdbCloudExadataInfrastructure(Rc::new(DataOdbCloudExadataInfrastructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbCloudExadataInfrastructureData {
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
pub struct DataOdbCloudExadataInfrastructureRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbCloudExadataInfrastructureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOdbCloudExadataInfrastructureRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `activated_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure."]
    pub fn activated_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activated_storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `additional_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure."]
    pub fn additional_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) for the Exadata infrastructure."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nhe name of the Availability Zone (AZ) where the Exadata infrastructure is located."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the Exadata infrastructure is located."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_storage_size_in_gbs` after provisioning.\nThe amount of available storage, in gigabytes (GB), for the Exadata infrastructure."]
    pub fn available_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\nThe number of database servers for the Exadata infrastructure."]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nThe total number of CPU cores that are allocated to the Exadata infrastructure."]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe time when the Exadata infrastructure was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nThe email addresses of contacts to receive notification from Oracle about maintenance updates for the Exadata infrastructure."]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> SetRef<DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the Exadata infrastructure's data disk group, in terabytes (TB)."]
    pub fn data_storage_size_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_in_tbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_server_type` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation."]
    pub fn database_server_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_server_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of\nvalid model names, use the ListDbSystemShapes operation."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_version` after provisioning.\nThe version of the Exadata infrastructure."]
    pub fn db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Exadata infrastructure."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the Exadata infrastructure."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_maintenance_run_id` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the last maintenance run for the Exadata infrastructure."]
    pub fn last_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_maintenance_run_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n The scheduling details for the maintenance window. Patching and system updates take place during the maintenance window "]
    pub fn maintenance_window(
        &self,
    ) -> ListRef<DataOdbCloudExadataInfrastructureMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available on the Exadata infrastructure."]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cpu_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_data_storage_in_tbs` after provisioning.\nThe total amount of data disk group storage, in terabytes (TB), that's available on the Exadata infrastructure."]
    pub fn max_data_storage_in_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_data_storage_in_tbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_db_node_storage_size_in_gbs` after provisioning.\nThe total amount of local node storage, in gigabytes (GB), that's available on the Exadata infrastructure."]
    pub fn max_db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total amount of memory, in gigabytes (GB), that's available on the Exadata infrastructure."]
    pub fn max_memory_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated on the Exadata infrastructure."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_db_server_version` after provisioning.\nThe monthly software version of the database servers installed on the Exadata infrastructure."]
    pub fn monthly_db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_db_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\nThe monthly software version of the storage servers installed on the Exadata infrastructure."]
    pub fn monthly_storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\nThe OCID of the next maintenance run for the Exadata infrastructure."]
    pub fn next_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the Exadata infrastructure."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the Exadata infrastructure in OCI."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the Exadata infrastructure in OCI."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ocid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the Exadata infrastructure expressed as a percentage."]
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
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe model name of the Exadata infrastructure."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the Exadata infrastructure."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the Exadata infrastructure."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_count` after provisioning.\nhe number of storage servers that are activated for the Exadata infrastructure."]
    pub fn storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_type` after provisioning.\nThe storage server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation."]
    pub fn storage_server_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_version` after provisioning.\nThe software version of the storage servers on the Exadata infrastructure."]
    pub fn storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_storage_size_in_gbs` after provisioning.\nThe total amount of storage, in gigabytes (GB), on the the Exadata infrastructure."]
    pub fn total_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_storage_size_in_gbs", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
    type O = BlockAssignable<DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {}
impl BuildDataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
    pub fn build(self) -> DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
        DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciEl {
            email: core::default::Default::default(),
        }
    }
}
pub struct DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef {
        DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbCloudExadataInfrastructureCustomerContactsToSendToOciElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
    type O = BlockAssignable<DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {}
impl BuildDataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
    pub fn build(self) -> DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
        DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef {
        DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
    type O = BlockAssignable<DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {}
impl BuildDataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
    pub fn build(self) -> DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
        DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef {
        DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_action_timeout_in_mins: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week:
        Option<SetField<DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_of_day: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_custom_action_timeout_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lead_time_in_weeks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    months: Option<SetField<DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patching_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weeks_of_month: Option<SetField<PrimField<f64>>>,
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowEl {
    #[doc = "Set the field `custom_action_timeout_in_mins`.\n"]
    pub fn set_custom_action_timeout_in_mins(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_action_timeout_in_mins = Some(v.into());
        self
    }
    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(
        mut self,
        v: impl Into<SetField<DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekEl>>,
    ) -> Self {
        self.days_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `hours_of_day`.\n"]
    pub fn set_hours_of_day(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.hours_of_day = Some(v.into());
        self
    }
    #[doc = "Set the field `is_custom_action_timeout_enabled`.\n"]
    pub fn set_is_custom_action_timeout_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_custom_action_timeout_enabled = Some(v.into());
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
        v: impl Into<SetField<DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsEl>>,
    ) -> Self {
        self.months = Some(v.into());
        self
    }
    #[doc = "Set the field `patching_mode`.\n"]
    pub fn set_patching_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.patching_mode = Some(v.into());
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
impl ToListMappable for DataOdbCloudExadataInfrastructureMaintenanceWindowEl {
    type O = BlockAssignable<DataOdbCloudExadataInfrastructureMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbCloudExadataInfrastructureMaintenanceWindowEl {}
impl BuildDataOdbCloudExadataInfrastructureMaintenanceWindowEl {
    pub fn build(self) -> DataOdbCloudExadataInfrastructureMaintenanceWindowEl {
        DataOdbCloudExadataInfrastructureMaintenanceWindowEl {
            custom_action_timeout_in_mins: core::default::Default::default(),
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            is_custom_action_timeout_enabled: core::default::Default::default(),
            lead_time_in_weeks: core::default::Default::default(),
            months: core::default::Default::default(),
            patching_mode: core::default::Default::default(),
            preference: core::default::Default::default(),
            weeks_of_month: core::default::Default::default(),
        }
    }
}
pub struct DataOdbCloudExadataInfrastructureMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbCloudExadataInfrastructureMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbCloudExadataInfrastructureMaintenanceWindowElRef {
        DataOdbCloudExadataInfrastructureMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbCloudExadataInfrastructureMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_action_timeout_in_mins` after provisioning.\n"]
    pub fn custom_action_timeout_in_mins(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_action_timeout_in_mins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(
        &self,
    ) -> SetRef<DataOdbCloudExadataInfrastructureMaintenanceWindowElDaysOfWeekElRef> {
        SetRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"]
    pub fn hours_of_day(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }
    #[doc = "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\n"]
    pub fn is_custom_action_timeout_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_custom_action_timeout_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lead_time_in_weeks` after provisioning.\n"]
    pub fn lead_time_in_weeks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lead_time_in_weeks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `months` after provisioning.\n"]
    pub fn months(
        &self,
    ) -> SetRef<DataOdbCloudExadataInfrastructureMaintenanceWindowElMonthsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.months", self.base))
    }
    #[doc = "Get a reference to the value of field `patching_mode` after provisioning.\n"]
    pub fn patching_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patching_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }
    #[doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"]
    pub fn weeks_of_month(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.weeks_of_month", self.base),
        )
    }
}
