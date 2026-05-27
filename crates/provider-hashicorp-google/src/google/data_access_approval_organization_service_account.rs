use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAccessApprovalOrganizationServiceAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    organization_id: PrimField<String>,
}
struct DataAccessApprovalOrganizationServiceAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccessApprovalOrganizationServiceAccountData>,
}
#[derive(Clone)]
pub struct DataAccessApprovalOrganizationServiceAccount(
    Rc<DataAccessApprovalOrganizationServiceAccount_>,
);
impl DataAccessApprovalOrganizationServiceAccount {
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
    #[doc = "Get a reference to the value of field `account_email` after provisioning.\n"]
    pub fn account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\n"]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
}
impl Referable for DataAccessApprovalOrganizationServiceAccount {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAccessApprovalOrganizationServiceAccount {}
impl ToListMappable for DataAccessApprovalOrganizationServiceAccount {
    type O = ListRef<DataAccessApprovalOrganizationServiceAccountRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAccessApprovalOrganizationServiceAccount_ {
    fn extract_datasource_type(&self) -> String {
        "google_access_approval_organization_service_account".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAccessApprovalOrganizationServiceAccount {
    pub tf_id: String,
    #[doc = ""]
    pub organization_id: PrimField<String>,
}
impl BuildDataAccessApprovalOrganizationServiceAccount {
    pub fn build(self, stack: &mut Stack) -> DataAccessApprovalOrganizationServiceAccount {
        let out = DataAccessApprovalOrganizationServiceAccount(Rc::new(
            DataAccessApprovalOrganizationServiceAccount_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataAccessApprovalOrganizationServiceAccountData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    organization_id: self.organization_id,
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAccessApprovalOrganizationServiceAccountRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccessApprovalOrganizationServiceAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAccessApprovalOrganizationServiceAccountRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `account_email` after provisioning.\n"]
    pub fn account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\n"]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
}
