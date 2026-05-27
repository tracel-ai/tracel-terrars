use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataApphubDiscoveredServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_uri: PrimField<String>,
}
struct DataApphubDiscoveredService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApphubDiscoveredServiceData>,
}
#[derive(Clone)]
pub struct DataApphubDiscoveredService(Rc<DataApphubDiscoveredService_>);
impl DataApphubDiscoveredService {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_properties` after provisioning.\n"]
    pub fn service_properties(&self) -> ListRef<DataApphubDiscoveredServiceServicePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_reference` after provisioning.\n"]
    pub fn service_reference(&self) -> ListRef<DataApphubDiscoveredServiceServiceReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_uri` after provisioning.\n"]
    pub fn service_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_uri", self.extract_ref()),
        )
    }
}
impl Referable for DataApphubDiscoveredService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataApphubDiscoveredService {}
impl ToListMappable for DataApphubDiscoveredService {
    type O = ListRef<DataApphubDiscoveredServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataApphubDiscoveredService_ {
    fn extract_datasource_type(&self) -> String {
        "google_apphub_discovered_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataApphubDiscoveredService {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub service_uri: PrimField<String>,
}
impl BuildDataApphubDiscoveredService {
    pub fn build(self, stack: &mut Stack) -> DataApphubDiscoveredService {
        let out = DataApphubDiscoveredService(Rc::new(DataApphubDiscoveredService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApphubDiscoveredServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_uri: self.service_uri,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataApphubDiscoveredServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataApphubDiscoveredServiceRef {
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_properties` after provisioning.\n"]
    pub fn service_properties(&self) -> ListRef<DataApphubDiscoveredServiceServicePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_reference` after provisioning.\n"]
    pub fn service_reference(&self) -> ListRef<DataApphubDiscoveredServiceServiceReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_uri` after provisioning.\n"]
    pub fn service_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_uri", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataApphubDiscoveredServiceServicePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataApphubDiscoveredServiceServicePropertiesEl {
    #[doc = "Set the field `gcp_project`.\n"]
    pub fn set_gcp_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_project = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubDiscoveredServiceServicePropertiesEl {
    type O = BlockAssignable<DataApphubDiscoveredServiceServicePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubDiscoveredServiceServicePropertiesEl {}
impl BuildDataApphubDiscoveredServiceServicePropertiesEl {
    pub fn build(self) -> DataApphubDiscoveredServiceServicePropertiesEl {
        DataApphubDiscoveredServiceServicePropertiesEl {
            gcp_project: core::default::Default::default(),
            location: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataApphubDiscoveredServiceServicePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredServiceServicePropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataApphubDiscoveredServiceServicePropertiesElRef {
        DataApphubDiscoveredServiceServicePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubDiscoveredServiceServicePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_project` after provisioning.\n"]
    pub fn gcp_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_project", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubDiscoveredServiceServiceReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataApphubDiscoveredServiceServiceReferenceEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubDiscoveredServiceServiceReferenceEl {
    type O = BlockAssignable<DataApphubDiscoveredServiceServiceReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubDiscoveredServiceServiceReferenceEl {}
impl BuildDataApphubDiscoveredServiceServiceReferenceEl {
    pub fn build(self) -> DataApphubDiscoveredServiceServiceReferenceEl {
        DataApphubDiscoveredServiceServiceReferenceEl {
            path: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataApphubDiscoveredServiceServiceReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredServiceServiceReferenceElRef {
    fn new(shared: StackShared, base: String) -> DataApphubDiscoveredServiceServiceReferenceElRef {
        DataApphubDiscoveredServiceServiceReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubDiscoveredServiceServiceReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
