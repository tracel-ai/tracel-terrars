use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOrganizationIamCustomRoleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    role_id: PrimField<String>,
}
struct DataOrganizationIamCustomRole_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationIamCustomRoleData>,
}
#[derive(Clone)]
pub struct DataOrganizationIamCustomRole(Rc<DataOrganizationIamCustomRole_>);
impl DataOrganizationIamCustomRole {
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
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nThe current deleted state of the role."]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the role."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the role in the format organizations/{{org_id}}/roles/{{role_id}}. Like id, this field can be used as a reference in other resources such as IAM role bindings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization in which you want to create a custom role."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\nThe names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified."]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_id` after provisioning.\nThe role id to use for this role."]
    pub fn role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\nThe current launch stage of the role. Defaults to GA."]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nA human-readable title for the role."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
}
impl Referable for DataOrganizationIamCustomRole {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOrganizationIamCustomRole {}
impl ToListMappable for DataOrganizationIamCustomRole {
    type O = ListRef<DataOrganizationIamCustomRoleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOrganizationIamCustomRole_ {
    fn extract_datasource_type(&self) -> String {
        "google_organization_iam_custom_role".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOrganizationIamCustomRole {
    pub tf_id: String,
    #[doc = "The numeric ID of the organization in which you want to create a custom role."]
    pub org_id: PrimField<String>,
    #[doc = "The role id to use for this role."]
    pub role_id: PrimField<String>,
}
impl BuildDataOrganizationIamCustomRole {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationIamCustomRole {
        let out = DataOrganizationIamCustomRole(Rc::new(DataOrganizationIamCustomRole_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationIamCustomRoleData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                org_id: self.org_id,
                role_id: self.role_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOrganizationIamCustomRoleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOrganizationIamCustomRoleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOrganizationIamCustomRoleRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nThe current deleted state of the role."]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the role."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the role in the format organizations/{{org_id}}/roles/{{role_id}}. Like id, this field can be used as a reference in other resources such as IAM role bindings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization in which you want to create a custom role."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\nThe names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified."]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_id` after provisioning.\nThe role id to use for this role."]
    pub fn role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\nThe current launch stage of the role. Defaults to GA."]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nA human-readable title for the role."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
}
