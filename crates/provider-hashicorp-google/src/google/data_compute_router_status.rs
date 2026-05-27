use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRouterStatusData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeRouterStatus_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRouterStatusData>,
}
#[derive(Clone)]
pub struct DataComputeRouterStatus(Rc<DataComputeRouterStatus_>);
impl DataComputeRouterStatus {
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
    #[doc = "Set the field `project`.\nProject ID of the target router."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion of the target router."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> ListRef<DataComputeRouterStatusBestRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.best_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(
        &self,
    ) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.best_routes_for_router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRouterStatus {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRouterStatus {}
impl ToListMappable for DataComputeRouterStatus {
    type O = ListRef<DataComputeRouterStatusRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRouterStatus_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_router_status".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRouterStatus {
    pub tf_id: String,
    #[doc = "Name of the router to query."]
    pub name: PrimField<String>,
}
impl BuildDataComputeRouterStatus {
    pub fn build(self, stack: &mut Stack) -> DataComputeRouterStatus {
        let out = DataComputeRouterStatus(Rc::new(DataComputeRouterStatus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRouterStatusData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRouterStatusRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRouterStatusRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> ListRef<DataComputeRouterStatusBestRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.best_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(
        &self,
    ) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.best_routes_for_router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesElAsPathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    as_lists: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_segment_type: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesElAsPathsEl {
    #[doc = "Set the field `as_lists`.\n"]
    pub fn set_as_lists(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.as_lists = Some(v.into());
        self
    }
    #[doc = "Set the field `path_segment_type`.\n"]
    pub fn set_path_segment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_segment_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesElAsPathsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesElAsPathsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesElAsPathsEl {}
impl BuildDataComputeRouterStatusBestRoutesElAsPathsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesElAsPathsEl {
        DataComputeRouterStatusBestRoutesElAsPathsEl {
            as_lists: core::default::Default::default(),
            path_segment_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesElAsPathsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesElAsPathsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesElAsPathsElRef {
        DataComputeRouterStatusBestRoutesElAsPathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesElAsPathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_lists` after provisioning.\n"]
    pub fn as_lists(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.as_lists", self.base))
    }
    #[doc = "Get a reference to the value of field `path_segment_type` after provisioning.\n"]
    pub fn path_segment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_segment_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesElParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeRouterStatusBestRoutesElParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesElParamsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesElParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesElParamsEl {}
impl BuildDataComputeRouterStatusBestRoutesElParamsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesElParamsEl {
        DataComputeRouterStatusBestRoutesElParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesElParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesElParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesElParamsElRef {
        DataComputeRouterStatusBestRoutesElParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesElParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesElWarningsElDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesElWarningsElDataEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesElWarningsElDataEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesElWarningsElDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesElWarningsElDataEl {}
impl BuildDataComputeRouterStatusBestRoutesElWarningsElDataEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesElWarningsElDataEl {
        DataComputeRouterStatusBestRoutesElWarningsElDataEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesElWarningsElDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesElWarningsElDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRouterStatusBestRoutesElWarningsElDataElRef {
        DataComputeRouterStatusBestRoutesElWarningsElDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesElWarningsElDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesElWarningsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<ListField<DataComputeRouterStatusBestRoutesElWarningsElDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesElWarningsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesElWarningsElDataEl>>,
    ) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesElWarningsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesElWarningsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesElWarningsEl {}
impl BuildDataComputeRouterStatusBestRoutesElWarningsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesElWarningsEl {
        DataComputeRouterStatusBestRoutesElWarningsEl {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesElWarningsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesElWarningsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesElWarningsElRef {
        DataComputeRouterStatusBestRoutesElWarningsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesElWarningsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DataComputeRouterStatusBestRoutesElWarningsElDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    as_paths: Option<ListField<DataComputeRouterStatusBestRoutesElAsPathsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_hub: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_inter_region_cost: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_med: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_peering: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<ListField<DataComputeRouterStatusBestRoutesElParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warnings: Option<ListField<DataComputeRouterStatusBestRoutesElWarningsEl>>,
}
impl DataComputeRouterStatusBestRoutesEl {
    #[doc = "Set the field `as_paths`.\n"]
    pub fn set_as_paths(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesElAsPathsEl>>,
    ) -> Self {
        self.as_paths = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_timestamp = Some(v.into());
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
    #[doc = "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest_range = Some(v.into());
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
    #[doc = "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_hub`.\n"]
    pub fn set_next_hop_hub(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_hub = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ilb = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_inter_region_cost`.\n"]
    pub fn set_next_hop_inter_region_cost(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_inter_region_cost = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_med`.\n"]
    pub fn set_next_hop_med(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_med = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_network = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_origin`.\n"]
    pub fn set_next_hop_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_origin = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_peering`.\n"]
    pub fn set_next_hop_peering(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_peering = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_vpn_tunnel = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesElParamsEl>>,
    ) -> Self {
        self.params = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `route_status`.\n"]
    pub fn set_route_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_status = Some(v.into());
        self
    }
    #[doc = "Set the field `route_type`.\n"]
    pub fn set_route_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_type = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `warnings`.\n"]
    pub fn set_warnings(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesElWarningsEl>>,
    ) -> Self {
        self.warnings = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesEl {}
impl BuildDataComputeRouterStatusBestRoutesEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesEl {
        DataComputeRouterStatusBestRoutesEl {
            as_paths: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_hub: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_inter_region_cost: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_med: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_origin: core::default::Default::default(),
            next_hop_peering: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            params: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            route_status: core::default::Default::default(),
            route_type: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
            warnings: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesElRef {
        DataComputeRouterStatusBestRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> ListRef<DataComputeRouterStatusBestRoutesElAsPathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.as_paths", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.base),
        )
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
    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_gateway", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\n"]
    pub fn next_hop_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_hub", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\n"]
    pub fn next_hop_inter_region_cost(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_inter_region_cost", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\n"]
    pub fn next_hop_med(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_med", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\n"]
    pub fn next_hop_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_origin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\n"]
    pub fn next_hop_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_peering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_vpn_tunnel", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<DataComputeRouterStatusBestRoutesElParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `route_status` after provisioning.\n"]
    pub fn route_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_status", self.base))
    }
    #[doc = "Get a reference to the value of field `route_type` after provisioning.\n"]
    pub fn route_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_type", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\n"]
    pub fn warnings(&self) -> ListRef<DataComputeRouterStatusBestRoutesElWarningsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    as_lists: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_segment_type: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
    #[doc = "Set the field `as_lists`.\n"]
    pub fn set_as_lists(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.as_lists = Some(v.into());
        self
    }
    #[doc = "Set the field `path_segment_type`.\n"]
    pub fn set_path_segment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_segment_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterElAsPathsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesForRouterElAsPathsEl {}
impl BuildDataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
        DataComputeRouterStatusBestRoutesForRouterElAsPathsEl {
            as_lists: core::default::Default::default(),
            path_segment_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef {
        DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_lists` after provisioning.\n"]
    pub fn as_lists(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.as_lists", self.base))
    }
    #[doc = "Get a reference to the value of field `path_segment_type` after provisioning.\n"]
    pub fn path_segment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_segment_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterElParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeRouterStatusBestRoutesForRouterElParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterElParamsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterElParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesForRouterElParamsEl {}
impl BuildDataComputeRouterStatusBestRoutesForRouterElParamsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterElParamsEl {
        DataComputeRouterStatusBestRoutesForRouterElParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesForRouterElParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesForRouterElParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRouterStatusBestRoutesForRouterElParamsElRef {
        DataComputeRouterStatusBestRoutesForRouterElParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesForRouterElParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {}
impl BuildDataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
        DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef {
        DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterElWarningsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<ListField<DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DataComputeRouterStatusBestRoutesForRouterElWarningsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesForRouterElWarningsElDataEl>>,
    ) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterElWarningsEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterElWarningsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesForRouterElWarningsEl {}
impl BuildDataComputeRouterStatusBestRoutesForRouterElWarningsEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterElWarningsEl {
        DataComputeRouterStatusBestRoutesForRouterElWarningsEl {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesForRouterElWarningsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesForRouterElWarningsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRouterStatusBestRoutesForRouterElWarningsElRef {
        DataComputeRouterStatusBestRoutesForRouterElWarningsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesForRouterElWarningsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElWarningsElDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    as_paths: Option<ListField<DataComputeRouterStatusBestRoutesForRouterElAsPathsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_hub: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_inter_region_cost: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_med: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_peering: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<ListField<DataComputeRouterStatusBestRoutesForRouterElParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warnings: Option<ListField<DataComputeRouterStatusBestRoutesForRouterElWarningsEl>>,
}
impl DataComputeRouterStatusBestRoutesForRouterEl {
    #[doc = "Set the field `as_paths`.\n"]
    pub fn set_as_paths(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesForRouterElAsPathsEl>>,
    ) -> Self {
        self.as_paths = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_timestamp = Some(v.into());
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
    #[doc = "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest_range = Some(v.into());
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
    #[doc = "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_hub`.\n"]
    pub fn set_next_hop_hub(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_hub = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ilb = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_inter_region_cost`.\n"]
    pub fn set_next_hop_inter_region_cost(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_inter_region_cost = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_med`.\n"]
    pub fn set_next_hop_med(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_med = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_network = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_origin`.\n"]
    pub fn set_next_hop_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_origin = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_peering`.\n"]
    pub fn set_next_hop_peering(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_peering = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_vpn_tunnel = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesForRouterElParamsEl>>,
    ) -> Self {
        self.params = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `route_status`.\n"]
    pub fn set_route_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_status = Some(v.into());
        self
    }
    #[doc = "Set the field `route_type`.\n"]
    pub fn set_route_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_type = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `warnings`.\n"]
    pub fn set_warnings(
        mut self,
        v: impl Into<ListField<DataComputeRouterStatusBestRoutesForRouterElWarningsEl>>,
    ) -> Self {
        self.warnings = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterStatusBestRoutesForRouterEl {}
impl BuildDataComputeRouterStatusBestRoutesForRouterEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterEl {
        DataComputeRouterStatusBestRoutesForRouterEl {
            as_paths: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_hub: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_inter_region_cost: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_med: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_origin: core::default::Default::default(),
            next_hop_peering: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            params: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            route_status: core::default::Default::default(),
            route_type: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
            warnings: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterStatusBestRoutesForRouterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterStatusBestRoutesForRouterElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesForRouterElRef {
        DataComputeRouterStatusBestRoutesForRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterStatusBestRoutesForRouterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElAsPathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.as_paths", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.base),
        )
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
    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_gateway", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\n"]
    pub fn next_hop_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_hub", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_instance_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\n"]
    pub fn next_hop_inter_region_cost(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_inter_region_cost", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\n"]
    pub fn next_hop_med(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_med", self.base))
    }
    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\n"]
    pub fn next_hop_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_origin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\n"]
    pub fn next_hop_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_peering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_vpn_tunnel", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `route_status` after provisioning.\n"]
    pub fn route_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_status", self.base))
    }
    #[doc = "Get a reference to the value of field `route_type` after provisioning.\n"]
    pub fn route_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_type", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\n"]
    pub fn warnings(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElWarningsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}
