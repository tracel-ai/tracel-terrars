use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BlockchainNodeEngineBlockchainNodesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    blockchain_node_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blockchain_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ethereum_details: Option<Vec<BlockchainNodeEngineBlockchainNodesEthereumDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BlockchainNodeEngineBlockchainNodesTimeoutsEl>,
    dynamic: BlockchainNodeEngineBlockchainNodesDynamic,
}
struct BlockchainNodeEngineBlockchainNodes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BlockchainNodeEngineBlockchainNodesData>,
}
#[derive(Clone)]
pub struct BlockchainNodeEngineBlockchainNodes(Rc<BlockchainNodeEngineBlockchainNodes_>);
impl BlockchainNodeEngineBlockchainNodes {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `blockchain_type`.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn set_blockchain_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().blockchain_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `ethereum_details`.\n"]
    pub fn set_ethereum_details(
        self,
        v: impl Into<BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ethereum_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ethereum_details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BlockchainNodeEngineBlockchainNodesTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `blockchain_node_id` after provisioning.\nID of the requesting object."]
    pub fn blockchain_node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blockchain_node_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blockchain_type` after provisioning.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn blockchain_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blockchain_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_info` after provisioning.\nThe connection information through which to interact with a blockchain node."]
    pub fn connection_info(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesConnectionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the blockchain node was first created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of Blockchain Node being created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the blockchain node. e.g. projects/my-project/locations/us-central1/blockchainNodes/my-node."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the blockchain node was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ethereum_details` after provisioning.\n"]
    pub fn ethereum_details(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ethereum_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
        BlockchainNodeEngineBlockchainNodesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BlockchainNodeEngineBlockchainNodes {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BlockchainNodeEngineBlockchainNodes {}
impl ToListMappable for BlockchainNodeEngineBlockchainNodes {
    type O = ListRef<BlockchainNodeEngineBlockchainNodesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BlockchainNodeEngineBlockchainNodes_ {
    fn extract_resource_type(&self) -> String {
        "google_blockchain_node_engine_blockchain_nodes".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodes {
    pub tf_id: String,
    #[doc = "ID of the requesting object."]
    pub blockchain_node_id: PrimField<String>,
    #[doc = "Location of Blockchain Node being created."]
    pub location: PrimField<String>,
}
impl BuildBlockchainNodeEngineBlockchainNodes {
    pub fn build(self, stack: &mut Stack) -> BlockchainNodeEngineBlockchainNodes {
        let out =
            BlockchainNodeEngineBlockchainNodes(Rc::new(BlockchainNodeEngineBlockchainNodes_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(BlockchainNodeEngineBlockchainNodesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    blockchain_node_id: self.blockchain_node_id,
                    blockchain_type: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    ethereum_details: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BlockchainNodeEngineBlockchainNodesRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BlockchainNodeEngineBlockchainNodesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `blockchain_node_id` after provisioning.\nID of the requesting object."]
    pub fn blockchain_node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blockchain_node_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blockchain_type` after provisioning.\nUser-provided key-value pairs Possible values: [\"ETHEREUM\"]"]
    pub fn blockchain_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blockchain_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_info` after provisioning.\nThe connection information through which to interact with a blockchain node."]
    pub fn connection_info(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesConnectionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the blockchain node was first created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-provided key-value pairs\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of Blockchain Node being created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the blockchain node. e.g. projects/my-project/locations/us-central1/blockchainNodes/my-node."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the blockchain node was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ethereum_details` after provisioning.\n"]
    pub fn ethereum_details(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ethereum_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
        BlockchainNodeEngineBlockchainNodesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_rpc_api_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    websockets_api_endpoint: Option<PrimField<String>>,
}
impl BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
    #[doc = "Set the field `json_rpc_api_endpoint`.\n"]
    pub fn set_json_rpc_api_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_rpc_api_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `websockets_api_endpoint`.\n"]
    pub fn set_websockets_api_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.websockets_api_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {}
impl BuildBlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
        BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl {
            json_rpc_api_endpoint: core::default::Default::default(),
            websockets_api_endpoint: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef {
        BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_rpc_api_endpoint` after provisioning.\n"]
    pub fn json_rpc_api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_rpc_api_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `websockets_api_endpoint` after provisioning.\n"]
    pub fn websockets_api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.websockets_api_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesConnectionInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_info:
        Option<ListField<BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl BlockchainNodeEngineBlockchainNodesConnectionInfoEl {
    #[doc = "Set the field `endpoint_info`.\n"]
    pub fn set_endpoint_info(
        mut self,
        v: impl Into<ListField<BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoEl>>,
    ) -> Self {
        self.endpoint_info = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesConnectionInfoEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesConnectionInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesConnectionInfoEl {}
impl BuildBlockchainNodeEngineBlockchainNodesConnectionInfoEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesConnectionInfoEl {
        BlockchainNodeEngineBlockchainNodesConnectionInfoEl {
            endpoint_info: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesConnectionInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesConnectionInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesConnectionInfoElRef {
        BlockchainNodeEngineBlockchainNodesConnectionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesConnectionInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_info` after provisioning.\n"]
    pub fn endpoint_info(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesConnectionInfoElEndpointInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    beacon_api_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beacon_prometheus_metrics_api_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_client_prometheus_metrics_api_endpoint: Option<PrimField<String>>,
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
    #[doc = "Set the field `beacon_api_endpoint`.\n"]
    pub fn set_beacon_api_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.beacon_api_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `beacon_prometheus_metrics_api_endpoint`.\n"]
    pub fn set_beacon_prometheus_metrics_api_endpoint(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.beacon_prometheus_metrics_api_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_client_prometheus_metrics_api_endpoint`.\n"]
    pub fn set_execution_client_prometheus_metrics_api_endpoint(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.execution_client_prometheus_metrics_api_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
    type O =
        BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {}
impl BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
    pub fn build(
        self,
    ) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsEl {
            beacon_api_endpoint: core::default::Default::default(),
            beacon_prometheus_metrics_api_endpoint: core::default::Default::default(),
            execution_client_prometheus_metrics_api_endpoint: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `beacon_api_endpoint` after provisioning.\n"]
    pub fn beacon_api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beacon_api_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `beacon_prometheus_metrics_api_endpoint` after provisioning.\n"]
    pub fn beacon_prometheus_metrics_api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beacon_prometheus_metrics_api_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_client_prometheus_metrics_api_endpoint` after provisioning.\n"]
    pub fn execution_client_prometheus_metrics_api_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.execution_client_prometheus_metrics_api_endpoint",
                self.base
            ),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    garbage_collection_mode: Option<PrimField<String>>,
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
    #[doc = "Set the field `garbage_collection_mode`.\nBlockchain garbage collection modes. Only applicable when NodeType is FULL or ARCHIVE. Possible values: [\"FULL\", \"ARCHIVE\"]"]
    pub fn set_garbage_collection_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.garbage_collection_mode = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {}
impl BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl {
            garbage_collection_mode: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `garbage_collection_mode` after provisioning.\nBlockchain garbage collection modes. Only applicable when NodeType is FULL or ARCHIVE. Possible values: [\"FULL\", \"ARCHIVE\"]"]
    pub fn garbage_collection_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.garbage_collection_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    beacon_fee_recipient: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mev_relay_urls: Option<ListField<PrimField<String>>>,
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
    #[doc = "Set the field `beacon_fee_recipient`.\nAn Ethereum address which the beacon client will send fee rewards to if no recipient is configured in the validator client. See https://lighthouse-book.sigmaprime.io/suggested-fee-recipient.html or https://docs.prylabs.network/docs/execution-node/fee-recipient for examples of how this is used. Note that while this is often described as \"suggested\", as we run the execution node we can trust the execution node, and therefore this is considered enforced."]
    pub fn set_beacon_fee_recipient(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.beacon_fee_recipient = Some(v.into());
        self
    }
    #[doc = "Set the field `mev_relay_urls`.\nURLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client."]
    pub fn set_mev_relay_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mev_relay_urls = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {}
impl BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl {
            beacon_fee_recipient: core::default::Default::default(),
            mev_relay_urls: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `beacon_fee_recipient` after provisioning.\nAn Ethereum address which the beacon client will send fee rewards to if no recipient is configured in the validator client. See https://lighthouse-book.sigmaprime.io/suggested-fee-recipient.html or https://docs.prylabs.network/docs/execution-node/fee-recipient for examples of how this is used. Note that while this is often described as \"suggested\", as we run the execution node we can trust the execution node, and therefore this is considered enforced."]
    pub fn beacon_fee_recipient(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beacon_fee_recipient", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mev_relay_urls` after provisioning.\nURLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client."]
    pub fn mev_relay_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mev_relay_urls", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElDynamic {
    geth_details:
        Option<DynamicBlock<BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl>>,
    validator_config:
        Option<DynamicBlock<BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl>>,
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_enable_admin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_enable_debug: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consensus_client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geth_details: Option<Vec<BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validator_config:
        Option<Vec<BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl>>,
    dynamic: BlockchainNodeEngineBlockchainNodesEthereumDetailsElDynamic,
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
    #[doc = "Set the field `api_enable_admin`.\nEnables JSON-RPC access to functions in the admin namespace. Defaults to false."]
    pub fn set_api_enable_admin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.api_enable_admin = Some(v.into());
        self
    }
    #[doc = "Set the field `api_enable_debug`.\nEnables JSON-RPC access to functions in the debug namespace. Defaults to false."]
    pub fn set_api_enable_debug(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.api_enable_debug = Some(v.into());
        self
    }
    #[doc = "Set the field `consensus_client`.\nThe consensus client Possible values: [\"CONSENSUS_CLIENT_UNSPECIFIED\", \"LIGHTHOUSE\"]"]
    pub fn set_consensus_client(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consensus_client = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_client`.\nThe execution client Possible values: [\"EXECUTION_CLIENT_UNSPECIFIED\", \"GETH\", \"ERIGON\"]"]
    pub fn set_execution_client(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_client = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe Ethereum environment being accessed. Possible values: [\"MAINNET\", \"TESTNET_GOERLI_PRATER\", \"TESTNET_SEPOLIA\"]"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type`.\nThe type of Ethereum node. Possible values: [\"LIGHT\", \"FULL\", \"ARCHIVE\"]"]
    pub fn set_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `geth_details`.\n"]
    pub fn set_geth_details(
        mut self,
        v: impl Into<BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geth_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geth_details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validator_config`.\n"]
    pub fn set_validator_config(
        mut self,
        v: impl Into<
            BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validator_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validator_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesEthereumDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsEl {}
impl BuildBlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsEl {
            api_enable_admin: core::default::Default::default(),
            api_enable_debug: core::default::Default::default(),
            consensus_client: core::default::Default::default(),
            execution_client: core::default::Default::default(),
            network: core::default::Default::default(),
            node_type: core::default::Default::default(),
            geth_details: core::default::Default::default(),
            validator_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef {
        BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesEthereumDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_endpoints` after provisioning.\nUser-provided key-value pairs"]
    pub fn additional_endpoints(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesEthereumDetailsElAdditionalEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_enable_admin` after provisioning.\nEnables JSON-RPC access to functions in the admin namespace. Defaults to false."]
    pub fn api_enable_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_enable_admin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_enable_debug` after provisioning.\nEnables JSON-RPC access to functions in the debug namespace. Defaults to false."]
    pub fn api_enable_debug(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_enable_debug", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consensus_client` after provisioning.\nThe consensus client Possible values: [\"CONSENSUS_CLIENT_UNSPECIFIED\", \"LIGHTHOUSE\"]"]
    pub fn consensus_client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consensus_client", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_client` after provisioning.\nThe execution client Possible values: [\"EXECUTION_CLIENT_UNSPECIFIED\", \"GETH\", \"ERIGON\"]"]
    pub fn execution_client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_client", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe Ethereum environment being accessed. Possible values: [\"MAINNET\", \"TESTNET_GOERLI_PRATER\", \"TESTNET_SEPOLIA\"]"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nThe type of Ethereum node. Possible values: [\"LIGHT\", \"FULL\", \"ARCHIVE\"]"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
    #[doc = "Get a reference to the value of field `geth_details` after provisioning.\n"]
    pub fn geth_details(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesEthereumDetailsElGethDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geth_details", self.base))
    }
    #[doc = "Get a reference to the value of field `validator_config` after provisioning.\n"]
    pub fn validator_config(
        &self,
    ) -> ListRef<BlockchainNodeEngineBlockchainNodesEthereumDetailsElValidatorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validator_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BlockchainNodeEngineBlockchainNodesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BlockchainNodeEngineBlockchainNodesTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BlockchainNodeEngineBlockchainNodesTimeoutsEl {
    type O = BlockAssignable<BlockchainNodeEngineBlockchainNodesTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBlockchainNodeEngineBlockchainNodesTimeoutsEl {}
impl BuildBlockchainNodeEngineBlockchainNodesTimeoutsEl {
    pub fn build(self) -> BlockchainNodeEngineBlockchainNodesTimeoutsEl {
        BlockchainNodeEngineBlockchainNodesTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
        BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BlockchainNodeEngineBlockchainNodesTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BlockchainNodeEngineBlockchainNodesDynamic {
    ethereum_details: Option<DynamicBlock<BlockchainNodeEngineBlockchainNodesEthereumDetailsEl>>,
}
