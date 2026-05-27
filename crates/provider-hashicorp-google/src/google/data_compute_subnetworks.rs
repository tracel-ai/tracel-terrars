use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeSubnetworksData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeSubnetworks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeSubnetworksData>,
}
#[derive(Clone)]
pub struct DataComputeSubnetworks(Rc<DataComputeSubnetworks_>);
impl DataComputeSubnetworks {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
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
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetworks` after provisioning.\n"]
    pub fn subnetworks(&self) -> ListRef<DataComputeSubnetworksSubnetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnetworks", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeSubnetworks {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeSubnetworks {}
impl ToListMappable for DataComputeSubnetworks {
    type O = ListRef<DataComputeSubnetworksRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeSubnetworks_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_subnetworks".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeSubnetworks {
    pub tf_id: String,
}
impl BuildDataComputeSubnetworks {
    pub fn build(self, stack: &mut Stack) -> DataComputeSubnetworks {
        let out = DataComputeSubnetworks(Rc::new(DataComputeSubnetworks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeSubnetworksData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeSubnetworksRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSubnetworksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeSubnetworksRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetworks` after provisioning.\n"]
    pub fn subnetworks(&self) -> ListRef<DataComputeSubnetworksSubnetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnetworks", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSubnetworksSubnetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_google_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
impl DataComputeSubnetworksSubnetworksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_name`.\n"]
    pub fn set_network_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_self_link`.\n"]
    pub fn set_network_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ip_google_access`.\n"]
    pub fn set_private_ip_google_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.private_ip_google_access = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSubnetworksSubnetworksEl {
    type O = BlockAssignable<DataComputeSubnetworksSubnetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSubnetworksSubnetworksEl {}
impl BuildDataComputeSubnetworksSubnetworksEl {
    pub fn build(self) -> DataComputeSubnetworksSubnetworksEl {
        DataComputeSubnetworksSubnetworksEl {
            description: core::default::Default::default(),
            ip_cidr_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            network_name: core::default::Default::default(),
            network_self_link: core::default::Default::default(),
            private_ip_google_access: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSubnetworksSubnetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSubnetworksSubnetworksElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSubnetworksSubnetworksElRef {
        DataComputeSubnetworksSubnetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSubnetworksSubnetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_name` after provisioning.\n"]
    pub fn network_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_name", self.base))
    }
    #[doc = "Get a reference to the value of field `network_self_link` after provisioning.\n"]
    pub fn network_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_google_access` after provisioning.\n"]
    pub fn private_ip_google_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_google_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
