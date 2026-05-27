use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesTlsRouteData {
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
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_proxies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<NetworkServicesTlsRouteRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesTlsRouteTimeoutsEl>,
    dynamic: NetworkServicesTlsRouteDynamic,
}
struct NetworkServicesTlsRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesTlsRouteData>,
}
#[derive(Clone)]
pub struct NetworkServicesTlsRoute(Rc<NetworkServicesTlsRoute_>);
impl NetworkServicesTlsRoute {
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
    #[doc = "Set the field `gateways`.\nGateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/*/gateways/<gateway_name>"]
    pub fn set_gateways(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gateways = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nLocation (region) of the TLS Route."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `meshes`.\nMeshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/*/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn set_meshes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().meshes = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `target_proxies`.\nTargetProxies defines a list of target proxies this TlsRoute is attached to, as one of the routing rules to route the requests served by the load balancer.\nEach target proxy reference should match the pattern: projects/*/locations/global/targetTcpProxies/<target_tcp_proxy_name>"]
    pub fn set_target_proxies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_proxies = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<NetworkServicesTlsRouteRulesEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<NetworkServicesTlsRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TlsRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/*/gateways/<gateway_name>"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the TLS Route."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/*/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the TlsRoute resource."]
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
    #[doc = "Get a reference to the value of field `target_proxies` after provisioning.\nTargetProxies defines a list of target proxies this TlsRoute is attached to, as one of the routing rules to route the requests served by the load balancer.\nEach target proxy reference should match the pattern: projects/*/locations/global/targetTcpProxies/<target_tcp_proxy_name>"]
    pub fn target_proxies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TlsRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesTlsRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesTlsRouteTimeoutsElRef {
        NetworkServicesTlsRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesTlsRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesTlsRoute {}
impl ToListMappable for NetworkServicesTlsRoute {
    type O = ListRef<NetworkServicesTlsRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesTlsRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_tls_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesTlsRoute {
    pub tf_id: String,
    #[doc = "Name of the TlsRoute resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesTlsRoute {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesTlsRoute {
        let out = NetworkServicesTlsRoute(Rc::new(NetworkServicesTlsRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesTlsRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                gateways: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                meshes: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target_proxies: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesTlsRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesTlsRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the TlsRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/*/gateways/<gateway_name>"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the TLS Route."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/*/meshes/<mesh_name>\nThe attached Mesh should be of a type SIDECAR"]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the TlsRoute resource."]
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
    #[doc = "Get a reference to the value of field `target_proxies` after provisioning.\nTargetProxies defines a list of target proxies this TlsRoute is attached to, as one of the routing rules to route the requests served by the load balancer.\nEach target proxy reference should match the pattern: projects/*/locations/global/targetTcpProxies/<target_tcp_proxy_name>"]
    pub fn target_proxies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the TlsRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesTlsRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesTlsRouteTimeoutsElRef {
        NetworkServicesTlsRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTlsRouteRulesElActionElDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl NetworkServicesTlsRouteRulesElActionElDestinationsEl {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesTlsRouteRulesElActionElDestinationsEl {
    type O = BlockAssignable<NetworkServicesTlsRouteRulesElActionElDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTlsRouteRulesElActionElDestinationsEl {}
impl BuildNetworkServicesTlsRouteRulesElActionElDestinationsEl {
    pub fn build(self) -> NetworkServicesTlsRouteRulesElActionElDestinationsEl {
        NetworkServicesTlsRouteRulesElActionElDestinationsEl {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesTlsRouteRulesElActionElDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteRulesElActionElDestinationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesTlsRouteRulesElActionElDestinationsElRef {
        NetworkServicesTlsRouteRulesElActionElDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTlsRouteRulesElActionElDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesTlsRouteRulesElActionElDynamic {
    destinations: Option<DynamicBlock<NetworkServicesTlsRouteRulesElActionElDestinationsEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesTlsRouteRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<NetworkServicesTlsRouteRulesElActionElDestinationsEl>>,
    dynamic: NetworkServicesTlsRouteRulesElActionElDynamic,
}
impl NetworkServicesTlsRouteRulesElActionEl {
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesTlsRouteRulesElActionElDestinationsEl>>,
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
impl ToListMappable for NetworkServicesTlsRouteRulesElActionEl {
    type O = BlockAssignable<NetworkServicesTlsRouteRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTlsRouteRulesElActionEl {}
impl BuildNetworkServicesTlsRouteRulesElActionEl {
    pub fn build(self) -> NetworkServicesTlsRouteRulesElActionEl {
        NetworkServicesTlsRouteRulesElActionEl {
            destinations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesTlsRouteRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTlsRouteRulesElActionElRef {
        NetworkServicesTlsRouteRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTlsRouteRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<NetworkServicesTlsRouteRulesElActionElDestinationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTlsRouteRulesElMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alpn: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni_host: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesTlsRouteRulesElMatchesEl {
    #[doc = "Set the field `alpn`.\nALPN (Application-Layer Protocol Negotiation) to match against. Examples: \"http/1.1\", \"h2\". At least one of sniHost and alpn is required. Up to 5 alpns across all matches can be set."]
    pub fn set_alpn(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.alpn = Some(v.into());
        self
    }
    #[doc = "Set the field `sni_host`.\nSNI (server name indicator) to match against. SNI will be matched against all wildcard domains, i.e. www.example.com will be first matched against www.example.com, then *.example.com, then *.com.\nPartial wildcards are not supported, and values like *w.example.com are invalid. At least one of sniHost and alpn is required. Up to 5 sni hosts across all matches can be set."]
    pub fn set_sni_host(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.sni_host = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesTlsRouteRulesElMatchesEl {
    type O = BlockAssignable<NetworkServicesTlsRouteRulesElMatchesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTlsRouteRulesElMatchesEl {}
impl BuildNetworkServicesTlsRouteRulesElMatchesEl {
    pub fn build(self) -> NetworkServicesTlsRouteRulesElMatchesEl {
        NetworkServicesTlsRouteRulesElMatchesEl {
            alpn: core::default::Default::default(),
            sni_host: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesTlsRouteRulesElMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteRulesElMatchesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTlsRouteRulesElMatchesElRef {
        NetworkServicesTlsRouteRulesElMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTlsRouteRulesElMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alpn` after provisioning.\nALPN (Application-Layer Protocol Negotiation) to match against. Examples: \"http/1.1\", \"h2\". At least one of sniHost and alpn is required. Up to 5 alpns across all matches can be set."]
    pub fn alpn(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alpn", self.base))
    }
    #[doc = "Get a reference to the value of field `sni_host` after provisioning.\nSNI (server name indicator) to match against. SNI will be matched against all wildcard domains, i.e. www.example.com will be first matched against www.example.com, then *.example.com, then *.com.\nPartial wildcards are not supported, and values like *w.example.com are invalid. At least one of sniHost and alpn is required. Up to 5 sni hosts across all matches can be set."]
    pub fn sni_host(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.sni_host", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesTlsRouteRulesElDynamic {
    action: Option<DynamicBlock<NetworkServicesTlsRouteRulesElActionEl>>,
    matches: Option<DynamicBlock<NetworkServicesTlsRouteRulesElMatchesEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesTlsRouteRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<NetworkServicesTlsRouteRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<NetworkServicesTlsRouteRulesElMatchesEl>>,
    dynamic: NetworkServicesTlsRouteRulesElDynamic,
}
impl NetworkServicesTlsRouteRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesTlsRouteRulesElActionEl>>,
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
        v: impl Into<BlockAssignable<NetworkServicesTlsRouteRulesElMatchesEl>>,
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
impl ToListMappable for NetworkServicesTlsRouteRulesEl {
    type O = BlockAssignable<NetworkServicesTlsRouteRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTlsRouteRulesEl {}
impl BuildNetworkServicesTlsRouteRulesEl {
    pub fn build(self) -> NetworkServicesTlsRouteRulesEl {
        NetworkServicesTlsRouteRulesEl {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesTlsRouteRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTlsRouteRulesElRef {
        NetworkServicesTlsRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTlsRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<NetworkServicesTlsRouteRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> ListRef<NetworkServicesTlsRouteRulesElMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesTlsRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesTlsRouteTimeoutsEl {
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
impl ToListMappable for NetworkServicesTlsRouteTimeoutsEl {
    type O = BlockAssignable<NetworkServicesTlsRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesTlsRouteTimeoutsEl {}
impl BuildNetworkServicesTlsRouteTimeoutsEl {
    pub fn build(self) -> NetworkServicesTlsRouteTimeoutsEl {
        NetworkServicesTlsRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesTlsRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesTlsRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesTlsRouteTimeoutsElRef {
        NetworkServicesTlsRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesTlsRouteTimeoutsElRef {
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
struct NetworkServicesTlsRouteDynamic {
    rules: Option<DynamicBlock<NetworkServicesTlsRouteRulesEl>>,
}
