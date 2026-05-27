use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataClientOpenidUserinfoData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
}
struct DataClientOpenidUserinfo_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataClientOpenidUserinfoData>,
}
#[derive(Clone)]
pub struct DataClientOpenidUserinfo(Rc<DataClientOpenidUserinfo_>);
impl DataClientOpenidUserinfo {
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
    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the account used by the provider to authenticate with GCP."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source in Terraform state. Its value is the same as the `email` attribute. Do not use this field, use the `email` attribute instead."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
impl Referable for DataClientOpenidUserinfo {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataClientOpenidUserinfo {}
impl ToListMappable for DataClientOpenidUserinfo {
    type O = ListRef<DataClientOpenidUserinfoRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataClientOpenidUserinfo_ {
    fn extract_datasource_type(&self) -> String {
        "google_client_openid_userinfo".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataClientOpenidUserinfo {
    pub tf_id: String,
}
impl BuildDataClientOpenidUserinfo {
    pub fn build(self, stack: &mut Stack) -> DataClientOpenidUserinfo {
        let out = DataClientOpenidUserinfo(Rc::new(DataClientOpenidUserinfo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataClientOpenidUserinfoData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataClientOpenidUserinfoRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataClientOpenidUserinfoRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataClientOpenidUserinfoRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the account used by the provider to authenticate with GCP."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source in Terraform state. Its value is the same as the `email` attribute. Do not use this field, use the `email` attribute instead."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
