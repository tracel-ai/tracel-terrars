use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeNetworkEndpointGroupsData {
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
    zone: Option<PrimField<String>>,
}
struct DataComputeNetworkEndpointGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeNetworkEndpointGroupsData>,
}
#[derive(Clone)]
pub struct DataComputeNetworkEndpointGroups(Rc<DataComputeNetworkEndpointGroups_>);
impl DataComputeNetworkEndpointGroups {
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
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
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
    #[doc = "Get a reference to the value of field `network_endpoint_groups` after provisioning.\n"]
    pub fn network_endpoint_groups(
        &self,
    ) -> ListRef<DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_endpoint_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeNetworkEndpointGroups {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeNetworkEndpointGroups {}
impl ToListMappable for DataComputeNetworkEndpointGroups {
    type O = ListRef<DataComputeNetworkEndpointGroupsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeNetworkEndpointGroups_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_network_endpoint_groups".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeNetworkEndpointGroups {
    pub tf_id: String,
}
impl BuildDataComputeNetworkEndpointGroups {
    pub fn build(self, stack: &mut Stack) -> DataComputeNetworkEndpointGroups {
        let out = DataComputeNetworkEndpointGroups(Rc::new(DataComputeNetworkEndpointGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeNetworkEndpointGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeNetworkEndpointGroupsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeNetworkEndpointGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeNetworkEndpointGroupsRef {
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
    #[doc = "Get a reference to the value of field `network_endpoint_groups` after provisioning.\n"]
    pub fn network_endpoint_groups(
        &self,
    ) -> ListRef<DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_endpoint_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generated_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_endpoint_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
    #[doc = "Set the field `default_port`.\n"]
    pub fn set_default_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_port = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `generated_id`.\n"]
    pub fn set_generated_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generated_id = Some(v.into());
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
    #[doc = "Set the field `network_endpoint_type`.\n"]
    pub fn set_network_endpoint_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_endpoint_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
    type O = BlockAssignable<DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {}
impl BuildDataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
    pub fn build(self) -> DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
        DataComputeNetworkEndpointGroupsNetworkEndpointGroupsEl {
            default_port: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            generated_id: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            network_endpoint_type: core::default::Default::default(),
            project: core::default::Default::default(),
            self_link: core::default::Default::default(),
            size: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef {
        DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeNetworkEndpointGroupsNetworkEndpointGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_port` after provisioning.\n"]
    pub fn default_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_port", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `generated_id` after provisioning.\n"]
    pub fn generated_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generated_id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_endpoint_type` after provisioning.\n"]
    pub fn network_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_endpoint_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
