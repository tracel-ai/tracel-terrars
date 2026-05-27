use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OracleDatabaseCloudVmClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_odb_subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_subnet_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    cloud_vm_cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    exadata_infrastructure: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<OracleDatabaseCloudVmClusterPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OracleDatabaseCloudVmClusterTimeoutsEl>,
    dynamic: OracleDatabaseCloudVmClusterDynamic,
}
struct OracleDatabaseCloudVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OracleDatabaseCloudVmClusterData>,
}
#[derive(Clone)]
pub struct OracleDatabaseCloudVmCluster(Rc<OracleDatabaseCloudVmCluster_>);
impl OracleDatabaseCloudVmCluster {
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
    #[doc = "Set the field `backup_odb_subnet`.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn set_backup_odb_subnet(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_odb_subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_subnet_cidr`.\nCIDR range of the backup subnet."]
    pub fn set_backup_subnet_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_subnet_cidr = Some(v.into());
        self
    }
    #[doc = "Set the field `cidr`.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn set_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser friendly name for this resource."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn set_odb_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().odb_network = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_subnet`.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn set_odb_subnet(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().odb_subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl Into<BlockAssignable<OracleDatabaseCloudVmClusterPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OracleDatabaseCloudVmClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<OracleDatabaseCloudVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudVmClusterTimeoutsElRef {
        OracleDatabaseCloudVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OracleDatabaseCloudVmCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OracleDatabaseCloudVmCluster {}
impl ToListMappable for OracleDatabaseCloudVmCluster {
    type O = ListRef<OracleDatabaseCloudVmClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OracleDatabaseCloudVmCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_oracle_database_cloud_vm_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOracleDatabaseCloudVmCluster {
    pub tf_id: String,
    #[doc = "The ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_vm_cluster_id: PrimField<String>,
    #[doc = "The name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub exadata_infrastructure: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub location: PrimField<String>,
}
impl BuildOracleDatabaseCloudVmCluster {
    pub fn build(self, stack: &mut Stack) -> OracleDatabaseCloudVmCluster {
        let out = OracleDatabaseCloudVmCluster(Rc::new(OracleDatabaseCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OracleDatabaseCloudVmClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_odb_subnet: core::default::Default::default(),
                backup_subnet_cidr: core::default::Default::default(),
                cidr: core::default::Default::default(),
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                exadata_infrastructure: self.exadata_infrastructure,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                odb_network: core::default::Default::default(),
                odb_subnet: core::default::Default::default(),
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OracleDatabaseCloudVmClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseCloudVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OracleDatabaseCloudVmClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<OracleDatabaseCloudVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudVmClusterTimeoutsElRef {
        OracleDatabaseCloudVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_monitoring_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_logs_enabled: Option<PrimField<bool>>,
}
impl OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    #[doc = "Set the field `diagnostics_events_enabled`.\nIndicates whether diagnostic collection is enabled for the VM cluster"]
    pub fn set_diagnostics_events_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.diagnostics_events_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `health_monitoring_enabled`.\nIndicates whether health monitoring is enabled for the VM cluster"]
    pub fn set_health_monitoring_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.health_monitoring_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `incident_logs_enabled`.\nIndicates whether incident logs and trace collection are enabled for the VM\ncluster"]
    pub fn set_incident_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.incident_logs_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    type O =
        BlockAssignable<OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {}
impl BuildOracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
    pub fn build(
        self,
    ) -> OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
        OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl {
            diagnostics_events_enabled: core::default::Default::default(),
            health_monitoring_enabled: core::default::Default::default(),
            incident_logs_enabled: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
        OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `diagnostics_events_enabled` after provisioning.\nIndicates whether diagnostic collection is enabled for the VM cluster"]
    pub fn diagnostics_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.diagnostics_events_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_monitoring_enabled` after provisioning.\nIndicates whether health monitoring is enabled for the VM cluster"]
    pub fn health_monitoring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_monitoring_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `incident_logs_enabled` after provisioning.\nIndicates whether incident logs and trace collection are enabled for the VM\ncluster"]
    pub fn incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incident_logs_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    #[doc = "Set the field `id`.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    type O = BlockAssignable<OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {}
impl BuildOracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
    pub fn build(self) -> OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
        OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
        OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseCloudVmClusterPropertiesElDynamic {
    diagnostics_data_collection_options: Option<
        DynamicBlock<OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl>,
    >,
    time_zone: Option<DynamicBlock<OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseCloudVmClusterPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_name: Option<PrimField<String>>,
    cpu_core_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage_size_tb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_server_ocids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_redundancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gi_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_prefix: Option<PrimField<String>>,
    license_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_backup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparse_diskgroup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_public_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics_data_collection_options:
        Option<Vec<OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<Vec<OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>>,
    dynamic: OracleDatabaseCloudVmClusterPropertiesElDynamic,
}
impl OracleDatabaseCloudVmClusterPropertiesEl {
    #[doc = "Set the field `cluster_name`.\nOCI Cluster name."]
    pub fn set_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_tb`.\nThe data disk group size to be allocated in TBs."]
    pub fn set_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_node_storage_size_gb`.\nLocal storage per VM"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_server_ocids`.\nOCID of database servers."]
    pub fn set_db_server_ocids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.db_server_ocids = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_redundancy`.\nThe type of redundancy. \n Possible values:\n DISK_REDUNDANCY_UNSPECIFIED\nHIGH\nNORMAL"]
    pub fn set_disk_redundancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_redundancy = Some(v.into());
        self
    }
    #[doc = "Set the field `gi_version`.\nGrid Infrastructure Version."]
    pub fn set_gi_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gi_version = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname_prefix`.\nPrefix for VM cluster host names."]
    pub fn set_hostname_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `local_backup_enabled`.\nUse local backup."]
    pub fn set_local_backup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.local_backup_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\nMemory allocated in GBs."]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nNumber of database servers."]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `ocpu_count`.\nOCPU count per VM. Minimum is 0.1."]
    pub fn set_ocpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ocpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `sparse_diskgroup_enabled`.\nUse exadata sparse snapshots."]
    pub fn set_sparse_diskgroup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sparse_diskgroup_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ssh_public_keys`.\nSSH public keys to be stored with cluster."]
    pub fn set_ssh_public_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ssh_public_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `diagnostics_data_collection_options`.\n"]
    pub fn set_diagnostics_data_collection_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.diagnostics_data_collection_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.diagnostics_data_collection_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseCloudVmClusterPropertiesElTimeZoneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_zone = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_zone = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OracleDatabaseCloudVmClusterPropertiesEl {
    type O = BlockAssignable<OracleDatabaseCloudVmClusterPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseCloudVmClusterPropertiesEl {
    #[doc = "Number of enabled CPU cores."]
    pub cpu_core_count: PrimField<f64>,
    #[doc = "License type of VM Cluster. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub license_type: PrimField<String>,
}
impl BuildOracleDatabaseCloudVmClusterPropertiesEl {
    pub fn build(self) -> OracleDatabaseCloudVmClusterPropertiesEl {
        OracleDatabaseCloudVmClusterPropertiesEl {
            cluster_name: core::default::Default::default(),
            cpu_core_count: self.cpu_core_count,
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocids: core::default::Default::default(),
            disk_redundancy: core::default::Default::default(),
            gi_version: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            license_type: self.license_type,
            local_backup_enabled: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            sparse_diskgroup_enabled: core::default::Default::default(),
            ssh_public_keys: core::default::Default::default(),
            diagnostics_data_collection_options: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseCloudVmClusterPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseCloudVmClusterPropertiesElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseCloudVmClusterPropertiesElRef {
        OracleDatabaseCloudVmClusterPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseCloudVmClusterPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOCI Cluster name."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }
    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\nCompartment ID of cluster."]
    pub fn compartment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compartment_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of enabled CPU cores."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\nThe data disk group size to be allocated in TBs."]
    pub fn data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\nLocal storage per VM"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_ocids` after provisioning.\nOCID of database servers."]
    pub fn db_server_ocids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_server_ocids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy. \n Possible values:\n DISK_REDUNDANCY_UNSPECIFIED\nHIGH\nNORMAL"]
    pub fn disk_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_redundancy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_listener_ip` after provisioning.\nDNS listener IP."]
    pub fn dns_listener_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_listener_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nParent DNS domain where SCAN DNS and hosts names are qualified.\nex: ocispdelegated.ocisp10jvnet.oraclevcn.com"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\nGrid Infrastructure Version."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nhost name without domain.\nformat: \"-\" with some suffix.\nex: sp2-yi0xq where \"sp2\" is the hostname_prefix."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\nPrefix for VM cluster host names."]
    pub fn hostname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `license_type` after provisioning.\nLicense type of VM Cluster. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }
    #[doc = "Get a reference to the value of field `local_backup_enabled` after provisioning.\nUse local backup."]
    pub fn local_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_backup_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nMemory allocated in GBs."]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nNumber of database servers."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Infrastructure ID of VM Cluster."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\nOCPU count per VM. Minimum is 0.1."]
    pub fn ocpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_dns` after provisioning.\nSCAN DNS name.\nex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com"]
    pub fn scan_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_dns", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nOCID of scan DNS record."]
    pub fn scan_dns_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_dns_record_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\nOCIDs of scan IPs."]
    pub fn scan_ip_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nSCAN listener port - TCP"]
    pub fn scan_listener_port_tcp(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_listener_port_tcp_ssl` after provisioning.\nSCAN listener port - TLS"]
    pub fn scan_listener_port_tcp_ssl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp_ssl", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nShape of VM Cluster."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }
    #[doc = "Get a reference to the value of field `sparse_diskgroup_enabled` after provisioning.\nUse exadata sparse snapshots."]
    pub fn sparse_diskgroup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sparse_diskgroup_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\nSSH public keys to be stored with cluster."]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the cluster. \n Possible values:\n STATE_UNSPECIFIED\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMAINTENANCE_IN_PROGRESS"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_size_gb` after provisioning.\nThe storage allocation for the disk group, in gigabytes (GB)."]
    pub fn storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `system_version` after provisioning.\nOperating system version of the image."]
    pub fn system_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `diagnostics_data_collection_options` after provisioning.\n"]
    pub fn diagnostics_data_collection_options(
        &self,
    ) -> ListRef<OracleDatabaseCloudVmClusterPropertiesElDiagnosticsDataCollectionOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.diagnostics_data_collection_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> ListRef<OracleDatabaseCloudVmClusterPropertiesElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseCloudVmClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OracleDatabaseCloudVmClusterTimeoutsEl {
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
impl ToListMappable for OracleDatabaseCloudVmClusterTimeoutsEl {
    type O = BlockAssignable<OracleDatabaseCloudVmClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseCloudVmClusterTimeoutsEl {}
impl BuildOracleDatabaseCloudVmClusterTimeoutsEl {
    pub fn build(self) -> OracleDatabaseCloudVmClusterTimeoutsEl {
        OracleDatabaseCloudVmClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseCloudVmClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseCloudVmClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseCloudVmClusterTimeoutsElRef {
        OracleDatabaseCloudVmClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseCloudVmClusterTimeoutsElRef {
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
struct OracleDatabaseCloudVmClusterDynamic {
    properties: Option<DynamicBlock<OracleDatabaseCloudVmClusterPropertiesEl>>,
}
