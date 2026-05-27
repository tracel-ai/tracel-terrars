use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAccessContextManagerSupportedServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    service_name: PrimField<String>,
}
struct DataAccessContextManagerSupportedService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccessContextManagerSupportedServiceData>,
}
#[derive(Clone)]
pub struct DataAccessContextManagerSupportedService(Rc<DataAccessContextManagerSupportedService_>);
impl DataAccessContextManagerSupportedService {
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
    #[doc = "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\nTrue if the service is available on the restricted VIP. Services on the restricted VIP typically either support VPC Service Controls or are core infrastructure services required for the functioning of Google Cloud."]
    pub fn available_on_restricted_vip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_on_restricted_vip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `known_limitations` after provisioning.\nTrue if the service is supported with some limitations. Check documentation for details."]
    pub fn known_limitations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.known_limitations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_support_stage` after provisioning.\nThe support stage of the service. Values are `GA`, `PREVIEW`, and `DEPRECATED`."]
    pub fn service_support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_support_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_stage` after provisioning.\nThe support stage of the service."]
    pub fn support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_methods` after provisioning.\nThe list of supported methods for this service."]
    pub fn supported_methods(
        &self,
    ) -> ListRef<DataAccessContextManagerSupportedServiceSupportedMethodsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_methods", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe name of the supported product, such as 'Cloud Storage'."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
}
impl Referable for DataAccessContextManagerSupportedService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAccessContextManagerSupportedService {}
impl ToListMappable for DataAccessContextManagerSupportedService {
    type O = ListRef<DataAccessContextManagerSupportedServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAccessContextManagerSupportedService_ {
    fn extract_datasource_type(&self) -> String {
        "google_access_context_manager_supported_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAccessContextManagerSupportedService {
    pub tf_id: String,
    #[doc = "The name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub service_name: PrimField<String>,
}
impl BuildDataAccessContextManagerSupportedService {
    pub fn build(self, stack: &mut Stack) -> DataAccessContextManagerSupportedService {
        let out = DataAccessContextManagerSupportedService(Rc::new(
            DataAccessContextManagerSupportedService_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataAccessContextManagerSupportedServiceData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    service_name: self.service_name,
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAccessContextManagerSupportedServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccessContextManagerSupportedServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAccessContextManagerSupportedServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\nTrue if the service is available on the restricted VIP. Services on the restricted VIP typically either support VPC Service Controls or are core infrastructure services required for the functioning of Google Cloud."]
    pub fn available_on_restricted_vip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_on_restricted_vip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `known_limitations` after provisioning.\nTrue if the service is supported with some limitations. Check documentation for details."]
    pub fn known_limitations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.known_limitations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_support_stage` after provisioning.\nThe support stage of the service. Values are `GA`, `PREVIEW`, and `DEPRECATED`."]
    pub fn service_support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_support_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_stage` after provisioning.\nThe support stage of the service."]
    pub fn support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_methods` after provisioning.\nThe list of supported methods for this service."]
    pub fn supported_methods(
        &self,
    ) -> ListRef<DataAccessContextManagerSupportedServiceSupportedMethodsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_methods", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe name of the supported product, such as 'Cloud Storage'."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAccessContextManagerSupportedServiceSupportedMethodsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl DataAccessContextManagerSupportedServiceSupportedMethodsEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }
}
impl ToListMappable for DataAccessContextManagerSupportedServiceSupportedMethodsEl {
    type O = BlockAssignable<DataAccessContextManagerSupportedServiceSupportedMethodsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAccessContextManagerSupportedServiceSupportedMethodsEl {}
impl BuildDataAccessContextManagerSupportedServiceSupportedMethodsEl {
    pub fn build(self) -> DataAccessContextManagerSupportedServiceSupportedMethodsEl {
        DataAccessContextManagerSupportedServiceSupportedMethodsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}
pub struct DataAccessContextManagerSupportedServiceSupportedMethodsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccessContextManagerSupportedServiceSupportedMethodsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAccessContextManagerSupportedServiceSupportedMethodsElRef {
        DataAccessContextManagerSupportedServiceSupportedMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAccessContextManagerSupportedServiceSupportedMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}
