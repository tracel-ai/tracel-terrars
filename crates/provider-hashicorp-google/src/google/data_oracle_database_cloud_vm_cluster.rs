use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseCloudVmClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_vm_cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataOracleDatabaseCloudVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseCloudVmClusterData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseCloudVmCluster(Rc<DataOracleDatabaseCloudVmCluster_>);
impl DataOracleDatabaseCloudVmCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nCIDR range of the backup subnet."]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_subnet_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nThe ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the VM cluster was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone\nof Exadata infrastructure."]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the VM Cluster resource with the format:\nprojects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nVarious properties and settings associated with Exadata VM cluster."]
    pub fn properties(&self) -> ListRef<DataOracleDatabaseCloudVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataOracleDatabaseCloudVmCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseCloudVmCluster {}
impl ToListMappable for DataOracleDatabaseCloudVmCluster {
    type O = ListRef<DataOracleDatabaseCloudVmClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseCloudVmCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_cloud_vm_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseCloudVmCluster {
    pub tf_id: String,
    #[doc = "The ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_vm_cluster_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseCloudVmCluster {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseCloudVmCluster {
        let out = DataOracleDatabaseCloudVmCluster(Rc::new(DataOracleDatabaseCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOracleDatabaseCloudVmClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOracleDatabaseCloudVmClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseCloudVmClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nCIDR range of the backup subnet."]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_subnet_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nThe ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the VM cluster was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone\nof Exadata infrastructure."]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the VM Cluster resource with the format:\nprojects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nVarious properties and settings associated with Exadata VM cluster."]
    pub fn properties(&self) -> ListRef<DataOracleDatabaseCloudVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_monitoring_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_logs_enabled: Option<PrimField<bool>>,
}
impl DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    #[doc = "Set the field `diagnostics_events_enabled`.\n"]
    pub fn set_diagnostics_events_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.diagnostics_events_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `health_monitoring_enabled`.\n"]
    pub fn set_health_monitoring_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.health_monitoring_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `incident_logs_enabled`.\n"]
    pub fn set_incident_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.incident_logs_enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl
{
    type O = BlockAssignable<
        DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {}
impl BuildDataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
        DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
            diagnostics_events_enabled: core::default::Default::default(),
            health_monitoring_enabled: core::default::Default::default(),
            incident_logs_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
        DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `diagnostics_events_enabled` after provisioning.\n"]
    pub fn diagnostics_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.diagnostics_events_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_monitoring_enabled` after provisioning.\n"]
    pub fn health_monitoring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_monitoring_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `incident_logs_enabled` after provisioning.\n"]
    pub fn incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incident_logs_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    type O = BlockAssignable<DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {}
impl BuildDataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
        DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
        DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusterPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compartment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage_size_tb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_server_ocids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics_data_collection_options: Option<
        ListField<DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_redundancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_listener_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gi_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_backup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_dns: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_dns_record_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_ip_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_listener_port_tcp: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_listener_port_tcp_ssl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparse_diskgroup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_public_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<ListField<DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>>,
}
impl DataOracleDatabaseCloudVmClusterPropertiesEl {
    #[doc = "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `compartment_id`.\n"]
    pub fn set_compartment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compartment_id = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_core_count = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_server_ocids`.\n"]
    pub fn set_db_server_ocids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.db_server_ocids = Some(v.into());
        self
    }
    #[doc = "Set the field `diagnostics_data_collection_options`.\n"]
    pub fn set_diagnostics_data_collection_options(
        mut self,
        v: impl Into<
            ListField<
                DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl,
            >,
        >,
    ) -> Self {
        self.diagnostics_data_collection_options = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_redundancy`.\n"]
    pub fn set_disk_redundancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_redundancy = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_listener_ip`.\n"]
    pub fn set_dns_listener_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_listener_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `gi_version`.\n"]
    pub fn set_gi_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gi_version = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname_prefix`.\n"]
    pub fn set_hostname_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `license_type`.\n"]
    pub fn set_license_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.license_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_backup_enabled`.\n"]
    pub fn set_local_backup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.local_backup_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_url = Some(v.into());
        self
    }
    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }
    #[doc = "Set the field `ocpu_count`.\n"]
    pub fn set_ocpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ocpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_dns`.\n"]
    pub fn set_scan_dns(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scan_dns = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_dns_record_id`.\n"]
    pub fn set_scan_dns_record_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scan_dns_record_id = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_ip_ids`.\n"]
    pub fn set_scan_ip_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scan_ip_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_listener_port_tcp`.\n"]
    pub fn set_scan_listener_port_tcp(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_listener_port_tcp_ssl`.\n"]
    pub fn set_scan_listener_port_tcp_ssl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shape = Some(v.into());
        self
    }
    #[doc = "Set the field `sparse_diskgroup_enabled`.\n"]
    pub fn set_sparse_diskgroup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sparse_diskgroup_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ssh_public_keys`.\n"]
    pub fn set_ssh_public_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ssh_public_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_size_gb`.\n"]
    pub fn set_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `system_version`.\n"]
    pub fn set_system_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.system_version = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<ListField<DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>>,
    ) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseCloudVmClusterPropertiesEl {
    type O = BlockAssignable<DataOracleDatabaseCloudVmClusterPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudVmClusterPropertiesEl {}
impl BuildDataOracleDatabaseCloudVmClusterPropertiesEl {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusterPropertiesEl {
        DataOracleDatabaseCloudVmClusterPropertiesEl {
            cluster_name: core::default::Default::default(),
            compartment_id: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocids: core::default::Default::default(),
            diagnostics_data_collection_options: core::default::Default::default(),
            disk_redundancy: core::default::Default::default(),
            dns_listener_ip: core::default::Default::default(),
            domain: core::default::Default::default(),
            gi_version: core::default::Default::default(),
            hostname: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            license_type: core::default::Default::default(),
            local_backup_enabled: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            scan_dns: core::default::Default::default(),
            scan_dns_record_id: core::default::Default::default(),
            scan_ip_ids: core::default::Default::default(),
            scan_listener_port_tcp: core::default::Default::default(),
            scan_listener_port_tcp_ssl: core::default::Default::default(),
            shape: core::default::Default::default(),
            sparse_diskgroup_enabled: core::default::Default::default(),
            ssh_public_keys: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_size_gb: core::default::Default::default(),
            system_version: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudVmClusterPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudVmClusterPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataOracleDatabaseCloudVmClusterPropertiesElRef {
        DataOracleDatabaseCloudVmClusterPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudVmClusterPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }
    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\n"]
    pub fn compartment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compartment_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_ocids` after provisioning.\n"]
    pub fn db_server_ocids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_server_ocids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `diagnostics_data_collection_options` after provisioning.\n"]
    pub fn diagnostics_data_collection_options(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.diagnostics_data_collection_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\n"]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_redundancy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_listener_ip` after provisioning.\n"]
    pub fn dns_listener_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_listener_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\n"]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\n"]
    pub fn hostname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `license_type` after provisioning.\n"]
    pub fn license_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }
    #[doc = "Get a reference to the value of field `local_backup_enabled` after provisioning.\n"]
    pub fn local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_backup_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\n"]
    pub fn ocpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_dns` after provisioning.\n"]
    pub fn scan_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\n"]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_record_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\n"]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\n"]
    pub fn scan_listener_port_tcp(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_listener_port_tcp_ssl` after provisioning.\n"]
    pub fn scan_listener_port_tcp_ssl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp_ssl", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }
    #[doc = "Get a reference to the value of field `sparse_diskgroup_enabled` after provisioning.\n"]
    pub fn sparse_diskgroup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sparse_diskgroup_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\n"]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_size_gb` after provisioning.\n"]
    pub fn storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `system_version` after provisioning.\n"]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> ListRef<DataOracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
