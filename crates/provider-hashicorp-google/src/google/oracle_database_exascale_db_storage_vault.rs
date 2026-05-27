use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OracleDatabaseExascaleDbStorageVaultData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    exascale_db_storage_vault_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_oracle_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<OracleDatabaseExascaleDbStorageVaultPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OracleDatabaseExascaleDbStorageVaultTimeoutsEl>,
    dynamic: OracleDatabaseExascaleDbStorageVaultDynamic,
}
struct OracleDatabaseExascaleDbStorageVault_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OracleDatabaseExascaleDbStorageVaultData>,
}
#[derive(Clone)]
pub struct OracleDatabaseExascaleDbStorageVault(Rc<OracleDatabaseExascaleDbStorageVault_>);
impl OracleDatabaseExascaleDbStorageVault {
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
    #[doc = "Set the field `gcp_oracle_zone`.\nThe GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn set_gcp_oracle_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gcp_oracle_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels or tags associated with the ExascaleDbStorageVault.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
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
        v: impl Into<BlockAssignable<OracleDatabaseExascaleDbStorageVaultPropertiesEl>>,
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
    pub fn set_timeouts(
        self,
        v: impl Into<OracleDatabaseExascaleDbStorageVaultTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time when the ExascaleDbStorageVault was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExascaleDbStorageVault. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nExascaleDbStorageVault."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exascale_db_storage_vault_id` after provisioning.\nThe ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exascale_db_storage_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exascale_db_storage_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExascaleDbStorageVault.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the ExascaleDbStorageVault.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    pub fn properties(&self) -> ListRef<OracleDatabaseExascaleDbStorageVaultPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
        OracleDatabaseExascaleDbStorageVaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OracleDatabaseExascaleDbStorageVault {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OracleDatabaseExascaleDbStorageVault {}
impl ToListMappable for OracleDatabaseExascaleDbStorageVault {
    type O = ListRef<OracleDatabaseExascaleDbStorageVaultRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OracleDatabaseExascaleDbStorageVault_ {
    fn extract_resource_type(&self) -> String {
        "google_oracle_database_exascale_db_storage_vault".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOracleDatabaseExascaleDbStorageVault {
    pub tf_id: String,
    #[doc = "The display name for the ExascaleDbStorageVault. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub display_name: PrimField<String>,
    #[doc = "The ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub exascale_db_storage_vault_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildOracleDatabaseExascaleDbStorageVault {
    pub fn build(self, stack: &mut Stack) -> OracleDatabaseExascaleDbStorageVault {
        let out =
            OracleDatabaseExascaleDbStorageVault(Rc::new(OracleDatabaseExascaleDbStorageVault_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(OracleDatabaseExascaleDbStorageVaultData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    deletion_protection: core::default::Default::default(),
                    display_name: self.display_name,
                    exascale_db_storage_vault_id: self.exascale_db_storage_vault_id,
                    gcp_oracle_zone: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
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
pub struct OracleDatabaseExascaleDbStorageVaultRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExascaleDbStorageVaultRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OracleDatabaseExascaleDbStorageVaultRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time when the ExascaleDbStorageVault was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExascaleDbStorageVault. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
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
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nExascaleDbStorageVault."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exascale_db_storage_vault_id` after provisioning.\nThe ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exascale_db_storage_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exascale_db_storage_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExascaleDbStorageVault.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the ExascaleDbStorageVault.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    pub fn properties(&self) -> ListRef<OracleDatabaseExascaleDbStorageVaultPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
        OracleDatabaseExascaleDbStorageVaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
    total_size_gbs: PrimField<f64>,
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {}
impl ToListMappable for OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
    type O =
        BlockAssignable<OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
    #[doc = "The total storage allocation for the ExascaleDbStorageVault, in gigabytes\n(GB)."]
    pub total_size_gbs: PrimField<f64>,
}
impl BuildOracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
    pub fn build(
        self,
    ) -> OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
        OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl {
            total_size_gbs: self.total_size_gbs,
        }
    }
}
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef {
        OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_size_gbs` after provisioning.\nThe available storage capacity for the ExascaleDbStorageVault, in gigabytes\n(GB)."]
    pub fn available_size_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_size_gbs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_size_gbs` after provisioning.\nThe total storage allocation for the ExascaleDbStorageVault, in gigabytes\n(GB)."]
    pub fn total_size_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_size_gbs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
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
impl ToListMappable for OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
    type O = BlockAssignable<OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {}
impl BuildOracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
    pub fn build(self) -> OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
        OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef {
        OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef {
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
#[derive(Serialize, Default)]
struct OracleDatabaseExascaleDbStorageVaultPropertiesElDynamic {
    exascale_db_storage_details: Option<
        DynamicBlock<OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl>,
    >,
    time_zone: Option<DynamicBlock<OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl>>,
}
#[derive(Serialize)]
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_flash_cache_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exascale_db_storage_details:
        Option<Vec<OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<Vec<OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl>>,
    dynamic: OracleDatabaseExascaleDbStorageVaultPropertiesElDynamic,
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesEl {
    #[doc = "Set the field `additional_flash_cache_percent`.\nThe size of additional flash cache in percentage of high capacity\ndatabase storage."]
    pub fn set_additional_flash_cache_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_flash_cache_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `exascale_db_storage_details`.\n"]
    pub fn set_exascale_db_storage_details(
        mut self,
        v: impl Into<
            BlockAssignable<
                OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exascale_db_storage_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exascale_db_storage_details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<BlockAssignable<OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneEl>>,
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
impl ToListMappable for OracleDatabaseExascaleDbStorageVaultPropertiesEl {
    type O = BlockAssignable<OracleDatabaseExascaleDbStorageVaultPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExascaleDbStorageVaultPropertiesEl {}
impl BuildOracleDatabaseExascaleDbStorageVaultPropertiesEl {
    pub fn build(self) -> OracleDatabaseExascaleDbStorageVaultPropertiesEl {
        OracleDatabaseExascaleDbStorageVaultPropertiesEl {
            additional_flash_cache_percent: core::default::Default::default(),
            exascale_db_storage_details: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OracleDatabaseExascaleDbStorageVaultPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExascaleDbStorageVaultPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OracleDatabaseExascaleDbStorageVaultPropertiesElRef {
        OracleDatabaseExascaleDbStorageVaultPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExascaleDbStorageVaultPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_flash_cache_percent` after provisioning.\nThe size of additional flash cache in percentage of high capacity\ndatabase storage."]
    pub fn additional_flash_cache_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_flash_cache_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `attached_shape_attributes` after provisioning.\nThe shape attributes of the VM clusters attached to the\nExascaleDbStorageVault."]
    pub fn attached_shape_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attached_shape_attributes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_shape_attributes` after provisioning.\nThe shape attributes available for the VM clusters to be attached to the\nExascaleDbStorageVault."]
    pub fn available_shape_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_shape_attributes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci_uri` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID for the ExascaleDbStorageVault."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the ExascaleDbStorageVault.\nPossible values:\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_cluster_count` after provisioning.\nThe number of VM clusters associated with the ExascaleDbStorageVault."]
    pub fn vm_cluster_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vm_cluster_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vm_cluster_ids` after provisioning.\nThe list of VM cluster OCIDs associated with the ExascaleDbStorageVault."]
    pub fn vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_cluster_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exascale_db_storage_details` after provisioning.\n"]
    pub fn exascale_db_storage_details(
        &self,
    ) -> ListRef<OracleDatabaseExascaleDbStorageVaultPropertiesElExascaleDbStorageDetailsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exascale_db_storage_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(
        &self,
    ) -> ListRef<OracleDatabaseExascaleDbStorageVaultPropertiesElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct OracleDatabaseExascaleDbStorageVaultTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OracleDatabaseExascaleDbStorageVaultTimeoutsEl {
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
impl ToListMappable for OracleDatabaseExascaleDbStorageVaultTimeoutsEl {
    type O = BlockAssignable<OracleDatabaseExascaleDbStorageVaultTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOracleDatabaseExascaleDbStorageVaultTimeoutsEl {}
impl BuildOracleDatabaseExascaleDbStorageVaultTimeoutsEl {
    pub fn build(self) -> OracleDatabaseExascaleDbStorageVaultTimeoutsEl {
        OracleDatabaseExascaleDbStorageVaultTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
        OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OracleDatabaseExascaleDbStorageVaultTimeoutsElRef {
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
struct OracleDatabaseExascaleDbStorageVaultDynamic {
    properties: Option<DynamicBlock<OracleDatabaseExascaleDbStorageVaultPropertiesEl>>,
}
