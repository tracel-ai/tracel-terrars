use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServiceAccountIdTokenData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegates: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_email: Option<PrimField<bool>>,
    target_audience: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_account: Option<PrimField<String>>,
}
struct DataServiceAccountIdToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServiceAccountIdTokenData>,
}
#[derive(Clone)]
pub struct DataServiceAccountIdToken(Rc<DataServiceAccountIdToken_>);
impl DataServiceAccountIdToken {
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
    #[doc = "Set the field `delegates`.\n"]
    pub fn set_delegates(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().delegates = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `include_email`.\n"]
    pub fn set_include_email(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_email = Some(v.into());
        self
    }
    #[doc = "Set the field `target_service_account`.\n"]
    pub fn set_target_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_service_account = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `delegates` after provisioning.\n"]
    pub fn delegates(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id_token` after provisioning.\n"]
    pub fn id_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.id_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_email` after provisioning.\n"]
    pub fn include_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_audience` after provisioning.\n"]
    pub fn target_audience(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_audience", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_account` after provisioning.\n"]
    pub fn target_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_service_account", self.extract_ref()),
        )
    }
}
impl Referable for DataServiceAccountIdToken {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServiceAccountIdToken {}
impl ToListMappable for DataServiceAccountIdToken {
    type O = ListRef<DataServiceAccountIdTokenRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServiceAccountIdToken_ {
    fn extract_datasource_type(&self) -> String {
        "google_service_account_id_token".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServiceAccountIdToken {
    pub tf_id: String,
    #[doc = ""]
    pub target_audience: PrimField<String>,
}
impl BuildDataServiceAccountIdToken {
    pub fn build(self, stack: &mut Stack) -> DataServiceAccountIdToken {
        let out = DataServiceAccountIdToken(Rc::new(DataServiceAccountIdToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServiceAccountIdTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                delegates: core::default::Default::default(),
                id: core::default::Default::default(),
                include_email: core::default::Default::default(),
                target_audience: self.target_audience,
                target_service_account: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServiceAccountIdTokenRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServiceAccountIdTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServiceAccountIdTokenRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `delegates` after provisioning.\n"]
    pub fn delegates(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id_token` after provisioning.\n"]
    pub fn id_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.id_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_email` after provisioning.\n"]
    pub fn include_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_audience` after provisioning.\n"]
    pub fn target_audience(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_audience", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_account` after provisioning.\n"]
    pub fn target_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_service_account", self.extract_ref()),
        )
    }
}
