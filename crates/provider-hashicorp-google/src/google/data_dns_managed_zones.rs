use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDnsManagedZonesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataDnsManagedZones_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDnsManagedZonesData>,
}
#[derive(Clone)]
pub struct DataDnsManagedZones(Rc<DataDnsManagedZones_>);
impl DataDnsManagedZones {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `managed_zones` after provisioning.\n"]
    pub fn managed_zones(&self) -> ListRef<DataDnsManagedZonesManagedZonesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataDnsManagedZones {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDnsManagedZones {}
impl ToListMappable for DataDnsManagedZones {
    type O = ListRef<DataDnsManagedZonesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDnsManagedZones_ {
    fn extract_datasource_type(&self) -> String {
        "google_dns_managed_zones".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDnsManagedZones {
    pub tf_id: String,
}
impl BuildDataDnsManagedZones {
    pub fn build(self, stack: &mut Stack) -> DataDnsManagedZones {
        let out = DataDnsManagedZones(Rc::new(DataDnsManagedZones_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDnsManagedZonesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDnsManagedZonesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsManagedZonesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDnsManagedZonesRef {
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
    #[doc = "Get a reference to the value of field `managed_zones` after provisioning.\n"]
    pub fn managed_zones(&self) -> ListRef<DataDnsManagedZonesManagedZonesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDnsManagedZonesManagedZonesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_zone_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
}
impl DataDnsManagedZonesManagedZonesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_zone_id`.\n"]
    pub fn set_managed_zone_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.managed_zone_id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_servers`.\n"]
    pub fn set_name_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }
}
impl ToListMappable for DataDnsManagedZonesManagedZonesEl {
    type O = BlockAssignable<DataDnsManagedZonesManagedZonesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDnsManagedZonesManagedZonesEl {}
impl BuildDataDnsManagedZonesManagedZonesEl {
    pub fn build(self) -> DataDnsManagedZonesManagedZonesEl {
        DataDnsManagedZonesManagedZonesEl {
            description: core::default::Default::default(),
            dns_name: core::default::Default::default(),
            id: core::default::Default::default(),
            managed_zone_id: core::default::Default::default(),
            name: core::default::Default::default(),
            name_servers: core::default::Default::default(),
            project: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}
pub struct DataDnsManagedZonesManagedZonesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsManagedZonesManagedZonesElRef {
    fn new(shared: StackShared, base: String) -> DataDnsManagedZonesManagedZonesElRef {
        DataDnsManagedZonesManagedZonesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDnsManagedZonesManagedZonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `managed_zone_id` after provisioning.\n"]
    pub fn managed_zone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\n"]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}
