use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAccessContextManagerSupportedServicesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataAccessContextManagerSupportedServices_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccessContextManagerSupportedServicesData>,
}
#[derive(Clone)]
pub struct DataAccessContextManagerSupportedServices(
    Rc<DataAccessContextManagerSupportedServices_>,
);
impl DataAccessContextManagerSupportedServices {
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
    #[doc = "Get a reference to the value of field `supported_services` after provisioning.\nThe list of VPC-SC supported services."]
    pub fn supported_services(
        &self,
    ) -> ListRef<DataAccessContextManagerSupportedServicesSupportedServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_services", self.extract_ref()),
        )
    }
}
impl Referable for DataAccessContextManagerSupportedServices {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAccessContextManagerSupportedServices {}
impl ToListMappable for DataAccessContextManagerSupportedServices {
    type O = ListRef<DataAccessContextManagerSupportedServicesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAccessContextManagerSupportedServices_ {
    fn extract_datasource_type(&self) -> String {
        "google_access_context_manager_supported_services".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAccessContextManagerSupportedServices {
    pub tf_id: String,
}
impl BuildDataAccessContextManagerSupportedServices {
    pub fn build(self, stack: &mut Stack) -> DataAccessContextManagerSupportedServices {
        let out = DataAccessContextManagerSupportedServices(Rc::new(
            DataAccessContextManagerSupportedServices_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataAccessContextManagerSupportedServicesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAccessContextManagerSupportedServicesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccessContextManagerSupportedServicesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAccessContextManagerSupportedServicesRef {
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
    #[doc = "Get a reference to the value of field `supported_services` after provisioning.\nThe list of VPC-SC supported services."]
    pub fn supported_services(
        &self,
    ) -> ListRef<DataAccessContextManagerSupportedServicesSupportedServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_services", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAccessContextManagerSupportedServicesSupportedServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    available_on_restricted_vip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    known_limitations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_support_stage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    support_stage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl DataAccessContextManagerSupportedServicesSupportedServicesEl {
    #[doc = "Set the field `available_on_restricted_vip`.\n"]
    pub fn set_available_on_restricted_vip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.available_on_restricted_vip = Some(v.into());
        self
    }
    #[doc = "Set the field `known_limitations`.\n"]
    pub fn set_known_limitations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.known_limitations = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `service_support_stage`.\n"]
    pub fn set_service_support_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_support_stage = Some(v.into());
        self
    }
    #[doc = "Set the field `support_stage`.\n"]
    pub fn set_support_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.support_stage = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for DataAccessContextManagerSupportedServicesSupportedServicesEl {
    type O = BlockAssignable<DataAccessContextManagerSupportedServicesSupportedServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAccessContextManagerSupportedServicesSupportedServicesEl {}
impl BuildDataAccessContextManagerSupportedServicesSupportedServicesEl {
    pub fn build(self) -> DataAccessContextManagerSupportedServicesSupportedServicesEl {
        DataAccessContextManagerSupportedServicesSupportedServicesEl {
            available_on_restricted_vip: core::default::Default::default(),
            known_limitations: core::default::Default::default(),
            name: core::default::Default::default(),
            service_support_stage: core::default::Default::default(),
            support_stage: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct DataAccessContextManagerSupportedServicesSupportedServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAccessContextManagerSupportedServicesSupportedServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAccessContextManagerSupportedServicesSupportedServicesElRef {
        DataAccessContextManagerSupportedServicesSupportedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAccessContextManagerSupportedServicesSupportedServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\n"]
    pub fn available_on_restricted_vip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_on_restricted_vip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `known_limitations` after provisioning.\n"]
    pub fn known_limitations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.known_limitations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_support_stage` after provisioning.\n"]
    pub fn service_support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_support_stage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `support_stage` after provisioning.\n"]
    pub fn support_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_stage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
