use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_ports: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_swg_autogen_router_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    envoy_headers: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesGatewayTimeoutsEl>,
}
struct NetworkServicesGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesGatewayData>,
}
#[derive(Clone)]
pub struct NetworkServicesGateway(Rc<NetworkServicesGateway_>);
impl NetworkServicesGateway {
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
    #[doc = "Set the field `addresses`.\nZero or one IPv4 or IPv6 address on which the Gateway will receive the traffic.\nWhen no address is provided, an IP from the subnetwork is allocated.\n\nThis field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6."]
    pub fn set_addresses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `all_ports`.\nConfigures this gateway to \u{200b}listen on all ports.\nBy enabling the wildcard ports feature on\u{200b} \u{200b}your Secure Web Proxy Gateway,\nit will accept traffic destined for any port (1-65535) on its\u{200b} assigned IP address.\u{200b}\nThis field is configurable only for gateways of type SECURE_WEB_GATEWAY."]
    pub fn set_all_ports(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().all_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_urls`.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_certificate_urls(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().certificate_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_swg_autogen_router_on_destroy`.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn set_delete_swg_autogen_router_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .delete_swg_autogen_router_on_destroy = Some(v.into());
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
    #[doc = "Set the field `envoy_headers`.\nDetermines if envoy will insert internal debug headers into upstream requests.\nOther Envoy headers may still be injected.\nBy default, envoy will not insert any debug headers. Possible values: [\"NONE\", \"DEBUG_HEADERS\"]"]
    pub fn set_envoy_headers(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().envoy_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `gateway_security_policy`.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_gateway_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_version`.\nThe IP Version that will be used by this gateway. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn set_ip_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_version = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the Gateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\nOne or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports.\n Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports."]
    pub fn set_ports(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().ports = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_mode`.\nThe routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. Possible values: [\"NEXT_HOP_ROUTING_MODE\", \"EXPLICIT_ROUTING_MODE\"]"]
    pub fn set_routing_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().routing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as a single coniguration to the proxy/load balancer.\n\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }
    #[doc = "Set the field `server_tls_policy`.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled."]
    pub fn set_server_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: projects/*/regions/us-central1/subnetworks/network-1.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nZero or one IPv4 or IPv6 address on which the Gateway will receive the traffic.\nWhen no address is provided, an IP from the subnetwork is allocated.\n\nThis field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_ports` after provisioning.\nConfigures this gateway to \u{200b}listen on all ports.\nBy enabling the wildcard ports feature on\u{200b} \u{200b}your Secure Web Proxy Gateway,\nit will accept traffic destined for any port (1-65535) on its\u{200b} assigned IP address.\u{200b}\nThis field is configurable only for gateways of type SECURE_WEB_GATEWAY."]
    pub fn all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_urls` after provisioning.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn certificate_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_swg_autogen_router_on_destroy` after provisioning.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn delete_swg_autogen_router_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.delete_swg_autogen_router_on_destroy",
                self.extract_ref()
            ),
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
    #[doc = "Get a reference to the value of field `envoy_headers` after provisioning.\nDetermines if envoy will insert internal debug headers into upstream requests.\nOther Envoy headers may still be injected.\nBy default, envoy will not insert any debug headers. Possible values: [\"NONE\", \"DEBUG_HEADERS\"]"]
    pub fn envoy_headers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.envoy_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_security_policy` after provisioning.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn gateway_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP Version that will be used by this gateway. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Gateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Gateway resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nOne or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports.\n Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports."]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_mode` after provisioning.\nThe routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. Possible values: [\"NEXT_HOP_ROUTING_MODE\", \"EXPLICIT_ROUTING_MODE\"]"]
    pub fn routing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as a single coniguration to the proxy/load balancer.\n\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: projects/*/regions/us-central1/subnetworks/network-1.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nImmutable. The type of the customer managed gateway. Possible values: [\"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesGateway {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesGateway {}
impl ToListMappable for NetworkServicesGateway {
    type O = ListRef<NetworkServicesGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesGateway_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesGateway {
    pub tf_id: String,
    #[doc = "Name of the Gateway resource."]
    pub name: PrimField<String>,
    #[doc = "Immutable. The type of the customer managed gateway. Possible values: [\"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub type_: PrimField<String>,
}
impl BuildNetworkServicesGateway {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesGateway {
        let out = NetworkServicesGateway(Rc::new(NetworkServicesGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                addresses: core::default::Default::default(),
                all_ports: core::default::Default::default(),
                certificate_urls: core::default::Default::default(),
                delete_swg_autogen_router_on_destroy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                envoy_headers: core::default::Default::default(),
                gateway_security_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_version: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                ports: core::default::Default::default(),
                project: core::default::Default::default(),
                routing_mode: core::default::Default::default(),
                scope: core::default::Default::default(),
                server_tls_policy: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                type_: self.type_,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nZero or one IPv4 or IPv6 address on which the Gateway will receive the traffic.\nWhen no address is provided, an IP from the subnetwork is allocated.\n\nThis field only applies to gateways of type 'SECURE_WEB_GATEWAY'.\nGateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_ports` after provisioning.\nConfigures this gateway to \u{200b}listen on all ports.\nBy enabling the wildcard ports feature on\u{200b} \u{200b}your Secure Web Proxy Gateway,\nit will accept traffic destined for any port (1-65535) on its\u{200b} assigned IP address.\u{200b}\nThis field is configurable only for gateways of type SECURE_WEB_GATEWAY."]
    pub fn all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_urls` after provisioning.\nA fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.\nThis feature only applies to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn certificate_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_swg_autogen_router_on_destroy` after provisioning.\nWhen deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.\nIf there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted."]
    pub fn delete_swg_autogen_router_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.delete_swg_autogen_router_on_destroy",
                self.extract_ref()
            ),
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
    #[doc = "Get a reference to the value of field `envoy_headers` after provisioning.\nDetermines if envoy will insert internal debug headers into upstream requests.\nOther Envoy headers may still be injected.\nBy default, envoy will not insert any debug headers. Possible values: [\"NONE\", \"DEBUG_HEADERS\"]"]
    pub fn envoy_headers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.envoy_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_security_policy` after provisioning.\nA fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.\nFor example: 'projects/*/locations/*/gatewaySecurityPolicies/swg-policy'.\nThis policy is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn gateway_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP Version that will be used by this gateway. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Gateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the gateway.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Gateway resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name identifying the VPC network that is using this configuration.\nFor example: 'projects/*/global/networks/network-1'.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nOne or more port numbers (1-65535), on which the Gateway will receive traffic.\nThe proxy binds to the specified ports.\n Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports."]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_mode` after provisioning.\nThe routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. Possible values: [\"NEXT_HOP_ROUTING_MODE\", \"EXPLICIT_ROUTING_MODE\"]"]
    pub fn routing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nImmutable. Scope determines how configuration across multiple Gateway instances are merged.\nThe configuration for multiple Gateway instances with the same scope will be merged as presented as a single coniguration to the proxy/load balancer.\n\nMax length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe relative resource name identifying the subnetwork in which this SWG is allocated.\nFor example: projects/*/regions/us-central1/subnetworks/network-1.\n\nCurrently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nImmutable. The type of the customer managed gateway. Possible values: [\"OPEN_MESH\", \"SECURE_WEB_GATEWAY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesGatewayTimeoutsEl {
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
impl ToListMappable for NetworkServicesGatewayTimeoutsEl {
    type O = BlockAssignable<NetworkServicesGatewayTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGatewayTimeoutsEl {}
impl BuildNetworkServicesGatewayTimeoutsEl {
    pub fn build(self) -> NetworkServicesGatewayTimeoutsEl {
        NetworkServicesGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGatewayTimeoutsElRef {
        NetworkServicesGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGatewayTimeoutsElRef {
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
