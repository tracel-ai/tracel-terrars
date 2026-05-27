use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServiceAccountJwtData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegates: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    payload: PrimField<String>,
    target_service_account: PrimField<String>,
}
struct DataServiceAccountJwt_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServiceAccountJwtData>,
}
#[derive(Clone)]
pub struct DataServiceAccountJwt(Rc<DataServiceAccountJwt_>);
impl DataServiceAccountJwt {
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
    #[doc = "Set the field `expires_in`.\nNumber of seconds until the JWT expires. If set and non-zero an `exp` claim will be added to the payload derived from the current timestamp plus expires_in seconds."]
    pub fn set_expires_in(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expires_in = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `delegates` after provisioning.\n"]
    pub fn delegates(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expires_in` after provisioning.\nNumber of seconds until the JWT expires. If set and non-zero an `exp` claim will be added to the payload derived from the current timestamp plus expires_in seconds."]
    pub fn expires_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expires_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `jwt` after provisioning.\n"]
    pub fn jwt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA JSON-encoded JWT claims set that will be included in the signed JWT."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload", self.extract_ref()),
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
impl Referable for DataServiceAccountJwt {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServiceAccountJwt {}
impl ToListMappable for DataServiceAccountJwt {
    type O = ListRef<DataServiceAccountJwtRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServiceAccountJwt_ {
    fn extract_datasource_type(&self) -> String {
        "google_service_account_jwt".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServiceAccountJwt {
    pub tf_id: String,
    #[doc = "A JSON-encoded JWT claims set that will be included in the signed JWT."]
    pub payload: PrimField<String>,
    #[doc = ""]
    pub target_service_account: PrimField<String>,
}
impl BuildDataServiceAccountJwt {
    pub fn build(self, stack: &mut Stack) -> DataServiceAccountJwt {
        let out = DataServiceAccountJwt(Rc::new(DataServiceAccountJwt_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServiceAccountJwtData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                delegates: core::default::Default::default(),
                expires_in: core::default::Default::default(),
                id: core::default::Default::default(),
                payload: self.payload,
                target_service_account: self.target_service_account,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServiceAccountJwtRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServiceAccountJwtRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServiceAccountJwtRef {
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
    #[doc = "Get a reference to the value of field `expires_in` after provisioning.\nNumber of seconds until the JWT expires. If set and non-zero an `exp` claim will be added to the payload derived from the current timestamp plus expires_in seconds."]
    pub fn expires_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expires_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `jwt` after provisioning.\n"]
    pub fn jwt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA JSON-encoded JWT claims set that will be included in the signed JWT."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload", self.extract_ref()),
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
