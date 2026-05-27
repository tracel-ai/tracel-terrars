use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServiceAccountAccessTokenData {
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
    lifetime: Option<PrimField<String>>,
    scopes: SetField<PrimField<String>>,
    target_service_account: PrimField<String>,
}
struct DataServiceAccountAccessToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServiceAccountAccessTokenData>,
}
#[derive(Clone)]
pub struct DataServiceAccountAccessToken(Rc<DataServiceAccountAccessToken_>);
impl DataServiceAccountAccessToken {
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
    #[doc = "Set the field `lifetime`.\n"]
    pub fn set_lifetime(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lifetime = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_token", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\n"]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
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
impl Referable for DataServiceAccountAccessToken {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServiceAccountAccessToken {}
impl ToListMappable for DataServiceAccountAccessToken {
    type O = ListRef<DataServiceAccountAccessTokenRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServiceAccountAccessToken_ {
    fn extract_datasource_type(&self) -> String {
        "google_service_account_access_token".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServiceAccountAccessToken {
    pub tf_id: String,
    #[doc = ""]
    pub scopes: SetField<PrimField<String>>,
    #[doc = ""]
    pub target_service_account: PrimField<String>,
}
impl BuildDataServiceAccountAccessToken {
    pub fn build(self, stack: &mut Stack) -> DataServiceAccountAccessToken {
        let out = DataServiceAccountAccessToken(Rc::new(DataServiceAccountAccessToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServiceAccountAccessTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                delegates: core::default::Default::default(),
                id: core::default::Default::default(),
                lifetime: core::default::Default::default(),
                scopes: self.scopes,
                target_service_account: self.target_service_account,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServiceAccountAccessTokenRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServiceAccountAccessTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServiceAccountAccessTokenRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_token", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\n"]
    pub fn lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
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
