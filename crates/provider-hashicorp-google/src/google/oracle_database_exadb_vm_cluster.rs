use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OracleDatabaseExadbVmClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_odb_subnet: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    exadb_vm_cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network: Option<PrimField<String>>,
    odb_subnet: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<OracleDatabaseExadbVmClusterPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OracleDatabaseExadbVmClusterTimeoutsEl>,
    dynamic: OracleDatabaseExadbVmClusterDynamic,
}
struct OracleDatabaseExadbVmCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OracleDatabaseExadbVmClusterData>,
}
#[derive(Clone)]
pub struct OracleDatabaseExadbVmCluster(Rc<OracleDatabaseExadbVmCluster_>);
impl OracleDatabaseExadbVmCluster {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn set_odb_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().odb_network = Some(v.into());
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
        v: impl Into<BlockAssignable<OracleDatabaseExadbVmClusterPropertiesEl>>,
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
    pub fn set_timeouts(self, v: impl Into<OracleDatabaseExadbVmClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the ExadbVmCluster was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the ExadbVmCluster."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exadb_vm_cluster_id` after provisioning.\nThe ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exadb_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadb_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExadbVmCluster is hosted.\nExample: us-east4-b-r2.\nDuring creation, the system will pick the zone assigned to the\nExascaleDbStorageVault."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the ExadbVmCluster resource in the following format:\nprojects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> ListRef<OracleDatabaseExadbVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExadbVmClusterTimeoutsElRef {
        OracleDatabaseExadbVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OracleDatabaseExadbVmCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OracleDatabaseExadbVmCluster {}
impl ToListMappable for OracleDatabaseExadbVmCluster {
    type O = ListRef<OracleDatabaseExadbVmClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OracleDatabaseExadbVmCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_oracle_database_exadb_vm_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOracleDatabaseExadbVmCluster {
    pub tf_id: String,
    #[doc = "The name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub backup_odb_subnet: PrimField<String>,
    #[doc = "The display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub display_name: PrimField<String>,
    #[doc = "The ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub exadb_vm_cluster_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub odb_subnet: PrimField<String>,
}
impl BuildOracleDatabaseExadbVmCluster {
    pub fn build(self, stack: &mut Stack) -> OracleDatabaseExadbVmCluster {
        let out = OracleDatabaseExadbVmCluster(Rc::new(OracleDatabaseExadbVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OracleDatabaseExadbVmClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_odb_subnet: self.backup_odb_subnet,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: self.display_name,
                exadb_vm_cluster_id: self.exadb_vm_cluster_id,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                odb_network: core::default::Default::default(),
                odb_subnet: self.odb_subnet,
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
pub struct OracleDatabaseExadbVmClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OracleDatabaseExadbVmClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_odb_subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the ExadbVmCluster was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the ExadbVmCluster."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exadb_vm_cluster_id` after provisioning.\nThe ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exadb_vm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadb_vm_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExadbVmCluster is hosted.\nExample: us-east4-b-r2.\nDuring creation, the system will pick the zone assigned to the\nExascaleDbStorageVault."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the ExadbVmCluster resource in the following format:\nprojects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> ListRef<OracleDatabaseExadbVmClusterPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExadbVmClusterTimeoutsElRef {
        OracleDatabaseExadbVmClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_diagnostics_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_health_monitoring_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_incident_logs_enabled: Option<PrimField<bool>>,
}
impl OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
    #[doc = "Set the field `is_diagnostics_events_enabled`.\nIndicates whether to enable data collection for diagnostics."]
    pub fn set_is_diagnostics_events_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_diagnostics_events_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `is_health_monitoring_enabled`.\nIndicates whether to enable health monitoring."]
    pub fn set_is_health_monitoring_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_health_monitoring_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `is_incident_logs_enabled`.\nIndicates whether to enable incident logs and trace collection."]
    pub fn set_is_incident_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_incident_logs_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
    type O = BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {}
impl BuildOracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
    pub fn build(self) -> OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
        OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl {
            is_diagnostics_events_enabled: core::default::Default::default(),
            is_health_monitoring_enabled: core::default::Default::default(),
            is_incident_logs_enabled: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef {
        OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_diagnostics_events_enabled` after provisioning.\nIndicates whether to enable data collection for diagnostics."]
    pub fn is_diagnostics_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_diagnostics_events_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_health_monitoring_enabled` after provisioning.\nIndicates whether to enable health monitoring."]
    pub fn is_health_monitoring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_health_monitoring_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\nIndicates whether to enable incident logs and trace collection."]
    pub fn is_incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_incident_logs_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
    #[doc = "Set the field `id`.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nIANA Time Zone Database version number. For example \"2019a\"."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
    type O = BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {}
impl BuildOracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
    pub fn build(self) -> OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
        OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef {
        OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number. For example \"2019a\"."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
    size_in_gbs_per_node: PrimField<f64>,
}
impl OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {}
impl ToListMappable for OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
    type O = BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
    #[doc = "The storage allocation for the exadbvmcluster per node, in gigabytes (GB).\nThis field is used to calculate the total storage allocation for the\nexadbvmcluster."]
    pub size_in_gbs_per_node: PrimField<f64>,
}
impl BuildOracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
    pub fn build(self) -> OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
        OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl {
            size_in_gbs_per_node: self.size_in_gbs_per_node,
        }
    }
}
pub struct OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef {
        OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_in_gbs_per_node` after provisioning.\nThe storage allocation for the exadbvmcluster per node, in gigabytes (GB).\nThis field is used to calculate the total storage allocation for the\nexadbvmcluster."]
    pub fn size_in_gbs_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_in_gbs_per_node", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseExadbVmClusterPropertiesElDynamic {
    data_collection_options:
        Option<DynamicBlock<OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl>>,
    time_zone: Option<DynamicBlock<OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl>>,
    vm_file_system_storage:
        Option<DynamicBlock<OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseExadbVmClusterPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_ecpu_count_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_name: Option<PrimField<String>>,
    enabled_ecpu_count_per_node: PrimField<f64>,
    exascale_db_storage_vault: PrimField<String>,
    grid_image_id: PrimField<String>,
    hostname_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_model: Option<PrimField<String>>,
    node_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_listener_port_tcp: Option<PrimField<f64>>,
    shape_attribute: PrimField<String>,
    ssh_public_keys: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_collection_options:
        Option<Vec<OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<Vec<OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_file_system_storage:
        Option<Vec<OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl>>,
    dynamic: OracleDatabaseExadbVmClusterPropertiesElDynamic,
}
impl OracleDatabaseExadbVmClusterPropertiesEl {
    #[doc = "Set the field `additional_ecpu_count_per_node`.\nThe number of additional ECPUs per node for an Exadata VM cluster on\nexascale infrastructure."]
    pub fn set_additional_ecpu_count_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_ecpu_count_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_name`.\nThe cluster name for Exascale vm cluster. The cluster name must begin with\nan alphabetic character and may contain hyphens(-) but can not contain\nunderscores(_). It should be not more than 11 characters and is not case\nsensitive.\nOCI Cluster name."]
    pub fn set_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `license_model`.\nThe license type of the ExadbVmCluster.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn set_license_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.license_model = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_listener_port_tcp`.\nSCAN listener port - TCP"]
    pub fn set_scan_listener_port_tcp(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp = Some(v.into());
        self
    }
    #[doc = "Set the field `data_collection_options`.\n"]
    pub fn set_data_collection_options(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_collection_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_collection_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElTimeZoneEl>>,
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
    #[doc = "Set the field `vm_file_system_storage`.\n"]
    pub fn set_vm_file_system_storage(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vm_file_system_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_file_system_storage = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OracleDatabaseExadbVmClusterPropertiesEl {
    type O = BlockAssignable<OracleDatabaseExadbVmClusterPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExadbVmClusterPropertiesEl {
    #[doc = "The number of ECPUs enabled per node for an exadata vm cluster on\nexascale infrastructure."]
    pub enabled_ecpu_count_per_node: PrimField<f64>,
    #[doc = "The name of ExascaleDbStorageVault associated with the ExadbVmCluster.\nIt can refer to an existing ExascaleDbStorageVault. Or a new one can be\ncreated during the ExadbVmCluster creation (requires\nstorage_vault_properties to be set).\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub exascale_db_storage_vault: PrimField<String>,
    #[doc = "Grid Infrastructure Version."]
    pub grid_image_id: PrimField<String>,
    #[doc = "Prefix for VM cluster host names."]
    pub hostname_prefix: PrimField<String>,
    #[doc = "The number of nodes/VMs in the ExadbVmCluster."]
    pub node_count: PrimField<f64>,
    #[doc = "The shape attribute of the VM cluster. The type of Exascale storage used\nfor Exadata VM cluster. The default is SMART_STORAGE which supports Oracle\nDatabase 23ai and later\nPossible values:\nSMART_STORAGE\nBLOCK_STORAGE"]
    pub shape_attribute: PrimField<String>,
    #[doc = "The SSH public keys for the ExadbVmCluster."]
    pub ssh_public_keys: ListField<PrimField<String>>,
}
impl BuildOracleDatabaseExadbVmClusterPropertiesEl {
    pub fn build(self) -> OracleDatabaseExadbVmClusterPropertiesEl {
        OracleDatabaseExadbVmClusterPropertiesEl {
            additional_ecpu_count_per_node: core::default::Default::default(),
            cluster_name: core::default::Default::default(),
            enabled_ecpu_count_per_node: self.enabled_ecpu_count_per_node,
            exascale_db_storage_vault: self.exascale_db_storage_vault,
            grid_image_id: self.grid_image_id,
            hostname_prefix: self.hostname_prefix,
            license_model: core::default::Default::default(),
            node_count: self.node_count,
            scan_listener_port_tcp: core::default::Default::default(),
            shape_attribute: self.shape_attribute,
            ssh_public_keys: self.ssh_public_keys,
            data_collection_options: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_file_system_storage: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseExadbVmClusterPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterPropertiesElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseExadbVmClusterPropertiesElRef {
        OracleDatabaseExadbVmClusterPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExadbVmClusterPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_ecpu_count_per_node` after provisioning.\nThe number of additional ECPUs per node for an Exadata VM cluster on\nexascale infrastructure."]
    pub fn additional_ecpu_count_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_ecpu_count_per_node", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nThe cluster name for Exascale vm cluster. The cluster name must begin with\nan alphabetic character and may contain hyphens(-) but can not contain\nunderscores(_). It should be not more than 11 characters and is not case\nsensitive.\nOCI Cluster name."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled_ecpu_count_per_node` after provisioning.\nThe number of ECPUs enabled per node for an exadata vm cluster on\nexascale infrastructure."]
    pub fn enabled_ecpu_count_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled_ecpu_count_per_node", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exascale_db_storage_vault` after provisioning.\nThe name of ExascaleDbStorageVault associated with the ExadbVmCluster.\nIt can refer to an existing ExascaleDbStorageVault. Or a new one can be\ncreated during the ExadbVmCluster creation (requires\nstorage_vault_properties to be set).\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn exascale_db_storage_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exascale_db_storage_vault", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\nThe Oracle Grid Infrastructure (GI) software version."]
    pub fn gi_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }
    #[doc = "Get a reference to the value of field `grid_image_id` after provisioning.\nGrid Infrastructure Version."]
    pub fn grid_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grid_image_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the ExadbVmCluster."]
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
    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nThe license type of the ExadbVmCluster.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\nState of the cluster.\nPossible values:\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMAINTENANCE_IN_PROGRESS"]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nMemory per VM (GB) (Read-only): Shows the amount of memory allocated to\neach VM. Memory is calculated based on 2.75 GB per Total ECPUs."]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes/VMs in the ExadbVmCluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `oci_uri` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nSCAN listener port - TCP"]
    pub fn scan_listener_port_tcp(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scan_listener_port_tcp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shape_attribute` after provisioning.\nThe shape attribute of the VM cluster. The type of Exascale storage used\nfor Exadata VM cluster. The default is SMART_STORAGE which supports Oracle\nDatabase 23ai and later\nPossible values:\nSMART_STORAGE\nBLOCK_STORAGE"]
    pub fn shape_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape_attribute", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe SSH public keys for the ExadbVmCluster."]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(
        &self,
    ) -> ListRef<OracleDatabaseExadbVmClusterPropertiesElDataCollectionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_collection_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> ListRef<OracleDatabaseExadbVmClusterPropertiesElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_file_system_storage` after provisioning.\n"]
    pub fn vm_file_system_storage(
        &self,
    ) -> ListRef<OracleDatabaseExadbVmClusterPropertiesElVmFileSystemStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_file_system_storage", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExadbVmClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OracleDatabaseExadbVmClusterTimeoutsEl {
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
impl ToListMappable for OracleDatabaseExadbVmClusterTimeoutsEl {
    type O = BlockAssignable<OracleDatabaseExadbVmClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExadbVmClusterTimeoutsEl {}
impl BuildOracleDatabaseExadbVmClusterTimeoutsEl {
    pub fn build(self) -> OracleDatabaseExadbVmClusterTimeoutsEl {
        OracleDatabaseExadbVmClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseExadbVmClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExadbVmClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseExadbVmClusterTimeoutsElRef {
        OracleDatabaseExadbVmClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExadbVmClusterTimeoutsElRef {
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
struct OracleDatabaseExadbVmClusterDynamic {
    properties: Option<DynamicBlock<OracleDatabaseExadbVmClusterPropertiesEl>>,
}
