use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBillingAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_projects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open: Option<PrimField<bool>>,
}
struct DataBillingAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBillingAccountData>,
}
#[derive(Clone)]
pub struct DataBillingAccount(Rc<DataBillingAccount_>);
impl DataBillingAccount {
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
    #[doc = "Set the field `billing_account`.\n"]
    pub fn set_billing_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_account = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `lookup_projects`.\n"]
    pub fn set_lookup_projects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lookup_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `open`.\n"]
    pub fn set_open(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().open = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\n"]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.currency_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `lookup_projects` after provisioning.\n"]
    pub fn lookup_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lookup_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open` after provisioning.\n"]
    pub fn open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_ids` after provisioning.\n"]
    pub fn project_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.project_ids", self.extract_ref()),
        )
    }
}
impl Referable for DataBillingAccount {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBillingAccount {}
impl ToListMappable for DataBillingAccount {
    type O = ListRef<DataBillingAccountRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBillingAccount_ {
    fn extract_datasource_type(&self) -> String {
        "google_billing_account".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBillingAccount {
    pub tf_id: String,
}
impl BuildDataBillingAccount {
    pub fn build(self, stack: &mut Stack) -> DataBillingAccount {
        let out = DataBillingAccount(Rc::new(DataBillingAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBillingAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                billing_account: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                lookup_projects: core::default::Default::default(),
                open: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBillingAccountRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBillingAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBillingAccountRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\n"]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.currency_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `lookup_projects` after provisioning.\n"]
    pub fn lookup_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lookup_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open` after provisioning.\n"]
    pub fn open(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_ids` after provisioning.\n"]
    pub fn project_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.project_ids", self.extract_ref()),
        )
    }
}
