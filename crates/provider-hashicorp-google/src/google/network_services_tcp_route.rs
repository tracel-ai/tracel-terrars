use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesTcpRouteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<NetworkServicesTcpRouteRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesTcpRouteTimeoutsEl>,
    dynamic: NetworkServicesTcpRouteDynamic,
}
struct NetworkServicesTcpRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesTcpRouteData>,
}
#[derive(Clone)]
pub struct NetworkServicesTcpRoute(Rc<NetworkServicesTcpRoute_>);
impl NetworkServicesTcpRoute {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `gateways`.\nGateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn set_gateways(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gateways = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `meshes`.\nMeshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn set_meshes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().meshes = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<NetworkServicesTcpRouteRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesTcpRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the TcpRoute resource."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesTcpRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesTcpRouteTimeoutsElRef {
        NetworkServicesTcpRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesTcpRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesTcpRoute {}
impl ToListMappable for NetworkServicesTcpRoute {
    type O = ListRef<NetworkServicesTcpRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesTcpRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_tcp_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesTcpRoute {
    pub tf_id: String,
    #[doc = "Name of the TcpRoute resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesTcpRoute {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesTcpRoute {
        let out = NetworkServicesTcpRoute(Rc::new(NetworkServicesTcpRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesTcpRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                gateways: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                meshes: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesTcpRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesTcpRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the TcpRoute resource."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesTcpRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesTcpRouteTimeoutsElRef {
        NetworkServicesTcpRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTcpRouteRulesElActionElDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl NetworkServicesTcpRouteRulesElActionElDestinationsEl {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesTcpRouteRulesElActionElDestinationsEl {
    type O = BlockAssignable<NetworkServicesTcpRouteRulesElActionElDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTcpRouteRulesElActionElDestinationsEl {}
impl BuildNetworkServicesTcpRouteRulesElActionElDestinationsEl {
    pub fn build(self) -> NetworkServicesTcpRouteRulesElActionElDestinationsEl {
        NetworkServicesTcpRouteRulesElActionElDestinationsEl {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesTcpRouteRulesElActionElDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteRulesElActionElDestinationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesTcpRouteRulesElActionElDestinationsElRef {
        NetworkServicesTcpRouteRulesElActionElDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTcpRouteRulesElActionElDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesTcpRouteRulesElActionElDynamic {
    destinations: Option<DynamicBlock<NetworkServicesTcpRouteRulesElActionElDestinationsEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesTcpRouteRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_destination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<NetworkServicesTcpRouteRulesElActionElDestinationsEl>>,
    dynamic: NetworkServicesTcpRouteRulesElActionElDynamic,
}
impl NetworkServicesTcpRouteRulesElActionEl {
    #[doc = "Set the field `idle_timeout`.\nSpecifies the idle timeout for the selected route. The idle timeout is defined as the period in which there are no bytes sent or received on either the upstream or downstream connection. If not set, the default idle timeout is 30 seconds. If set to 0s, the timeout will be disabled.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_idle_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `original_destination`.\nIf true, Router will use the destination IP and port of the original connection as the destination of the request."]
    pub fn set_original_destination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.original_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesTcpRouteRulesElActionElDestinationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destinations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesTcpRouteRulesElActionEl {
    type O = BlockAssignable<NetworkServicesTcpRouteRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTcpRouteRulesElActionEl {}
impl BuildNetworkServicesTcpRouteRulesElActionEl {
    pub fn build(self) -> NetworkServicesTcpRouteRulesElActionEl {
        NetworkServicesTcpRouteRulesElActionEl {
            idle_timeout: core::default::Default::default(),
            original_destination: core::default::Default::default(),
            destinations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesTcpRouteRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTcpRouteRulesElActionElRef {
        NetworkServicesTcpRouteRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTcpRouteRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\nSpecifies the idle timeout for the selected route. The idle timeout is defined as the period in which there are no bytes sent or received on either the upstream or downstream connection. If not set, the default idle timeout is 30 seconds. If set to 0s, the timeout will be disabled.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn idle_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `original_destination` after provisioning.\nIf true, Router will use the destination IP and port of the original connection as the destination of the request."]
    pub fn original_destination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<NetworkServicesTcpRouteRulesElActionElDestinationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTcpRouteRulesElMatchesEl {
    address: PrimField<String>,
    port: PrimField<String>,
}
impl NetworkServicesTcpRouteRulesElMatchesEl {}
impl ToListMappable for NetworkServicesTcpRouteRulesElMatchesEl {
    type O = BlockAssignable<NetworkServicesTcpRouteRulesElMatchesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTcpRouteRulesElMatchesEl {
    #[doc = "Must be specified in the CIDR range format. A CIDR range consists of an IP Address and a prefix length to construct the subnet mask.\nBy default, the prefix length is 32 (i.e. matches a single IP address). Only IPV4 addresses are supported. Examples: \"10.0.0.1\" - matches against this exact IP address. \"10.0.0.0/8\" - matches against any IP address within the 10.0.0.0 subnet and 255.255.255.0 mask. \"0.0.0.0/0\" - matches against any IP address'."]
    pub address: PrimField<String>,
    #[doc = "Specifies the destination port to match against."]
    pub port: PrimField<String>,
}
impl BuildNetworkServicesTcpRouteRulesElMatchesEl {
    pub fn build(self) -> NetworkServicesTcpRouteRulesElMatchesEl {
        NetworkServicesTcpRouteRulesElMatchesEl {
            address: self.address,
            port: self.port,
        }
    }
}
pub struct NetworkServicesTcpRouteRulesElMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteRulesElMatchesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTcpRouteRulesElMatchesElRef {
        NetworkServicesTcpRouteRulesElMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTcpRouteRulesElMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nMust be specified in the CIDR range format. A CIDR range consists of an IP Address and a prefix length to construct the subnet mask.\nBy default, the prefix length is 32 (i.e. matches a single IP address). Only IPV4 addresses are supported. Examples: \"10.0.0.1\" - matches against this exact IP address. \"10.0.0.0/8\" - matches against any IP address within the 10.0.0.0 subnet and 255.255.255.0 mask. \"0.0.0.0/0\" - matches against any IP address'."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nSpecifies the destination port to match against."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesTcpRouteRulesElDynamic {
    action: Option<DynamicBlock<NetworkServicesTcpRouteRulesElActionEl>>,
    matches: Option<DynamicBlock<NetworkServicesTcpRouteRulesElMatchesEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesTcpRouteRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<NetworkServicesTcpRouteRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<NetworkServicesTcpRouteRulesElMatchesEl>>,
    dynamic: NetworkServicesTcpRouteRulesElDynamic,
}
impl NetworkServicesTcpRouteRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesTcpRouteRulesElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `matches`.\n"]
    pub fn set_matches(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesTcpRouteRulesElMatchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.matches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.matches = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesTcpRouteRulesEl {
    type O = BlockAssignable<NetworkServicesTcpRouteRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTcpRouteRulesEl {}
impl BuildNetworkServicesTcpRouteRulesEl {
    pub fn build(self) -> NetworkServicesTcpRouteRulesEl {
        NetworkServicesTcpRouteRulesEl {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesTcpRouteRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTcpRouteRulesElRef {
        NetworkServicesTcpRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTcpRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<NetworkServicesTcpRouteRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> ListRef<NetworkServicesTcpRouteRulesElMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTcpRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesTcpRouteTimeoutsEl {
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
impl ToListMappable for NetworkServicesTcpRouteTimeoutsEl {
    type O = BlockAssignable<NetworkServicesTcpRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTcpRouteTimeoutsEl {}
impl BuildNetworkServicesTcpRouteTimeoutsEl {
    pub fn build(self) -> NetworkServicesTcpRouteTimeoutsEl {
        NetworkServicesTcpRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesTcpRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTcpRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTcpRouteTimeoutsElRef {
        NetworkServicesTcpRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTcpRouteTimeoutsElRef {
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
struct NetworkServicesTcpRouteDynamic {
    rules: Option<DynamicBlock<NetworkServicesTcpRouteRulesEl>>,
}
