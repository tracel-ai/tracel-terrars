use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OracleDatabaseDbSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    db_system_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_oracle_zone: Option<PrimField<String>>,
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
    properties: Option<Vec<OracleDatabaseDbSystemPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OracleDatabaseDbSystemTimeoutsEl>,
    dynamic: OracleDatabaseDbSystemDynamic,
}
struct OracleDatabaseDbSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OracleDatabaseDbSystemData>,
}
#[derive(Clone)]
pub struct OracleDatabaseDbSystem(Rc<OracleDatabaseDbSystem_>);
impl OracleDatabaseDbSystem {
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
    #[doc = "Set the field `gcp_oracle_zone`.\nThe GCP Oracle zone where Oracle DbSystem is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn set_gcp_oracle_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gcp_oracle_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels or tags associated with the DbSystem.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the DbSystem.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
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
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesEl>>,
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
    pub fn set_timeouts(self, v: impl Into<OracleDatabaseDbSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the DbSystem was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\nThe ID of the DbSystem to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the System db. The name does not have to\nbe unique within your project."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the DbSystem"]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle DbSystem is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the DbSystem.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the DbSystem resource in the following format:\nprojects/{project}/locations/{region}/dbSystems/{db_system}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the DbSystem.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the DbSystem for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> ListRef<OracleDatabaseDbSystemPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseDbSystemTimeoutsElRef {
        OracleDatabaseDbSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OracleDatabaseDbSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OracleDatabaseDbSystem {}
impl ToListMappable for OracleDatabaseDbSystem {
    type O = ListRef<OracleDatabaseDbSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OracleDatabaseDbSystem_ {
    fn extract_resource_type(&self) -> String {
        "google_oracle_database_db_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOracleDatabaseDbSystem {
    pub tf_id: String,
    #[doc = "The ID of the DbSystem to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub db_system_id: PrimField<String>,
    #[doc = "The display name for the System db. The name does not have to\nbe unique within your project."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The name of the OdbSubnet associated with the DbSystem for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub odb_subnet: PrimField<String>,
}
impl BuildOracleDatabaseDbSystem {
    pub fn build(self, stack: &mut Stack) -> OracleDatabaseDbSystem {
        let out = OracleDatabaseDbSystem(Rc::new(OracleDatabaseDbSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OracleDatabaseDbSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                db_system_id: self.db_system_id,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: self.display_name,
                gcp_oracle_zone: core::default::Default::default(),
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
pub struct OracleDatabaseDbSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OracleDatabaseDbSystemRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the DbSystem was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\nThe ID of the DbSystem to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn db_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_system_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the System db. The name does not have to\nbe unique within your project."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the DbSystem"]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle DbSystem is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the DbSystem.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the DbSystem resource in the following format:\nprojects/{project}/locations/{region}/dbSystems/{db_system}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the DbSystem.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the DbSystem for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> ListRef<OracleDatabaseDbSystemPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseDbSystemTimeoutsElRef {
        OracleDatabaseDbSystemTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_diagnostics_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_incident_logs_enabled: Option<PrimField<bool>>,
}
impl OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
    #[doc = "Set the field `is_diagnostics_events_enabled`.\nIndicates whether to enable data collection for diagnostics."]
    pub fn set_is_diagnostics_events_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_diagnostics_events_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `is_incident_logs_enabled`.\nIndicates whether to enable incident logs and trace collection."]
    pub fn set_is_incident_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_incident_logs_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {}
impl BuildOracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
        OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl {
            is_diagnostics_events_enabled: core::default::Default::default(),
            is_incident_logs_enabled: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef {
        OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef {
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
    #[doc = "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\nIndicates whether to enable incident logs and trace collection."]
    pub fn is_incident_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_incident_logs_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl
{}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl {}
impl ToListMappable
    for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl
{
    type O = BlockAssignable<
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl
{}
impl
    BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl
{
    pub fn build(
        self,
    ) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl
    {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl {}
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef { fn new (shared : StackShared , base : String) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef { OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef { shared : shared , base : base . to_string () , } } }
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `management_state` after provisioning.\nThe status of the Database Management service.\nPossible values:\nENABLING\nENABLED\nDISABLING\nDISABLED\nUPDATING\nFAILED_ENABLING\nFAILED_DISABLING\nFAILED_UPDATING"]
    pub fn management_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `management_type` after provisioning.\nThe Database Management type.\nPossible values:\nBASIC\nADVANCED"]
    pub fn management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl
{
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl { # [doc = "Set the field `type_`.\nThe type of the database backup destination.\nPossible values:\nNFS\nRECOVERY_APPLIANCE\nOBJECT_STORE\nLOCAL\nDBRS"] pub fn set_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_ = Some (v . into ()) ; self } }
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl { type O = BlockAssignable < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl
{}
impl BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl { pub fn build (self) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl { OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl { type_ : core :: default :: Default :: default () , } } }
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef { fn new (shared : StackShared , base : String) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef { OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef { shared : shared , base : base . to_string () , } } }
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the database backup destination.\nPossible values:\nNFS\nRECOVERY_APPLIANCE\nOBJECT_STORE\nLOCAL\nDBRS"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElDynamic { backup_destination_details : Option < DynamicBlock < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl >> , }
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl { # [serde (skip_serializing_if = "Option::is_none")] auto_backup_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] auto_full_backup_day : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] auto_full_backup_window : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] auto_incremental_backup_window : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] backup_deletion_policy : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] retention_period_days : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] backup_destination_details : Option < Vec < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl > > , dynamic : OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElDynamic , }
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl {
    #[doc = "Set the field `auto_backup_enabled`.\nIf set to true, enables automatic backups on the database."]
    pub fn set_auto_backup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_backup_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_full_backup_day`.\nPossible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub fn set_auto_full_backup_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_full_backup_day = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_full_backup_window`.\nThe window in which the full backup should be performed on the database.\nIf no value is provided, the default is anytime.\nPossible values:\nSLOT_ONE\nSLOT_TWO\nSLOT_THREE\nSLOT_FOUR\nSLOT_FIVE\nSLOT_SIX\nSLOT_SEVEN\nSLOT_EIGHT\nSLOT_NINE\nSLOT_TEN\nSLOT_ELEVEN\nSLOT_TWELVE"]
    pub fn set_auto_full_backup_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_full_backup_window = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_incremental_backup_window`.\nThe window in which the incremental backup should be performed on the\ndatabase. If no value is provided, the default is anytime except the auto\nfull backup day.\nPossible values:\nSLOT_ONE\nSLOT_TWO\nSLOT_THREE\nSLOT_FOUR\nSLOT_FIVE\nSLOT_SIX\nSLOT_SEVEN\nSLOT_EIGHT\nSLOT_NINE\nSLOT_TEN\nSLOT_ELEVEN\nSLOT_TWELVE"]
    pub fn set_auto_incremental_backup_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_incremental_backup_window = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_deletion_policy`.\nThis defines when the backups will be deleted after Database termination.\nPossible values:\nDELETE_IMMEDIATELY\nDELETE_AFTER_RETENTION_PERIOD"]
    pub fn set_backup_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_period_days`.\nThe number of days an automatic backup is retained before being\nautomatically deleted. This value determines the earliest point in time to\nwhich a database can be restored. Min: 1, Max: 60."]
    pub fn set_retention_period_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_period_days = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_destination_details`.\n"]
    pub fn set_backup_destination_details(
        mut self,
        v : impl Into < BlockAssignable < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backup_destination_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_destination_details = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl
{
    type O = BlockAssignable<
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl {
}
impl BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl {
    pub fn build(
        self,
    ) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl {
            auto_backup_enabled: core::default::Default::default(),
            auto_full_backup_day: core::default::Default::default(),
            auto_full_backup_window: core::default::Default::default(),
            auto_incremental_backup_window: core::default::Default::default(),
            backup_deletion_policy: core::default::Default::default(),
            retention_period_days: core::default::Default::default(),
            backup_destination_details: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_backup_enabled` after provisioning.\nIf set to true, enables automatic backups on the database."]
    pub fn auto_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_backup_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_full_backup_day` after provisioning.\nPossible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub fn auto_full_backup_day(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_full_backup_day", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_full_backup_window` after provisioning.\nThe window in which the full backup should be performed on the database.\nIf no value is provided, the default is anytime.\nPossible values:\nSLOT_ONE\nSLOT_TWO\nSLOT_THREE\nSLOT_FOUR\nSLOT_FIVE\nSLOT_SIX\nSLOT_SEVEN\nSLOT_EIGHT\nSLOT_NINE\nSLOT_TEN\nSLOT_ELEVEN\nSLOT_TWELVE"]
    pub fn auto_full_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_full_backup_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_incremental_backup_window` after provisioning.\nThe window in which the incremental backup should be performed on the\ndatabase. If no value is provided, the default is anytime except the auto\nfull backup day.\nPossible values:\nSLOT_ONE\nSLOT_TWO\nSLOT_THREE\nSLOT_FOUR\nSLOT_FIVE\nSLOT_SIX\nSLOT_SEVEN\nSLOT_EIGHT\nSLOT_NINE\nSLOT_TEN\nSLOT_ELEVEN\nSLOT_TWELVE"]
    pub fn auto_incremental_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_incremental_backup_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_deletion_policy` after provisioning.\nThis defines when the backups will be deleted after Database termination.\nPossible values:\nDELETE_IMMEDIATELY\nDELETE_AFTER_RETENTION_PERIOD"]
    pub fn backup_deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_period_days` after provisioning.\nThe number of days an automatic backup is retained before being\nautomatically deleted. This value determines the earliest point in time to\nwhich a database can be restored. Min: 1, Max: 60."]
    pub fn retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_destination_details` after provisioning.\n"]    pub fn backup_destination_details (& self) -> ListRef < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElBackupDestinationDetailsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_destination_details", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDynamic { database_management_config : Option < DynamicBlock < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl >> , db_backup_config : Option < DynamicBlock < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl >> , }
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl { db_version : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] database_management_config : Option < Vec < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] db_backup_config : Option < Vec < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl > > , dynamic : OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDynamic , }
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
    #[doc = "Set the field `database_management_config`.\n"]
    pub fn set_database_management_config(
        mut self,
        v : impl Into < BlockAssignable < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database_management_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database_management_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `db_backup_config`.\n"]
    pub fn set_db_backup_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.db_backup_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.db_backup_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
    #[doc = "The Oracle Database version."]
    pub db_version: PrimField<String>,
}
impl BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl {
            db_version: self.db_version,
            database_management_config: core::default::Default::default(),
            db_backup_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nThe Oracle Database version."]
    pub fn db_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Database.\nPossible values:\nPROVISIONING\nAVAILABLE\nUPDATING\nBACKUP_IN_PROGRESS\nUPGRADING\nCONVERTING\nTERMINATING\nTERMINATED\nRESTORE_FAILED\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `database_management_config` after provisioning.\n"]    pub fn database_management_config (& self) -> ListRef < OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDatabaseManagementConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_management_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_backup_config` after provisioning.\n"]
    pub fn db_backup_config(
        &self,
    ) -> ListRef<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElDbBackupConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_backup_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElDynamic {
    properties:
        Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
    admin_password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character_set: Option<PrimField<String>>,
    database_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_home_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_unique_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_oracle_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ncharacter_set: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pluggable_database_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pluggable_database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tde_wallet_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl>>,
    dynamic: OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElDynamic,
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
    #[doc = "Set the field `character_set`.\nThe character set for the database. The default is AL32UTF8."]
    pub fn set_character_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.character_set = Some(v.into());
        self
    }
    #[doc = "Set the field `db_home_name`.\nThe name of the DbHome resource associated with the Database."]
    pub fn set_db_home_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_home_name = Some(v.into());
        self
    }
    #[doc = "Set the field `db_name`.\nThe database name. The name must begin with an alphabetic character and can\ncontain a maximum of eight alphanumeric characters. Special characters are\nnot permitted."]
    pub fn set_db_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_name = Some(v.into());
        self
    }
    #[doc = "Set the field `db_unique_name`.\nThe DB_UNIQUE_NAME of the Oracle Database being backed up."]
    pub fn set_db_unique_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_unique_name = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_oracle_zone`.\nThe GCP Oracle zone where the Database is created."]
    pub fn set_gcp_oracle_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_oracle_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `ncharacter_set`.\nThe national character set for the database. The default is AL16UTF16."]
    pub fn set_ncharacter_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ncharacter_set = Some(v.into());
        self
    }
    #[doc = "Set the field `pluggable_database_id`.\nThe ID of the pluggable database associated with Database. The ID must be unique within the project and location."]
    pub fn set_pluggable_database_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pluggable_database_id = Some(v.into());
        self
    }
    #[doc = "Set the field `pluggable_database_name`.\nThe pluggable dataabse associated with the Database. The name must begin with an alphabetic character and can contain a maximum of thirty alphanumeric characters."]
    pub fn set_pluggable_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pluggable_database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tde_wallet_password`.\nThe TDE wallet password for the database."]
    pub fn set_tde_wallet_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tde_wallet_password = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
    #[doc = "The password for the default ADMIN user."]
    pub admin_password: PrimField<String>,
    #[doc = "The database ID of the Database."]
    pub database_id: PrimField<String>,
}
impl BuildOracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl {
            admin_password: self.admin_password,
            character_set: core::default::Default::default(),
            database_id: self.database_id,
            db_home_name: core::default::Default::default(),
            db_name: core::default::Default::default(),
            db_unique_name: core::default::Default::default(),
            gcp_oracle_zone: core::default::Default::default(),
            ncharacter_set: core::default::Default::default(),
            pluggable_database_id: core::default::Default::default(),
            pluggable_database_name: core::default::Default::default(),
            tde_wallet_password: core::default::Default::default(),
            properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef {
        OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_password` after provisioning.\nThe password for the default ADMIN user."]
    pub fn admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `character_set` after provisioning.\nThe character set for the database. The default is AL32UTF8."]
    pub fn character_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.character_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Database was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `database_id` after provisioning.\nThe database ID of the Database."]
    pub fn database_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }
    #[doc = "Get a reference to the value of field `db_home_name` after provisioning.\nThe name of the DbHome resource associated with the Database."]
    pub fn db_home_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_home_name", self.base))
    }
    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nThe database name. The name must begin with an alphabetic character and can\ncontain a maximum of eight alphanumeric characters. Special characters are\nnot permitted."]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }
    #[doc = "Get a reference to the value of field `db_unique_name` after provisioning.\nThe DB_UNIQUE_NAME of the Oracle Database being backed up."]
    pub fn db_unique_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_unique_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where the Database is created."]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Database resource in the following format:\nprojects/{project}/locations/{region}/databases/{database}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `ncharacter_set` after provisioning.\nThe national character set for the database. The default is AL16UTF16."]
    pub fn ncharacter_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ncharacter_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }
    #[doc = "Get a reference to the value of field `ops_insights_status` after provisioning.\nThe Status of Operations Insights for this Database.\nPossible values:\nENABLING\nENABLED\nDISABLING\nNOT_ENABLED\nFAILED_ENABLING\nFAILED_DISABLING"]
    pub fn ops_insights_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ops_insights_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pluggable_database_id` after provisioning.\nThe ID of the pluggable database associated with Database. The ID must be unique within the project and location."]
    pub fn pluggable_database_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pluggable_database_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pluggable_database_name` after provisioning.\nThe pluggable dataabse associated with the Database. The name must begin with an alphabetic character and can contain a maximum of thirty alphanumeric characters."]
    pub fn pluggable_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pluggable_database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tde_wallet_password` after provisioning.\nThe TDE wallet password for the database."]
    pub fn tde_wallet_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tde_wallet_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseDbSystemPropertiesElDbHomeElDynamic {
    database: Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbHomeEl {
    db_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_unified_auditing_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl>>,
    dynamic: OracleDatabaseDbSystemPropertiesElDbHomeElDynamic,
}
impl OracleDatabaseDbSystemPropertiesElDbHomeEl {
    #[doc = "Set the field `display_name`.\nThe display name for the Database Home. The name does not have to\nbe unique within your project."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `is_unified_auditing_enabled`.\nWhether unified auditing is enabled for the Database Home."]
    pub fn set_is_unified_auditing_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_unified_auditing_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDbHomeEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbHomeEl {
    #[doc = "A valid Oracle Database version. For a list of supported versions, use the\nListDbVersions operation."]
    pub db_version: PrimField<String>,
}
impl BuildOracleDatabaseDbSystemPropertiesElDbHomeEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElDbHomeEl {
        OracleDatabaseDbSystemPropertiesElDbHomeEl {
            db_version: self.db_version,
            display_name: core::default::Default::default(),
            is_unified_auditing_enabled: core::default::Default::default(),
            database: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbHomeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbHomeElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseDbSystemPropertiesElDbHomeElRef {
        OracleDatabaseDbSystemPropertiesElDbHomeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDbHomeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nA valid Oracle Database version. For a list of supported versions, use the\nListDbVersions operation."]
    pub fn db_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the Database Home. The name does not have to\nbe unique within your project."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `is_unified_auditing_enabled` after provisioning.\nWhether unified auditing is enabled for the Database Home."]
    pub fn is_unified_auditing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_unified_auditing_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<OracleDatabaseDbSystemPropertiesElDbHomeElDatabaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_management: Option<PrimField<String>>,
}
impl OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
    #[doc = "Set the field `storage_management`.\nThe storage option used in DB system.\nPossible values:\nASM\nLVM"]
    pub fn set_storage_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_management = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {}
impl BuildOracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
        OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl {
            storage_management: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef {
        OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_management` after provisioning.\nThe storage option used in DB system.\nPossible values:\nASM\nLVM"]
    pub fn storage_management(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_management", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesElTimeZoneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
impl OracleDatabaseDbSystemPropertiesElTimeZoneEl {
    #[doc = "Set the field `id`.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}
impl ToListMappable for OracleDatabaseDbSystemPropertiesElTimeZoneEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesElTimeZoneEl {}
impl BuildOracleDatabaseDbSystemPropertiesElTimeZoneEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesElTimeZoneEl {
        OracleDatabaseDbSystemPropertiesElTimeZoneEl {
            id: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElTimeZoneElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseDbSystemPropertiesElTimeZoneElRef {
        OracleDatabaseDbSystemPropertiesElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElTimeZoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize, Default)]
struct OracleDatabaseDbSystemPropertiesElDynamic {
    data_collection_options:
        Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl>>,
    db_home: Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElDbHomeEl>>,
    db_system_options: Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl>>,
    time_zone: Option<DynamicBlock<OracleDatabaseDbSystemPropertiesElTimeZoneEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemPropertiesEl {
    compute_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage_size_gb: Option<PrimField<f64>>,
    database_edition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_prefix: Option<PrimField<String>>,
    initial_data_storage_size_gb: PrimField<f64>,
    license_model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reco_storage_size_gb: Option<PrimField<f64>>,
    shape: PrimField<String>,
    ssh_public_keys: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_collection_options: Option<Vec<OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_home: Option<Vec<OracleDatabaseDbSystemPropertiesElDbHomeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_system_options: Option<Vec<OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<Vec<OracleDatabaseDbSystemPropertiesElTimeZoneEl>>,
    dynamic: OracleDatabaseDbSystemPropertiesElDynamic,
}
impl OracleDatabaseDbSystemPropertiesEl {
    #[doc = "Set the field `compute_model`.\nThe compute model of the DbSystem.\nPossible values:\nECPU\nOCPU"]
    pub fn set_compute_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compute_model = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_gb`.\nThe data storage size in GB that is currently available to DbSystems."]
    pub fn set_data_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `domain`.\nThe host domain name of the DbSystem."]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname_prefix`.\nPrefix for DB System host names."]
    pub fn set_hostname_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\nThe memory size in GB."]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nThe number of nodes in the DbSystem."]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ip`.\nThe private IP address of the DbSystem."]
    pub fn set_private_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `reco_storage_size_gb`.\nThe reco/redo storage size in GB."]
    pub fn set_reco_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.reco_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `data_collection_options`.\n"]
    pub fn set_data_collection_options(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElDataCollectionOptionsEl>>,
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
    #[doc = "Set the field `db_home`.\n"]
    pub fn set_db_home(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElDbHomeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.db_home = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.db_home = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `db_system_options`.\n"]
    pub fn set_db_system_options(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElDbSystemOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.db_system_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.db_system_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseDbSystemPropertiesElTimeZoneEl>>,
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
impl ToListMappable for OracleDatabaseDbSystemPropertiesEl {
    type O = BlockAssignable<OracleDatabaseDbSystemPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemPropertiesEl {
    #[doc = "The number of CPU cores to enable for the DbSystem."]
    pub compute_count: PrimField<f64>,
    #[doc = "The database edition of the DbSystem.\nPossible values:\nSTANDARD_EDITION\nENTERPRISE_EDITION\nENTERPRISE_EDITION_HIGH_PERFORMANCE"]
    pub database_edition: PrimField<String>,
    #[doc = "The initial data storage size in GB."]
    pub initial_data_storage_size_gb: PrimField<f64>,
    #[doc = "The license model of the DbSystem.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub license_model: PrimField<String>,
    #[doc = "Shape of DB System."]
    pub shape: PrimField<String>,
    #[doc = "SSH public keys to be stored with the DbSystem."]
    pub ssh_public_keys: ListField<PrimField<String>>,
}
impl BuildOracleDatabaseDbSystemPropertiesEl {
    pub fn build(self) -> OracleDatabaseDbSystemPropertiesEl {
        OracleDatabaseDbSystemPropertiesEl {
            compute_count: self.compute_count,
            compute_model: core::default::Default::default(),
            data_storage_size_gb: core::default::Default::default(),
            database_edition: self.database_edition,
            domain: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            initial_data_storage_size_gb: self.initial_data_storage_size_gb,
            license_model: self.license_model,
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            private_ip: core::default::Default::default(),
            reco_storage_size_gb: core::default::Default::default(),
            shape: self.shape,
            ssh_public_keys: self.ssh_public_keys,
            data_collection_options: core::default::Default::default(),
            db_home: core::default::Default::default(),
            db_system_options: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemPropertiesElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseDbSystemPropertiesElRef {
        OracleDatabaseDbSystemPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\nThe number of CPU cores to enable for the DbSystem."]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model of the DbSystem.\nPossible values:\nECPU\nOCPU"]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_gb` after provisioning.\nThe data storage size in GB that is currently available to DbSystems."]
    pub fn data_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_edition` after provisioning.\nThe database edition of the DbSystem.\nPossible values:\nSTANDARD_EDITION\nENTERPRISE_EDITION\nENTERPRISE_EDITION_HIGH_PERFORMANCE"]
    pub fn database_edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_edition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe host domain name of the DbSystem."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the DbSystem."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\nPrefix for DB System host names."]
    pub fn hostname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hostname_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_data_storage_size_gb` after provisioning.\nThe initial data storage size in GB."]
    pub fn initial_data_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_data_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nThe license model of the DbSystem.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\nState of the DbSystem.\nPossible values:\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMIGRATED\nMAINTENANCE_IN_PROGRESS\nNEEDS_ATTENTION\nUPGRADING"]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nThe memory size in GB."]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the DbSystem."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nOCID of the DbSystem."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\nThe private IP address of the DbSystem."]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `reco_storage_size_gb` after provisioning.\nThe reco/redo storage size in GB."]
    pub fn reco_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reco_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nShape of DB System."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }
    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\nSSH public keys to be stored with the DbSystem."]
    pub fn ssh_public_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssh_public_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(
        &self,
    ) -> ListRef<OracleDatabaseDbSystemPropertiesElDataCollectionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_collection_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_home` after provisioning.\n"]
    pub fn db_home(&self) -> ListRef<OracleDatabaseDbSystemPropertiesElDbHomeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.db_home", self.base))
    }
    #[doc = "Get a reference to the value of field `db_system_options` after provisioning.\n"]
    pub fn db_system_options(
        &self,
    ) -> ListRef<OracleDatabaseDbSystemPropertiesElDbSystemOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_system_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> ListRef<OracleDatabaseDbSystemPropertiesElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseDbSystemTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OracleDatabaseDbSystemTimeoutsEl {
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
impl ToListMappable for OracleDatabaseDbSystemTimeoutsEl {
    type O = BlockAssignable<OracleDatabaseDbSystemTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseDbSystemTimeoutsEl {}
impl BuildOracleDatabaseDbSystemTimeoutsEl {
    pub fn build(self) -> OracleDatabaseDbSystemTimeoutsEl {
        OracleDatabaseDbSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseDbSystemTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseDbSystemTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseDbSystemTimeoutsElRef {
        OracleDatabaseDbSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseDbSystemTimeoutsElRef {
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
struct OracleDatabaseDbSystemDynamic {
    properties: Option<DynamicBlock<OracleDatabaseDbSystemPropertiesEl>>,
}
