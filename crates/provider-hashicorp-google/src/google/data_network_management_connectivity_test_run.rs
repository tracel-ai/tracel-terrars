use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetworkManagementConnectivityTestRunData {
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
}
struct DataNetworkManagementConnectivityTestRun_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkManagementConnectivityTestRunData>,
}
#[derive(Clone)]
pub struct DataNetworkManagementConnectivityTestRun(Rc<DataNetworkManagementConnectivityTestRun_>);
impl DataNetworkManagementConnectivityTestRun {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
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
    #[doc = "Get a reference to the value of field `reachability_details` after provisioning.\nConnectivity test reachability details."]
    pub fn reachability_details(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reachability_details", self.extract_ref()),
        )
    }
}
impl Referable for DataNetworkManagementConnectivityTestRun {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetworkManagementConnectivityTestRun {}
impl ToListMappable for DataNetworkManagementConnectivityTestRun {
    type O = ListRef<DataNetworkManagementConnectivityTestRunRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetworkManagementConnectivityTestRun_ {
    fn extract_datasource_type(&self) -> String {
        "google_network_management_connectivity_test_run".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetworkManagementConnectivityTestRun {
    pub tf_id: String,
    #[doc = "Unique name for the connectivity test."]
    pub name: PrimField<String>,
}
impl BuildDataNetworkManagementConnectivityTestRun {
    pub fn build(self, stack: &mut Stack) -> DataNetworkManagementConnectivityTestRun {
        let out = DataNetworkManagementConnectivityTestRun(Rc::new(
            DataNetworkManagementConnectivityTestRun_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataNetworkManagementConnectivityTestRunData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataNetworkManagementConnectivityTestRunRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestRunRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetworkManagementConnectivityTestRunRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
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
    #[doc = "Get a reference to the value of field `reachability_details` after provisioning.\nConnectivity test reachability details."]
    pub fn reachability_details(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reachability_details", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_network_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_agent_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_network_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port: Option<PrimField<f64>>,
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl {
    #[doc = "Set the field `destination_ip`.\n"]
    pub fn set_destination_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_network_uri`.\n"]
    pub fn set_destination_network_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_network_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_port`.\n"]
    pub fn set_destination_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.destination_port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `source_agent_uri`.\n"]
    pub fn set_source_agent_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_agent_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `source_network_uri`.\n"]
    pub fn set_source_network_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_network_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `source_port`.\n"]
    pub fn set_source_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.source_port = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl
{
    type O = BlockAssignable<
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl
{}
impl BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl {
    pub fn build(
        self,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl {
            destination_ip: core::default::Default::default(),
            destination_network_uri: core::default::Default::default(),
            destination_port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_agent_uri: core::default::Default::default(),
            source_ip: core::default::Default::default(),
            source_network_uri: core::default::Default::default(),
            source_port: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef
    {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_network_uri` after provisioning.\n"]
    pub fn destination_network_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_network_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `source_agent_uri` after provisioning.\n"]
    pub fn source_agent_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_agent_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `source_network_uri` after provisioning.\n"]
    pub fn source_network_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_network_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_port` after provisioning.\n"]
    pub fn source_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    causes_drop: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {
    #[doc = "Set the field `causes_drop`.\n"]
    pub fn set_causes_drop(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.causes_drop = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl
{
    type O = BlockAssignable<
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {}
impl BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {
    pub fn build(
        self,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl {
            causes_drop: core::default::Default::default(),
            description: core::default::Default::default(),
            project_id: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `causes_drop` after provisioning.\n"]
    pub fn causes_drop(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.causes_drop", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_info: Option<
        ListField<
            DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_trace_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<
        ListField<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl>,
    >,
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
    #[doc = "Set the field `endpoint_info`.\n"]
    pub fn set_endpoint_info(
        mut self,
        v: impl Into<
            ListField<
                DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoEl,
            >,
        >,
    ) -> Self {
        self.endpoint_info = Some(v.into());
        self
    }
    #[doc = "Set the field `forward_trace_id`.\n"]
    pub fn set_forward_trace_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.forward_trace_id = Some(v.into());
        self
    }
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl Into<
            ListField<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsEl>,
        >,
    ) -> Self {
        self.steps = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
    type O = BlockAssignable<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {}
impl BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl {
            endpoint_info: core::default::Default::default(),
            forward_trace_id: core::default::Default::default(),
            steps: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_info` after provisioning.\n"]
    pub fn endpoint_info(
        &self,
    ) -> ListRef<
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElEndpointInfoElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forward_trace_id` after provisioning.\n"]
    pub fn forward_trace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forward_trace_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElStepsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traces:
        Option<ListField<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_time: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }
    #[doc = "Set the field `traces`.\n"]
    pub fn set_traces(
        mut self,
        v: impl Into<ListField<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesEl>>,
    ) -> Self {
        self.traces = Some(v.into());
        self
    }
    #[doc = "Set the field `verify_time`.\n"]
    pub fn set_verify_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verify_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
    type O = BlockAssignable<DataNetworkManagementConnectivityTestRunReachabilityDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsEl {}
impl BuildDataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
    pub fn build(self) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsEl {
            result: core::default::Default::default(),
            traces: core::default::Default::default(),
            verify_time: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef {
        DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestRunReachabilityDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }
    #[doc = "Get a reference to the value of field `traces` after provisioning.\n"]
    pub fn traces(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestRunReachabilityDetailsElTracesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traces", self.base))
    }
    #[doc = "Get a reference to the value of field `verify_time` after provisioning.\n"]
    pub fn verify_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify_time", self.base))
    }
}
