use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineNsxCredentialsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
}
struct DataVmwareengineNsxCredentials_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineNsxCredentialsData>,
}
#[derive(Clone)]
pub struct DataVmwareengineNsxCredentials(Rc<DataVmwareengineNsxCredentials_>);
impl DataVmwareengineNsxCredentials {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud which contains NSX.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nInitial password."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nInitial username."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
}
impl Referable for DataVmwareengineNsxCredentials {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineNsxCredentials {}
impl ToListMappable for DataVmwareengineNsxCredentials {
    type O = ListRef<DataVmwareengineNsxCredentialsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineNsxCredentials_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_nsx_credentials".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineNsxCredentials {
    pub tf_id: String,
    #[doc = "The resource name of the private cloud which contains NSX.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}
impl BuildDataVmwareengineNsxCredentials {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineNsxCredentials {
        let out = DataVmwareengineNsxCredentials(Rc::new(DataVmwareengineNsxCredentials_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineNsxCredentialsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineNsxCredentialsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineNsxCredentialsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineNsxCredentialsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud which contains NSX.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nInitial password."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nInitial username."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
}
