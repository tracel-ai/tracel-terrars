use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeInterconnectLocationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataComputeInterconnectLocations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeInterconnectLocationsData>,
}
#[derive(Clone)]
pub struct DataComputeInterconnectLocations(Rc<DataComputeInterconnectLocations_>);
impl DataComputeInterconnectLocations {
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
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<DataComputeInterconnectLocationsLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
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
impl Referable for DataComputeInterconnectLocations {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeInterconnectLocations {}
impl ToListMappable for DataComputeInterconnectLocations {
    type O = ListRef<DataComputeInterconnectLocationsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeInterconnectLocations_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_interconnect_locations".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeInterconnectLocations {
    pub tf_id: String,
}
impl BuildDataComputeInterconnectLocations {
    pub fn build(self, stack: &mut Stack) -> DataComputeInterconnectLocations {
        let out = DataComputeInterconnectLocations(Rc::new(DataComputeInterconnectLocations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeInterconnectLocationsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeInterconnectLocationsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInterconnectLocationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeInterconnectLocationsRef {
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
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<DataComputeInterconnectLocationsLocationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
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
pub struct DataComputeInterconnectLocationsLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_features: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_link_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facility_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facility_provider_facility_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peeringdb_facility_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_pzs: Option<PrimField<bool>>,
}
impl DataComputeInterconnectLocationsLocationsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `available_features`.\n"]
    pub fn set_available_features(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.available_features = Some(v.into());
        self
    }
    #[doc = "Set the field `available_link_types`.\n"]
    pub fn set_available_link_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.available_link_types = Some(v.into());
        self
    }
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `continent`.\n"]
    pub fn set_continent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.continent = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `facility_provider`.\n"]
    pub fn set_facility_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.facility_provider = Some(v.into());
        self
    }
    #[doc = "Set the field `facility_provider_facility_id`.\n"]
    pub fn set_facility_provider_facility_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.facility_provider_facility_id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `peeringdb_facility_id`.\n"]
    pub fn set_peeringdb_facility_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peeringdb_facility_id = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `supports_pzs`.\n"]
    pub fn set_supports_pzs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.supports_pzs = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInterconnectLocationsLocationsEl {
    type O = BlockAssignable<DataComputeInterconnectLocationsLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInterconnectLocationsLocationsEl {}
impl BuildDataComputeInterconnectLocationsLocationsEl {
    pub fn build(self) -> DataComputeInterconnectLocationsLocationsEl {
        DataComputeInterconnectLocationsLocationsEl {
            address: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            available_features: core::default::Default::default(),
            available_link_types: core::default::Default::default(),
            city: core::default::Default::default(),
            continent: core::default::Default::default(),
            description: core::default::Default::default(),
            facility_provider: core::default::Default::default(),
            facility_provider_facility_id: core::default::Default::default(),
            name: core::default::Default::default(),
            peeringdb_facility_id: core::default::Default::default(),
            self_link: core::default::Default::default(),
            status: core::default::Default::default(),
            supports_pzs: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInterconnectLocationsLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInterconnectLocationsLocationsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInterconnectLocationsLocationsElRef {
        DataComputeInterconnectLocationsLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInterconnectLocationsLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_features` after provisioning.\n"]
    pub fn available_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_link_types` after provisioning.\n"]
    pub fn available_link_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_link_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `continent` after provisioning.\n"]
    pub fn continent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.continent", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `facility_provider` after provisioning.\n"]
    pub fn facility_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.facility_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `facility_provider_facility_id` after provisioning.\n"]
    pub fn facility_provider_facility_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.facility_provider_facility_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `peeringdb_facility_id` after provisioning.\n"]
    pub fn peeringdb_facility_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peeringdb_facility_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `supports_pzs` after provisioning.\n"]
    pub fn supports_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_pzs", self.base))
    }
}
