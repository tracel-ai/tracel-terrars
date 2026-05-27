use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionBackendServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affinity_cookie_ttl_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_draining_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cdn: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_checks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_selection_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality_lb_policy: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<Vec<ComputeRegionBackendServiceBackendEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdn_policy: Option<Vec<ComputeRegionBackendServiceCdnPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_breakers: Option<Vec<ComputeRegionBackendServiceCircuitBreakersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_tracking_policy: Option<Vec<ComputeRegionBackendServiceConnectionTrackingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consistent_hash: Option<Vec<ComputeRegionBackendServiceConsistentHashEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<Vec<ComputeRegionBackendServiceCustomMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_policy: Option<Vec<ComputeRegionBackendServiceFailoverPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ha_policy: Option<Vec<ComputeRegionBackendServiceHaPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iap: Option<Vec<ComputeRegionBackendServiceIapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeRegionBackendServiceLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_pass_through_lb_traffic_policy:
        Option<Vec<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outlier_detection: Option<Vec<ComputeRegionBackendServiceOutlierDetectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeRegionBackendServiceParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strong_session_affinity_cookie:
        Option<Vec<ComputeRegionBackendServiceStrongSessionAffinityCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionBackendServiceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<Vec<ComputeRegionBackendServiceTlsSettingsEl>>,
    dynamic: ComputeRegionBackendServiceDynamic,
}
struct ComputeRegionBackendService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionBackendServiceData>,
}
#[derive(Clone)]
pub struct ComputeRegionBackendService(Rc<ComputeRegionBackendService_>);
impl ComputeRegionBackendService {
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
    #[doc = "Set the field `affinity_cookie_ttl_sec`.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn set_affinity_cookie_ttl_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().affinity_cookie_ttl_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_draining_timeout_sec`.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn set_connection_draining_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connection_draining_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_cdn`.\nIf true, enable Cloud CDN for this RegionBackendService."]
    pub fn set_enable_cdn(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cdn = Some(v.into());
        self
    }
    #[doc = "Set the field `health_checks`.\nThe set of URLs to HealthCheck resources for health checking\nthis RegionBackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend."]
    pub fn set_health_checks(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().health_checks = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_selection_policy`.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn set_ip_address_selection_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_selection_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancing_scheme`.\nIndicates what kind of load balancing this regional backend service\nwill be used for. A backend service created for one type of load\nbalancing cannot be used with the other(s). For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"INTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `locality_lb_policy`.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn set_locality_lb_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().locality_lb_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe URL of the network to which this backend service belongs.\nThis field must be set for Internal Passthrough Network Load Balancers when the haPolicy is enabled, and for External Passthrough Network Load Balancers when the haPolicy fastIpMove is enabled.\nThis field can only be specified when the load balancing scheme is set to INTERNAL, or when the load balancing scheme is set to EXTERNAL and haPolicy fastIpMove is enabled.\nChanges to this field force recreation of the resource."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nA named port on a backend instance group representing the port for\ncommunication to the backend VMs in that group. Required when the\nloadBalancingScheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED, or INTERNAL_SELF_MANAGED\nand the backends are instance groups. The named port must be defined on each\nbackend instance group. This parameter has no meaning if the backends are NEGs. API sets a\ndefault of \"http\" if not given.\nMust be omitted when the loadBalancingScheme is INTERNAL (Internal TCP/UDP Load Balancing)."]
    pub fn set_port_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe Region in which the created backend service should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_policy`.\nThe security policy associated with this backend service."]
    pub fn set_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `session_affinity`.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"CLIENT_IP_NO_DESTINATION\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn set_session_affinity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().session_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_sec`.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn set_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceBackendEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backend = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backend = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceCdnPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cdn_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cdn_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `circuit_breakers`.\n"]
    pub fn set_circuit_breakers(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceCircuitBreakersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().circuit_breakers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.circuit_breakers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connection_tracking_policy`.\n"]
    pub fn set_connection_tracking_policy(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceConnectionTrackingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_tracking_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_tracking_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `consistent_hash`.\n"]
    pub fn set_consistent_hash(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceConsistentHashEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().consistent_hash = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.consistent_hash = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceCustomMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_metrics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `failover_policy`.\n"]
    pub fn set_failover_policy(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceFailoverPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().failover_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.failover_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ha_policy`.\n"]
    pub fn set_ha_policy(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceHaPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ha_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ha_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iap`.\n"]
    pub fn set_iap(self, v: impl Into<BlockAssignable<ComputeRegionBackendServiceIapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iap = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iap = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_pass_through_lb_traffic_policy`.\n"]
    pub fn set_network_pass_through_lb_traffic_policy(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .network_pass_through_lb_traffic_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .network_pass_through_lb_traffic_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceOutlierDetectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outlier_detection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outlier_detection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceParamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `strong_session_affinity_cookie`.\n"]
    pub fn set_strong_session_affinity_cookie(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceStrongSessionAffinityCookieEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().strong_session_affinity_cookie = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .strong_session_affinity_cookie = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionBackendServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_settings`.\n"]
    pub fn set_tls_settings(
        self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceTlsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.affinity_cookie_ttl_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this RegionBackendService."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this\nobject. This field is used in optimistic locking."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn generated_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to HealthCheck resources for health checking\nthis RegionBackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend."]
    pub fn health_checks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.health_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address_selection_policy` after provisioning.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn ip_address_selection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_selection_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates what kind of load balancing this regional backend service\nwill be used for. A backend service created for one type of load\nbalancing cannot be used with the other(s). For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"INTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locality_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe URL of the network to which this backend service belongs.\nThis field must be set for Internal Passthrough Network Load Balancers when the haPolicy is enabled, and for External Passthrough Network Load Balancers when the haPolicy fastIpMove is enabled.\nThis field can only be specified when the load balancing scheme is set to INTERNAL, or when the load balancing scheme is set to EXTERNAL and haPolicy fastIpMove is enabled.\nChanges to this field force recreation of the resource."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nA named port on a backend instance group representing the port for\ncommunication to the backend VMs in that group. Required when the\nloadBalancingScheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED, or INTERNAL_SELF_MANAGED\nand the backends are instance groups. The named port must be defined on each\nbackend instance group. This parameter has no meaning if the backends are NEGs. API sets a\ndefault of \"http\" if not given.\nMust be omitted when the loadBalancingScheme is INTERNAL (Internal TCP/UDP Load Balancing)."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created backend service should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe security policy associated with this backend service."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"CLIENT_IP_NO_DESTINATION\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn session_affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> ListRef<ComputeRegionBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> ListRef<ComputeRegionBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_tracking_policy` after provisioning.\n"]
    pub fn connection_tracking_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceConnectionTrackingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> ListRef<ComputeRegionBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<ComputeRegionBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `failover_policy` after provisioning.\n"]
    pub fn failover_policy(&self) -> ListRef<ComputeRegionBackendServiceFailoverPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ha_policy` after provisioning.\n"]
    pub fn ha_policy(&self) -> ListRef<ComputeRegionBackendServiceHaPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ha_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<ComputeRegionBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeRegionBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_pass_through_lb_traffic_policy` after provisioning.\n"]
    pub fn network_pass_through_lb_traffic_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.network_pass_through_lb_traffic_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<ComputeRegionBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRegionBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionBackendServiceTimeoutsElRef {
        ComputeRegionBackendServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> ListRef<ComputeRegionBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionBackendService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionBackendService {}
impl ToListMappable for ComputeRegionBackendService {
    type O = ListRef<ComputeRegionBackendServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionBackendService_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_backend_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionBackendService {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionBackendService {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionBackendService {
        let out = ComputeRegionBackendService(Rc::new(ComputeRegionBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionBackendServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                affinity_cookie_ttl_sec: core::default::Default::default(),
                connection_draining_timeout_sec: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_cdn: core::default::Default::default(),
                health_checks: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_selection_policy: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                locality_lb_policy: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                port_name: core::default::Default::default(),
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                security_policy: core::default::Default::default(),
                session_affinity: core::default::Default::default(),
                timeout_sec: core::default::Default::default(),
                backend: core::default::Default::default(),
                cdn_policy: core::default::Default::default(),
                circuit_breakers: core::default::Default::default(),
                connection_tracking_policy: core::default::Default::default(),
                consistent_hash: core::default::Default::default(),
                custom_metrics: core::default::Default::default(),
                failover_policy: core::default::Default::default(),
                ha_policy: core::default::Default::default(),
                iap: core::default::Default::default(),
                log_config: core::default::Default::default(),
                network_pass_through_lb_traffic_policy: core::default::Default::default(),
                outlier_detection: core::default::Default::default(),
                params: core::default::Default::default(),
                strong_session_affinity_cookie: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tls_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionBackendServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionBackendServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.affinity_cookie_ttl_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this RegionBackendService."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this\nobject. This field is used in optimistic locking."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn generated_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to HealthCheck resources for health checking\nthis RegionBackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend."]
    pub fn health_checks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.health_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address_selection_policy` after provisioning.\nSpecifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC). Possible values: [\"IPV4_ONLY\", \"PREFER_IPV6\", \"IPV6_ONLY\"]"]
    pub fn ip_address_selection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_selection_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates what kind of load balancing this regional backend service\nwill be used for. A backend service created for one type of load\nbalancing cannot be used with the other(s). For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"INTERNAL\" Possible values: [\"EXTERNAL\", \"EXTERNAL_MANAGED\", \"INTERNAL\", \"INTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locality_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe URL of the network to which this backend service belongs.\nThis field must be set for Internal Passthrough Network Load Balancers when the haPolicy is enabled, and for External Passthrough Network Load Balancers when the haPolicy fastIpMove is enabled.\nThis field can only be specified when the load balancing scheme is set to INTERNAL, or when the load balancing scheme is set to EXTERNAL and haPolicy fastIpMove is enabled.\nChanges to this field force recreation of the resource."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nA named port on a backend instance group representing the port for\ncommunication to the backend VMs in that group. Required when the\nloadBalancingScheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED, or INTERNAL_SELF_MANAGED\nand the backends are instance groups. The named port must be defined on each\nbackend instance group. This parameter has no meaning if the backends are NEGs. API sets a\ndefault of \"http\" if not given.\nMust be omitted when the loadBalancingScheme is INTERNAL (Internal TCP/UDP Load Balancing)."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created backend service should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe security policy associated with this backend service."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"CLIENT_IP_NO_DESTINATION\", \"STRONG_COOKIE_AFFINITY\"]"]
    pub fn session_affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> ListRef<ComputeRegionBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> ListRef<ComputeRegionBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_tracking_policy` after provisioning.\n"]
    pub fn connection_tracking_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceConnectionTrackingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> ListRef<ComputeRegionBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<ComputeRegionBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `failover_policy` after provisioning.\n"]
    pub fn failover_policy(&self) -> ListRef<ComputeRegionBackendServiceFailoverPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ha_policy` after provisioning.\n"]
    pub fn ha_policy(&self) -> ListRef<ComputeRegionBackendServiceHaPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ha_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<ComputeRegionBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeRegionBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_pass_through_lb_traffic_policy` after provisioning.\n"]
    pub fn network_pass_through_lb_traffic_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.network_pass_through_lb_traffic_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<ComputeRegionBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRegionBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionBackendServiceTimeoutsElRef {
        ComputeRegionBackendServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> ListRef<ComputeRegionBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceBackendElCustomMetricsEl {
    dry_run: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_utilization: Option<PrimField<f64>>,
    name: PrimField<String>,
}
impl ComputeRegionBackendServiceBackendElCustomMetricsEl {
    #[doc = "Set the field `max_utilization`.\nOptional parameter to define a target utilization for the Custom Metrics\nbalancing mode. The valid range is <code>[0.0, 1.0]</code>."]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceBackendElCustomMetricsEl {
    type O = BlockAssignable<ComputeRegionBackendServiceBackendElCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceBackendElCustomMetricsEl {
    #[doc = "If true, the metric data is collected and reported to Cloud\nMonitoring, but is not used for load balancing."]
    pub dry_run: PrimField<bool>,
    #[doc = "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionBackendServiceBackendElCustomMetricsEl {
    pub fn build(self) -> ComputeRegionBackendServiceBackendElCustomMetricsEl {
        ComputeRegionBackendServiceBackendElCustomMetricsEl {
            dry_run: self.dry_run,
            max_utilization: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct ComputeRegionBackendServiceBackendElCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceBackendElCustomMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceBackendElCustomMetricsElRef {
        ComputeRegionBackendServiceBackendElCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceBackendElCustomMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\nIf true, the metric data is collected and reported to Cloud\nMonitoring, but is not used for load balancing."]
    pub fn dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }
    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\nOptional parameter to define a target utilization for the Custom Metrics\nbalancing mode. The valid range is <code>[0.0, 1.0]</code>."]
    pub fn max_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceBackendElDynamic {
    custom_metrics: Option<DynamicBlock<ComputeRegionBackendServiceBackendElCustomMetricsEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    balancing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_scaler: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover: Option<PrimField<bool>>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections_per_endpoint: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections_per_instance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rate_per_endpoint: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rate_per_instance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_utilization: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<Vec<ComputeRegionBackendServiceBackendElCustomMetricsEl>>,
    dynamic: ComputeRegionBackendServiceBackendElDynamic,
}
impl ComputeRegionBackendServiceBackendEl {
    #[doc = "Set the field `balancing_mode`.\nSpecifies the balancing mode for this backend.\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\"]"]
    pub fn set_balancing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.balancing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_scaler`.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\n~>**NOTE**: This field cannot be set for\nINTERNAL region backend services (default loadBalancingScheme),\nbut is required for non-INTERNAL backend service. The total\ncapacity_scaler for all backends must be non-zero.\n\nA setting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
    pub fn set_capacity_scaler(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capacity_scaler = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource.\nProvide this property when you create the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `failover`.\nThis field designates whether this is a failover backend. More\nthan one failover backend can be configured for a given RegionBackendService."]
    pub fn set_failover(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections`.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\nCannot be set for INTERNAL backend services.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_endpoint`.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. Cannot be set\nfor INTERNAL backend services.\n\nThis is used to calculate the capacity of the group. Can be\nused in either CONNECTION or UTILIZATION balancing modes. For\nCONNECTION mode, either maxConnections or\nmaxConnectionsPerEndpoint must be set."]
    pub fn set_max_connections_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_instance`.\nThe max number of simultaneous connections that a single\nbackend instance can handle. Cannot be set for INTERNAL backend\nservices.\n\nThis is used to calculate the capacity of the group.\nCan be used in either CONNECTION or UTILIZATION balancing modes.\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn set_max_connections_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate`.\nThe max requests per second (RPS) of the group. Cannot be set\nfor INTERNAL backend services.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. Either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn set_max_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_endpoint`.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set. Cannot be set\nfor INTERNAL backend services."]
    pub fn set_max_rate_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_instance`.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set. Cannot be set\nfor INTERNAL backend services."]
    pub fn set_max_rate_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_utilization`.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0].\nCannot be set for INTERNAL backend services."]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceBackendElCustomMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metrics = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceBackendEl {
    type O = BlockAssignable<ComputeRegionBackendServiceBackendEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceBackendEl {
    #[doc = "The fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nWhen the 'load_balancing_scheme' is INTERNAL, only instance groups\nare supported.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub group: PrimField<String>,
}
impl BuildComputeRegionBackendServiceBackendEl {
    pub fn build(self) -> ComputeRegionBackendServiceBackendEl {
        ComputeRegionBackendServiceBackendEl {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            description: core::default::Default::default(),
            failover: core::default::Default::default(),
            group: self.group,
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceBackendElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceBackendElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceBackendElRef {
        ComputeRegionBackendServiceBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceBackendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `balancing_mode` after provisioning.\nSpecifies the balancing mode for this backend.\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\"]"]
    pub fn balancing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.balancing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_scaler` after provisioning.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\n~>**NOTE**: This field cannot be set for\nINTERNAL region backend services (default loadBalancingScheme),\nbut is required for non-INTERNAL backend service. The total\ncapacity_scaler for all backends must be non-zero.\n\nA setting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
    pub fn capacity_scaler(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_scaler", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource.\nProvide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `failover` after provisioning.\nThis field designates whether this is a failover backend. More\nthan one failover backend can be configured for a given RegionBackendService."]
    pub fn failover(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nWhen the 'load_balancing_scheme' is INTERNAL, only instance groups\nare supported.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\nCannot be set for INTERNAL backend services.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_endpoint` after provisioning.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. Cannot be set\nfor INTERNAL backend services.\n\nThis is used to calculate the capacity of the group. Can be\nused in either CONNECTION or UTILIZATION balancing modes. For\nCONNECTION mode, either maxConnections or\nmaxConnectionsPerEndpoint must be set."]
    pub fn max_connections_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_instance` after provisioning.\nThe max number of simultaneous connections that a single\nbackend instance can handle. Cannot be set for INTERNAL backend\nservices.\n\nThis is used to calculate the capacity of the group.\nCan be used in either CONNECTION or UTILIZATION balancing modes.\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn max_connections_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate` after provisioning.\nThe max requests per second (RPS) of the group. Cannot be set\nfor INTERNAL backend services.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. Either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn max_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `max_rate_per_endpoint` after provisioning.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set. Cannot be set\nfor INTERNAL backend services."]
    pub fn max_rate_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate_per_instance` after provisioning.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set. Cannot be set\nfor INTERNAL backend services."]
    pub fn max_rate_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0].\nCannot be set for INTERNAL backend services."]
    pub fn max_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceBackendElCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_host: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_named_cookies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_protocol: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_query_string: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_blacklist: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_whitelist: Option<SetField<PrimField<String>>>,
}
impl ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_host`.\nIf true requests to different hosts will be cached separately."]
    pub fn set_include_host(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_host = Some(v.into());
        self
    }
    #[doc = "Set the field `include_named_cookies`.\nNames of cookies to include in cache keys."]
    pub fn set_include_named_cookies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_named_cookies = Some(v.into());
        self
    }
    #[doc = "Set the field `include_protocol`.\nIf true, http and https requests will be cached separately."]
    pub fn set_include_protocol(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `include_query_string`.\nIf true, include query string parameters in the cache key\naccording to query_string_whitelist and\nquery_string_blacklist. If neither is set, the entire query\nstring will be included.\n\nIf false, the query string will be excluded from the cache\nkey entirely."]
    pub fn set_include_query_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_query_string = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_blacklist`.\nNames of query string parameters to exclude in cache keys.\n\nAll other parameters will be included. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn set_query_string_blacklist(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.query_string_blacklist = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_whitelist`.\nNames of query string parameters to include in cache keys.\n\nAll other parameters will be excluded. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn set_query_string_whitelist(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.query_string_whitelist = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {}
impl BuildComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
        ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
            include_host: core::default::Default::default(),
            include_named_cookies: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            include_query_string: core::default::Default::default(),
            query_string_blacklist: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
        ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_host` after provisioning.\nIf true requests to different hosts will be cached separately."]
    pub fn include_host(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }
    #[doc = "Get a reference to the value of field `include_named_cookies` after provisioning.\nNames of cookies to include in cache keys."]
    pub fn include_named_cookies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_named_cookies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_protocol` after provisioning.\nIf true, http and https requests will be cached separately."]
    pub fn include_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_protocol", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_query_string` after provisioning.\nIf true, include query string parameters in the cache key\naccording to query_string_whitelist and\nquery_string_blacklist. If neither is set, the entire query\nstring will be included.\n\nIf false, the query string will be excluded from the cache\nkey entirely."]
    pub fn include_query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_query_string", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_blacklist` after provisioning.\nNames of query string parameters to exclude in cache keys.\n\nAll other parameters will be included. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn query_string_blacklist(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_string_blacklist", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\nNames of query string parameters to include in cache keys.\n\nAll other parameters will be excluded. Either specify\nquery_string_whitelist or query_string_blacklist, not both.\n'&' and '=' will be percent encoded and not treated as\ndelimiters."]
    pub fn query_string_whitelist(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_string_whitelist", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
}
impl ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {}
impl BuildComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
        ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
        ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceCdnPolicyElDynamic {
    cache_key_policy: Option<DynamicBlock<ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    negative_caching_policy:
        Option<DynamicBlock<ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceCdnPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<Vec<ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching_policy:
        Option<Vec<ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    dynamic: ComputeRegionBackendServiceCdnPolicyElDynamic,
}
impl ComputeRegionBackendServiceCdnPolicyEl {
    #[doc = "Set the field `cache_mode`.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn set_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn set_client_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age)."]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn set_negative_caching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negative_caching = Some(v.into());
        self
    }
    #[doc = "Set the field `serve_while_stale`.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn set_serve_while_stale(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.serve_while_stale = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_url_cache_max_age_sec`.\nMaximum number of seconds the response to a signed URL request\nwill be considered fresh, defaults to 1hr (3600s). After this\ntime period, the response will be revalidated before\nbeing served.\n\nWhen serving responses to signed URL requests, Cloud CDN will\ninternally behave as though all responses from this backend had a\n\"Cache-Control: public, max-age=[TTL]\" header, regardless of any\nexisting Cache-Control header. The actual headers served in\nresponses will not be altered."]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_key_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.negative_caching_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.negative_caching_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceCdnPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceCdnPolicyEl {}
impl BuildComputeRegionBackendServiceCdnPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceCdnPolicyEl {
        ComputeRegionBackendServiceCdnPolicyEl {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceCdnPolicyElRef {
        ComputeRegionBackendServiceCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn client_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age)."]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin."]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn negative_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negative_caching", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn serve_while_stale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serve_while_stale", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\nMaximum number of seconds the response to a signed URL request\nwill be considered fresh, defaults to 1hr (3600s). After this\ntime period, the response will be revalidated before\nbeing served.\n\nWhen serving responses to signed URL requests, Cloud CDN will\ninternally behave as though all responses from this backend had a\n\"Cache-Control: public, max-age=[TTL]\" header, regardless of any\nexisting Cache-Control header. The actual headers served in\nresponses will not be altered."]
    pub fn signed_url_cache_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url_cache_max_age_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceCircuitBreakersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_requests: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests_per_connection: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
}
impl ComputeRegionBackendServiceCircuitBreakersEl {
    #[doc = "Set the field `max_connections`.\nThe maximum number of connections to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pending_requests`.\nThe maximum number of pending requests to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_pending_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pending_requests = Some(v.into());
        self
    }
    #[doc = "Set the field `max_requests`.\nThe maximum number of parallel requests to the backend cluster.\nDefaults to 1024."]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
    #[doc = "Set the field `max_requests_per_connection`.\nMaximum requests for a single backend connection. This parameter\nis respected by both the HTTP/1.1 and HTTP/2 implementations. If\nnot specified, there is no limit. Setting this parameter to 1\nwill effectively disable keep alive."]
    pub fn set_max_requests_per_connection(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests_per_connection = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retries`.\nThe maximum number of parallel retries to the backend cluster.\nDefaults to 3."]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceCircuitBreakersEl {
    type O = BlockAssignable<ComputeRegionBackendServiceCircuitBreakersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceCircuitBreakersEl {}
impl BuildComputeRegionBackendServiceCircuitBreakersEl {
    pub fn build(self) -> ComputeRegionBackendServiceCircuitBreakersEl {
        ComputeRegionBackendServiceCircuitBreakersEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceCircuitBreakersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceCircuitBreakersElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceCircuitBreakersElRef {
        ComputeRegionBackendServiceCircuitBreakersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceCircuitBreakersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\nThe maximum number of connections to the backend cluster.\nDefaults to 1024."]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\nThe maximum number of pending requests to the backend cluster.\nDefaults to 1024."]
    pub fn max_pending_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pending_requests", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\nThe maximum number of parallel requests to the backend cluster.\nDefaults to 1024."]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
    #[doc = "Get a reference to the value of field `max_requests_per_connection` after provisioning.\nMaximum requests for a single backend connection. This parameter\nis respected by both the HTTP/1.1 and HTTP/2 implementations. If\nnot specified, there is no limit. Setting this parameter to 1\nwill effectively disable keep alive."]
    pub fn max_requests_per_connection(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_requests_per_connection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\nThe maximum number of parallel retries to the backend cluster.\nDefaults to 3."]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceConnectionTrackingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_persistence_on_unhealthy_backends: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_strong_affinity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_mode: Option<PrimField<String>>,
}
impl ComputeRegionBackendServiceConnectionTrackingPolicyEl {
    #[doc = "Set the field `connection_persistence_on_unhealthy_backends`.\nSpecifies connection persistence when backends are unhealthy.\n\nIf set to 'DEFAULT_FOR_PROTOCOL', the existing connections persist on\nunhealthy backends only for connection-oriented protocols (TCP and SCTP)\nand only if the Tracking Mode is PER_CONNECTION (default tracking mode)\nor the Session Affinity is configured for 5-tuple. They do not persist\nfor UDP.\n\nIf set to 'NEVER_PERSIST', after a backend becomes unhealthy, the existing\nconnections on the unhealthy backend are never persisted on the unhealthy\nbackend. They are always diverted to newly selected healthy backends\n(unless all backends are unhealthy).\n\nIf set to 'ALWAYS_PERSIST', existing connections always persist on\nunhealthy backends regardless of protocol and session affinity. It is\ngenerally not recommended to use this mode overriding the default. Default value: \"DEFAULT_FOR_PROTOCOL\" Possible values: [\"DEFAULT_FOR_PROTOCOL\", \"NEVER_PERSIST\", \"ALWAYS_PERSIST\"]"]
    pub fn set_connection_persistence_on_unhealthy_backends(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.connection_persistence_on_unhealthy_backends = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_strong_affinity`.\nEnable Strong Session Affinity for Network Load Balancing. This option is not available publicly."]
    pub fn set_enable_strong_affinity(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_strong_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_timeout_sec`.\nSpecifies how long to keep a Connection Tracking entry while there is\nno matching traffic (in seconds).\n\nFor L4 ILB the minimum(default) is 10 minutes and maximum is 16 hours.\n\nFor NLB the minimum(default) is 60 seconds and the maximum is 16 hours."]
    pub fn set_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `tracking_mode`.\nSpecifies the key used for connection tracking. There are two options:\n'PER_CONNECTION': The Connection Tracking is performed as per the\nConnection Key (default Hash Method) for the specific protocol.\n\n'PER_SESSION': The Connection Tracking is performed as per the\nconfigured Session Affinity. It matches the configured Session Affinity. Default value: \"PER_CONNECTION\" Possible values: [\"PER_CONNECTION\", \"PER_SESSION\"]"]
    pub fn set_tracking_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tracking_mode = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceConnectionTrackingPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceConnectionTrackingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceConnectionTrackingPolicyEl {}
impl BuildComputeRegionBackendServiceConnectionTrackingPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceConnectionTrackingPolicyEl {
        ComputeRegionBackendServiceConnectionTrackingPolicyEl {
            connection_persistence_on_unhealthy_backends: core::default::Default::default(),
            enable_strong_affinity: core::default::Default::default(),
            idle_timeout_sec: core::default::Default::default(),
            tracking_mode: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceConnectionTrackingPolicyElRef {
        ComputeRegionBackendServiceConnectionTrackingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_persistence_on_unhealthy_backends` after provisioning.\nSpecifies connection persistence when backends are unhealthy.\n\nIf set to 'DEFAULT_FOR_PROTOCOL', the existing connections persist on\nunhealthy backends only for connection-oriented protocols (TCP and SCTP)\nand only if the Tracking Mode is PER_CONNECTION (default tracking mode)\nor the Session Affinity is configured for 5-tuple. They do not persist\nfor UDP.\n\nIf set to 'NEVER_PERSIST', after a backend becomes unhealthy, the existing\nconnections on the unhealthy backend are never persisted on the unhealthy\nbackend. They are always diverted to newly selected healthy backends\n(unless all backends are unhealthy).\n\nIf set to 'ALWAYS_PERSIST', existing connections always persist on\nunhealthy backends regardless of protocol and session affinity. It is\ngenerally not recommended to use this mode overriding the default. Default value: \"DEFAULT_FOR_PROTOCOL\" Possible values: [\"DEFAULT_FOR_PROTOCOL\", \"NEVER_PERSIST\", \"ALWAYS_PERSIST\"]"]
    pub fn connection_persistence_on_unhealthy_backends(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_persistence_on_unhealthy_backends", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_strong_affinity` after provisioning.\nEnable Strong Session Affinity for Network Load Balancing. This option is not available publicly."]
    pub fn enable_strong_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_strong_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_timeout_sec` after provisioning.\nSpecifies how long to keep a Connection Tracking entry while there is\nno matching traffic (in seconds).\n\nFor L4 ILB the minimum(default) is 10 minutes and maximum is 16 hours.\n\nFor NLB the minimum(default) is 60 seconds and the maximum is 16 hours."]
    pub fn idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_mode` after provisioning.\nSpecifies the key used for connection tracking. There are two options:\n'PER_CONNECTION': The Connection Tracking is performed as per the\nConnection Key (default Hash Method) for the specific protocol.\n\n'PER_SESSION': The Connection Tracking is performed as per the\nconfigured Session Affinity. It matches the configured Session Affinity. Default value: \"PER_CONNECTION\" Possible values: [\"PER_CONNECTION\", \"PER_SESSION\"]"]
    pub fn tracking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tracking_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    type O = BlockAssignable<ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    pub fn build(self) -> ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
        ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
        ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceConsistentHashElHttpCookieElDynamic {
    ttl: Option<DynamicBlock<ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>>,
    dynamic: ComputeRegionBackendServiceConsistentHashElHttpCookieElDynamic,
}
impl ComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    #[doc = "Set the field `name`.\nName of the cookie."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to set for the cookie."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ttl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    type O = BlockAssignable<ComputeRegionBackendServiceConsistentHashElHttpCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceConsistentHashElHttpCookieEl {}
impl BuildComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    pub fn build(self) -> ComputeRegionBackendServiceConsistentHashElHttpCookieEl {
        ComputeRegionBackendServiceConsistentHashElHttpCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
        ComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the cookie."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to set for the cookie."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<ComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceConsistentHashElDynamic {
    http_cookie: Option<DynamicBlock<ComputeRegionBackendServiceConsistentHashElHttpCookieEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceConsistentHashEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_ring_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_cookie: Option<Vec<ComputeRegionBackendServiceConsistentHashElHttpCookieEl>>,
    dynamic: ComputeRegionBackendServiceConsistentHashElDynamic,
}
impl ComputeRegionBackendServiceConsistentHashEl {
    #[doc = "Set the field `http_header_name`.\nThe hash based on the value of the specified header field.\nThis field is applicable if the sessionAffinity is set to HEADER_FIELD."]
    pub fn set_http_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_ring_size`.\nThe minimum number of virtual nodes to use for the hash ring.\nLarger ring sizes result in more granular load\ndistributions. If the number of hosts in the load balancing pool\nis larger than the ring size, each host will be assigned a single\nvirtual node.\nDefaults to 1024."]
    pub fn set_minimum_ring_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_ring_size = Some(v.into());
        self
    }
    #[doc = "Set the field `http_cookie`.\n"]
    pub fn set_http_cookie(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceConsistentHashElHttpCookieEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_cookie = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_cookie = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceConsistentHashEl {
    type O = BlockAssignable<ComputeRegionBackendServiceConsistentHashEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceConsistentHashEl {}
impl BuildComputeRegionBackendServiceConsistentHashEl {
    pub fn build(self) -> ComputeRegionBackendServiceConsistentHashEl {
        ComputeRegionBackendServiceConsistentHashEl {
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
            http_cookie: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceConsistentHashElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceConsistentHashElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceConsistentHashElRef {
        ComputeRegionBackendServiceConsistentHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceConsistentHashElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\nThe hash based on the value of the specified header field.\nThis field is applicable if the sessionAffinity is set to HEADER_FIELD."]
    pub fn http_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_header_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_ring_size` after provisioning.\nThe minimum number of virtual nodes to use for the hash ring.\nLarger ring sizes result in more granular load\ndistributions. If the number of hosts in the load balancing pool\nis larger than the ring size, each host will be assigned a single\nvirtual node.\nDefaults to 1024."]
    pub fn minimum_ring_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_ring_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_cookie` after provisioning.\n"]
    pub fn http_cookie(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceConsistentHashElHttpCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_cookie", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceCustomMetricsEl {
    dry_run: PrimField<bool>,
    name: PrimField<String>,
}
impl ComputeRegionBackendServiceCustomMetricsEl {}
impl ToListMappable for ComputeRegionBackendServiceCustomMetricsEl {
    type O = BlockAssignable<ComputeRegionBackendServiceCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceCustomMetricsEl {
    #[doc = "If true, the metric data is not used for load balancing."]
    pub dry_run: PrimField<bool>,
    #[doc = "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionBackendServiceCustomMetricsEl {
    pub fn build(self) -> ComputeRegionBackendServiceCustomMetricsEl {
        ComputeRegionBackendServiceCustomMetricsEl {
            dry_run: self.dry_run,
            name: self.name,
        }
    }
}
pub struct ComputeRegionBackendServiceCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceCustomMetricsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceCustomMetricsElRef {
        ComputeRegionBackendServiceCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceCustomMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\nIf true, the metric data is not used for load balancing."]
    pub fn dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceFailoverPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_connection_drain_on_failover: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_traffic_if_unhealthy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_ratio: Option<PrimField<f64>>,
}
impl ComputeRegionBackendServiceFailoverPolicyEl {
    #[doc = "Set the field `disable_connection_drain_on_failover`.\nOn failover or failback, this field indicates whether connection drain\nwill be honored. Setting this to true has the following effect: connections\nto the old active pool are not drained. Connections to the new active pool\nuse the timeout of 10 min (currently fixed). Setting to false has the\nfollowing effect: both old and new connections will have a drain timeout\nof 10 min.\nThis can be set to true only if the protocol is TCP.\nThe default is false."]
    pub fn set_disable_connection_drain_on_failover(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_connection_drain_on_failover = Some(v.into());
        self
    }
    #[doc = "Set the field `drop_traffic_if_unhealthy`.\nThis option is used only when no healthy VMs are detected in the primary\nand backup instance groups. When set to true, traffic is dropped. When\nset to false, new connections are sent across all VMs in the primary group.\nThe default is false."]
    pub fn set_drop_traffic_if_unhealthy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.drop_traffic_if_unhealthy = Some(v.into());
        self
    }
    #[doc = "Set the field `failover_ratio`.\nThe value of the field must be in [0, 1]. If the ratio of the healthy\nVMs in the primary backend is at or below this number, traffic arriving\nat the load-balanced IP will be directed to the failover backend.\nIn case where 'failoverRatio' is not set or all the VMs in the backup\nbackend are unhealthy, the traffic will be directed back to the primary\nbackend in the \"force\" mode, where traffic will be spread to the healthy\nVMs with the best effort, or to all VMs when no VM is healthy.\nThis field is only used with l4 load balancing."]
    pub fn set_failover_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failover_ratio = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceFailoverPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceFailoverPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceFailoverPolicyEl {}
impl BuildComputeRegionBackendServiceFailoverPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceFailoverPolicyEl {
        ComputeRegionBackendServiceFailoverPolicyEl {
            disable_connection_drain_on_failover: core::default::Default::default(),
            drop_traffic_if_unhealthy: core::default::Default::default(),
            failover_ratio: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceFailoverPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceFailoverPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceFailoverPolicyElRef {
        ComputeRegionBackendServiceFailoverPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceFailoverPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_connection_drain_on_failover` after provisioning.\nOn failover or failback, this field indicates whether connection drain\nwill be honored. Setting this to true has the following effect: connections\nto the old active pool are not drained. Connections to the new active pool\nuse the timeout of 10 min (currently fixed). Setting to false has the\nfollowing effect: both old and new connections will have a drain timeout\nof 10 min.\nThis can be set to true only if the protocol is TCP.\nThe default is false."]
    pub fn disable_connection_drain_on_failover(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_connection_drain_on_failover", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `drop_traffic_if_unhealthy` after provisioning.\nThis option is used only when no healthy VMs are detected in the primary\nand backup instance groups. When set to true, traffic is dropped. When\nset to false, new connections are sent across all VMs in the primary group.\nThe default is false."]
    pub fn drop_traffic_if_unhealthy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.drop_traffic_if_unhealthy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failover_ratio` after provisioning.\nThe value of the field must be in [0, 1]. If the ratio of the healthy\nVMs in the primary backend is at or below this number, traffic arriving\nat the load-balanced IP will be directed to the failover backend.\nIn case where 'failoverRatio' is not set or all the VMs in the backup\nbackend are unhealthy, the traffic will be directed back to the primary\nbackend in the \"force\" mode, where traffic will be spread to the healthy\nVMs with the best effort, or to all VMs when no VM is healthy.\nThis field is only used with l4 load balancing."]
    pub fn failover_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_ratio", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
}
impl ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    #[doc = "Set the field `instance`.\nThe name of the VM instance of the leader network endpoint. The instance must\nalready be attached to the NEG specified in the haPolicy.leader.backendGroup."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    type O = BlockAssignable<ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {}
impl BuildComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    pub fn build(self) -> ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
        ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
            instance: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
        ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the VM instance of the leader network endpoint. The instance must\nalready be attached to the NEG specified in the haPolicy.leader.backendGroup."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceHaPolicyElLeaderElDynamic {
    network_endpoint:
        Option<DynamicBlock<ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceHaPolicyElLeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_endpoint: Option<Vec<ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>>,
    dynamic: ComputeRegionBackendServiceHaPolicyElLeaderElDynamic,
}
impl ComputeRegionBackendServiceHaPolicyElLeaderEl {
    #[doc = "Set the field `backend_group`.\nA fully-qualified URL of the zonal Network Endpoint Group (NEG) that the leader is\nattached to."]
    pub fn set_backend_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_group = Some(v.into());
        self
    }
    #[doc = "Set the field `network_endpoint`.\n"]
    pub fn set_network_endpoint(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_endpoint = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceHaPolicyElLeaderEl {
    type O = BlockAssignable<ComputeRegionBackendServiceHaPolicyElLeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceHaPolicyElLeaderEl {}
impl BuildComputeRegionBackendServiceHaPolicyElLeaderEl {
    pub fn build(self) -> ComputeRegionBackendServiceHaPolicyElLeaderEl {
        ComputeRegionBackendServiceHaPolicyElLeaderEl {
            backend_group: core::default::Default::default(),
            network_endpoint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceHaPolicyElLeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceHaPolicyElLeaderElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceHaPolicyElLeaderElRef {
        ComputeRegionBackendServiceHaPolicyElLeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceHaPolicyElLeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_group` after provisioning.\nA fully-qualified URL of the zonal Network Endpoint Group (NEG) that the leader is\nattached to."]
    pub fn backend_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_endpoint` after provisioning.\n"]
    pub fn network_endpoint(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_endpoint", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceHaPolicyElDynamic {
    leader: Option<DynamicBlock<ComputeRegionBackendServiceHaPolicyElLeaderEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceHaPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_ip_move: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leader: Option<Vec<ComputeRegionBackendServiceHaPolicyElLeaderEl>>,
    dynamic: ComputeRegionBackendServiceHaPolicyElDynamic,
}
impl ComputeRegionBackendServiceHaPolicyEl {
    #[doc = "Set the field `fast_ip_move`.\nSpecifies whether fast IP move is enabled, and if so, the mechanism to achieve it.\nSupported values are:\n\n* 'DISABLED': Fast IP Move is disabled. You can only use the haPolicy.leader API to\n              update the leader.\n\n* 'GARP_RA': Provides a method to very quickly define a new network endpoint as the\n             leader. This method is faster than updating the leader using the\n             haPolicy.leader API. Fast IP move works as follows: The VM hosting the\n             network endpoint that should become the new leader sends either a\n             Gratuitous ARP (GARP) packet (IPv4) or an ICMPv6 Router Advertisement(RA)\n             packet (IPv6). Google Cloud immediately but temporarily associates the\n             forwarding rule IP address with that VM, and both new and in-flight packets\n             are quickly delivered to that VM. Possible values: [\"DISABLED\", \"GARP_RA\"]"]
    pub fn set_fast_ip_move(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fast_ip_move = Some(v.into());
        self
    }
    #[doc = "Set the field `leader`.\n"]
    pub fn set_leader(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceHaPolicyElLeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.leader = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.leader = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceHaPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceHaPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceHaPolicyEl {}
impl BuildComputeRegionBackendServiceHaPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceHaPolicyEl {
        ComputeRegionBackendServiceHaPolicyEl {
            fast_ip_move: core::default::Default::default(),
            leader: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceHaPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceHaPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceHaPolicyElRef {
        ComputeRegionBackendServiceHaPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceHaPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fast_ip_move` after provisioning.\nSpecifies whether fast IP move is enabled, and if so, the mechanism to achieve it.\nSupported values are:\n\n* 'DISABLED': Fast IP Move is disabled. You can only use the haPolicy.leader API to\n              update the leader.\n\n* 'GARP_RA': Provides a method to very quickly define a new network endpoint as the\n             leader. This method is faster than updating the leader using the\n             haPolicy.leader API. Fast IP move works as follows: The VM hosting the\n             network endpoint that should become the new leader sends either a\n             Gratuitous ARP (GARP) packet (IPv4) or an ICMPv6 Router Advertisement(RA)\n             packet (IPv6). Google Cloud immediately but temporarily associates the\n             forwarding rule IP address with that VM, and both new and in-flight packets\n             are quickly delivered to that VM. Possible values: [\"DISABLED\", \"GARP_RA\"]"]
    pub fn fast_ip_move(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fast_ip_move", self.base))
    }
    #[doc = "Get a reference to the value of field `leader` after provisioning.\n"]
    pub fn leader(&self) -> ListRef<ComputeRegionBackendServiceHaPolicyElLeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.leader", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceIapEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret: Option<PrimField<String>>,
}
impl ComputeRegionBackendServiceIapEl {
    #[doc = "Set the field `oauth2_client_id`.\nOAuth2 Client ID for IAP"]
    pub fn set_oauth2_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_client_secret`.\nOAuth2 Client Secret for IAP"]
    pub fn set_oauth2_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_client_secret = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceIapEl {
    type O = BlockAssignable<ComputeRegionBackendServiceIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceIapEl {
    #[doc = "Whether the serving infrastructure will authenticate and authorize all incoming requests."]
    pub enabled: PrimField<bool>,
}
impl BuildComputeRegionBackendServiceIapEl {
    pub fn build(self) -> ComputeRegionBackendServiceIapEl {
        ComputeRegionBackendServiceIapEl {
            enabled: self.enabled,
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceIapElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceIapElRef {
        ComputeRegionBackendServiceIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceIapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the serving infrastructure will authenticate and authorize all incoming requests."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\nOAuth2 Client ID for IAP"]
    pub fn oauth2_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret` after provisioning.\nOAuth2 Client Secret for IAP"]
    pub fn oauth2_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\nOAuth2 Client Secret SHA-256 for IAP"]
    pub fn oauth2_client_secret_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret_sha256", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl ComputeRegionBackendServiceLogConfigEl {
    #[doc = "Set the field `enable`.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_fields`.\nSpecifies the fields to include in logging. This field can only be specified if logging is enabled for this backend service."]
    pub fn set_optional_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.optional_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_mode`.\nSpecifies the optional logging mode for the load balancer traffic.\nSupported values: INCLUDE_ALL_OPTIONAL, EXCLUDE_ALL_OPTIONAL, CUSTOM. Possible values: [\"INCLUDE_ALL_OPTIONAL\", \"EXCLUDE_ALL_OPTIONAL\", \"CUSTOM\"]"]
    pub fn set_optional_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optional_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\nThis field can only be specified if logging is enabled for this backend service. The value of\nthe field must be in [0, 1]. This configures the sampling rate of requests to the load balancer\nwhere 1.0 means all logged requests are reported and 0.0 means no logged requests are reported.\nThe default value is 1.0."]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceLogConfigEl {
    type O = BlockAssignable<ComputeRegionBackendServiceLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceLogConfigEl {}
impl BuildComputeRegionBackendServiceLogConfigEl {
    pub fn build(self) -> ComputeRegionBackendServiceLogConfigEl {
        ComputeRegionBackendServiceLogConfigEl {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceLogConfigElRef {
        ComputeRegionBackendServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\nSpecifies the fields to include in logging. This field can only be specified if logging is enabled for this backend service."]
    pub fn optional_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.optional_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `optional_mode` after provisioning.\nSpecifies the optional logging mode for the load balancer traffic.\nSupported values: INCLUDE_ALL_OPTIONAL, EXCLUDE_ALL_OPTIONAL, CUSTOM. Possible values: [\"INCLUDE_ALL_OPTIONAL\", \"EXCLUDE_ALL_OPTIONAL\", \"CUSTOM\"]"]
    pub fn optional_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optional_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\nThis field can only be specified if logging is enabled for this backend service. The value of\nthe field must be in [0, 1]. This configures the sampling rate of requests to the load balancer\nwhere 1.0 means all logged requests are reported and 0.0 means no logged requests are reported.\nThe default value is 1.0."]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    spillover: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spillover_ratio: Option<PrimField<f64>>,
}
impl ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    #[doc = "Set the field `spillover`.\nThis field indicates whether zonal affinity is enabled or not. Default value: \"ZONAL_AFFINITY_DISABLED\" Possible values: [\"ZONAL_AFFINITY_DISABLED\", \"ZONAL_AFFINITY_SPILL_CROSS_ZONE\", \"ZONAL_AFFINITY_STAY_WITHIN_ZONE\"]"]
    pub fn set_spillover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spillover = Some(v.into());
        self
    }
    #[doc = "Set the field `spillover_ratio`.\nThe value of the field must be in [0, 1]. When the ratio of the count of healthy backend endpoints in a zone\nto the count of backend endpoints in that same zone is equal to or above this threshold, the load balancer\ndistributes new connections to all healthy endpoints in the local zone only. When the ratio of the count\nof healthy backend endpoints in a zone to the count of backend endpoints in that same zone is below this\nthreshold, the load balancer distributes all new connections to all healthy endpoints across all zones."]
    pub fn set_spillover_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.spillover_ratio = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl
{
    type O = BlockAssignable<
        ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {}
impl BuildComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    pub fn build(
        self,
    ) -> ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
        ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
            spillover: core::default::Default::default(),
            spillover_ratio: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
        ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `spillover` after provisioning.\nThis field indicates whether zonal affinity is enabled or not. Default value: \"ZONAL_AFFINITY_DISABLED\" Possible values: [\"ZONAL_AFFINITY_DISABLED\", \"ZONAL_AFFINITY_SPILL_CROSS_ZONE\", \"ZONAL_AFFINITY_STAY_WITHIN_ZONE\"]"]
    pub fn spillover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spillover", self.base))
    }
    #[doc = "Get a reference to the value of field `spillover_ratio` after provisioning.\nThe value of the field must be in [0, 1]. When the ratio of the count of healthy backend endpoints in a zone\nto the count of backend endpoints in that same zone is equal to or above this threshold, the load balancer\ndistributes new connections to all healthy endpoints in the local zone only. When the ratio of the count\nof healthy backend endpoints in a zone to the count of backend endpoints in that same zone is below this\nthreshold, the load balancer distributes all new connections to all healthy endpoints across all zones."]
    pub fn spillover_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spillover_ratio", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElDynamic {
    zonal_affinity: Option<
        DynamicBlock<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    zonal_affinity:
        Option<Vec<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl>>,
    dynamic: ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElDynamic,
}
impl ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    #[doc = "Set the field `zonal_affinity`.\n"]
    pub fn set_zonal_affinity(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zonal_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zonal_affinity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    type O = BlockAssignable<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {}
impl BuildComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    pub fn build(self) -> ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
        ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
            zonal_affinity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
        ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `zonal_affinity` after provisioning.\n"]
    pub fn zonal_affinity(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zonal_affinity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    type O = BlockAssignable<ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    pub fn build(self) -> ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
        ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
        ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    type O = BlockAssignable<ComputeRegionBackendServiceOutlierDetectionElIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    pub fn build(self) -> ComputeRegionBackendServiceOutlierDetectionElIntervalEl {
        ComputeRegionBackendServiceOutlierDetectionElIntervalEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
        ComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceOutlierDetectionElDynamic {
    base_ejection_time:
        Option<DynamicBlock<ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    interval: Option<DynamicBlock<ComputeRegionBackendServiceOutlierDetectionElIntervalEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceOutlierDetectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consecutive_errors: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consecutive_gateway_failure: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcing_consecutive_errors: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcing_consecutive_gateway_failure: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcing_success_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ejection_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_minimum_hosts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_request_volume: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_stdev_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ejection_time:
        Option<Vec<ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<Vec<ComputeRegionBackendServiceOutlierDetectionElIntervalEl>>,
    dynamic: ComputeRegionBackendServiceOutlierDetectionElDynamic,
}
impl ComputeRegionBackendServiceOutlierDetectionEl {
    #[doc = "Set the field `consecutive_errors`.\nNumber of errors before a host is ejected from the connection pool. When the\nbackend host is accessed over HTTP, a 5xx return code qualifies as an error.\nDefaults to 5."]
    pub fn set_consecutive_errors(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.consecutive_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `consecutive_gateway_failure`.\nThe number of consecutive gateway failures (502, 503, 504 status or connection\nerrors that are mapped to one of those status codes) before a consecutive\ngateway failure ejection occurs. Defaults to 5."]
    pub fn set_consecutive_gateway_failure(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.consecutive_gateway_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_consecutive_errors`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive 5xx. This setting can be used to disable\nejection or to ramp it up slowly. Defaults to 100."]
    pub fn set_enforcing_consecutive_errors(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.enforcing_consecutive_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_consecutive_gateway_failure`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive gateway failures. This setting can be\nused to disable ejection or to ramp it up slowly. Defaults to 0."]
    pub fn set_enforcing_consecutive_gateway_failure(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.enforcing_consecutive_gateway_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_success_rate`.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through success rate statistics. This setting can be used to\ndisable ejection or to ramp it up slowly. Defaults to 100."]
    pub fn set_enforcing_success_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.enforcing_success_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ejection_percent`.\nMaximum percentage of hosts in the load balancing pool for the backend service\nthat can be ejected. Defaults to 10%."]
    pub fn set_max_ejection_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ejection_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_minimum_hosts`.\nThe number of hosts in a cluster that must have enough request volume to detect\nsuccess rate outliers. If the number of hosts is less than this setting, outlier\ndetection via success rate statistics is not performed for any host in the\ncluster. Defaults to 5."]
    pub fn set_success_rate_minimum_hosts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_minimum_hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_request_volume`.\nThe minimum number of total requests that must be collected in one interval (as\ndefined by the interval duration above) to include this host in success rate\nbased outlier detection. If the volume is lower than this setting, outlier\ndetection via success rate statistics is not performed for that host. Defaults\nto 100."]
    pub fn set_success_rate_request_volume(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_request_volume = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_stdev_factor`.\nThis factor is used to determine the ejection threshold for success rate outlier\nejection. The ejection threshold is the difference between the mean success\nrate, and the product of this factor and the standard deviation of the mean\nsuccess rate: mean - (stdev * success_rate_stdev_factor). This factor is divided\nby a thousand to get a double. That is, if the desired factor is 1.9, the\nruntime value should be 1900. Defaults to 1900."]
    pub fn set_success_rate_stdev_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_stdev_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `base_ejection_time`.\n"]
    pub fn set_base_ejection_time(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.base_ejection_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.base_ejection_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceOutlierDetectionElIntervalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.interval = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.interval = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceOutlierDetectionEl {
    type O = BlockAssignable<ComputeRegionBackendServiceOutlierDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceOutlierDetectionEl {}
impl BuildComputeRegionBackendServiceOutlierDetectionEl {
    pub fn build(self) -> ComputeRegionBackendServiceOutlierDetectionEl {
        ComputeRegionBackendServiceOutlierDetectionEl {
            consecutive_errors: core::default::Default::default(),
            consecutive_gateway_failure: core::default::Default::default(),
            enforcing_consecutive_errors: core::default::Default::default(),
            enforcing_consecutive_gateway_failure: core::default::Default::default(),
            enforcing_success_rate: core::default::Default::default(),
            max_ejection_percent: core::default::Default::default(),
            success_rate_minimum_hosts: core::default::Default::default(),
            success_rate_request_volume: core::default::Default::default(),
            success_rate_stdev_factor: core::default::Default::default(),
            base_ejection_time: core::default::Default::default(),
            interval: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceOutlierDetectionElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceOutlierDetectionElRef {
        ComputeRegionBackendServiceOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceOutlierDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consecutive_errors` after provisioning.\nNumber of errors before a host is ejected from the connection pool. When the\nbackend host is accessed over HTTP, a 5xx return code qualifies as an error.\nDefaults to 5."]
    pub fn consecutive_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consecutive_errors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consecutive_gateway_failure` after provisioning.\nThe number of consecutive gateway failures (502, 503, 504 status or connection\nerrors that are mapped to one of those status codes) before a consecutive\ngateway failure ejection occurs. Defaults to 5."]
    pub fn consecutive_gateway_failure(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consecutive_gateway_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_consecutive_errors` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive 5xx. This setting can be used to disable\nejection or to ramp it up slowly. Defaults to 100."]
    pub fn enforcing_consecutive_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_consecutive_errors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_consecutive_gateway_failure` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through consecutive gateway failures. This setting can be\nused to disable ejection or to ramp it up slowly. Defaults to 0."]
    pub fn enforcing_consecutive_gateway_failure(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_consecutive_gateway_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_success_rate` after provisioning.\nThe percentage chance that a host will be actually ejected when an outlier\nstatus is detected through success rate statistics. This setting can be used to\ndisable ejection or to ramp it up slowly. Defaults to 100."]
    pub fn enforcing_success_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_success_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\nMaximum percentage of hosts in the load balancing pool for the backend service\nthat can be ejected. Defaults to 10%."]
    pub fn max_ejection_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ejection_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_minimum_hosts` after provisioning.\nThe number of hosts in a cluster that must have enough request volume to detect\nsuccess rate outliers. If the number of hosts is less than this setting, outlier\ndetection via success rate statistics is not performed for any host in the\ncluster. Defaults to 5."]
    pub fn success_rate_minimum_hosts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_minimum_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_request_volume` after provisioning.\nThe minimum number of total requests that must be collected in one interval (as\ndefined by the interval duration above) to include this host in success rate\nbased outlier detection. If the volume is lower than this setting, outlier\ndetection via success rate statistics is not performed for that host. Defaults\nto 100."]
    pub fn success_rate_request_volume(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_request_volume", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_stdev_factor` after provisioning.\nThis factor is used to determine the ejection threshold for success rate outlier\nejection. The ejection threshold is the difference between the mean success\nrate, and the product of this factor and the standard deviation of the mean\nsuccess rate: mean - (stdev * success_rate_stdev_factor). This factor is divided\nby a thousand to get a double. That is, if the desired factor is 1.9, the\nruntime value should be 1900. Defaults to 1900."]
    pub fn success_rate_stdev_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_stdev_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `base_ejection_time` after provisioning.\n"]
    pub fn base_ejection_time(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_ejection_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> ListRef<ComputeRegionBackendServiceOutlierDetectionElIntervalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeRegionBackendServiceParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the region backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceParamsEl {
    type O = BlockAssignable<ComputeRegionBackendServiceParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceParamsEl {}
impl BuildComputeRegionBackendServiceParamsEl {
    pub fn build(self) -> ComputeRegionBackendServiceParamsEl {
        ComputeRegionBackendServiceParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceParamsElRef {
        ComputeRegionBackendServiceParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the region backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    type O = BlockAssignable<ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    pub fn build(self) -> ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
        ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
        ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceStrongSessionAffinityCookieElDynamic {
    ttl: Option<DynamicBlock<ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>>,
    dynamic: ComputeRegionBackendServiceStrongSessionAffinityCookieElDynamic,
}
impl ComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    #[doc = "Set the field `name`.\nName of the cookie."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to set for the cookie."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ttl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    type O = BlockAssignable<ComputeRegionBackendServiceStrongSessionAffinityCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceStrongSessionAffinityCookieEl {}
impl BuildComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    pub fn build(self) -> ComputeRegionBackendServiceStrongSessionAffinityCookieEl {
        ComputeRegionBackendServiceStrongSessionAffinityCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
        ComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the cookie."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to set for the cookie."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<ComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionBackendServiceTimeoutsEl {
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
impl ToListMappable for ComputeRegionBackendServiceTimeoutsEl {
    type O = BlockAssignable<ComputeRegionBackendServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceTimeoutsEl {}
impl BuildComputeRegionBackendServiceTimeoutsEl {
    pub fn build(self) -> ComputeRegionBackendServiceTimeoutsEl {
        ComputeRegionBackendServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceTimeoutsElRef {
        ComputeRegionBackendServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceTimeoutsElRef {
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
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_resource_identifier: Option<PrimField<String>>,
}
impl ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[doc = "Set the field `dns_name`.\nThe SAN specified as a DNS Name."]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `uniform_resource_identifier`.\nThe SAN specified as a URI."]
    pub fn set_uniform_resource_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uniform_resource_identifier = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    type O = BlockAssignable<ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {}
impl BuildComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    pub fn build(self) -> ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
        ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
        ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe SAN specified as a DNS Name."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `uniform_resource_identifier` after provisioning.\nThe SAN specified as a URI."]
    pub fn uniform_resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uniform_resource_identifier", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceTlsSettingsElDynamic {
    subject_alt_names:
        Option<DynamicBlock<ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionBackendServiceTlsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names: Option<Vec<ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>>,
    dynamic: ComputeRegionBackendServiceTlsSettingsElDynamic,
}
impl ComputeRegionBackendServiceTlsSettingsEl {
    #[doc = "Set the field `authentication_config`.\nReference to the BackendAuthenticationConfig resource from the networksecurity.googleapis.com namespace.\nCan be used in authenticating TLS connections to the backend, as specified by the authenticationMode field.\nCan only be specified if authenticationMode is not NONE."]
    pub fn set_authentication_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sni`.\nServer Name Indication - see RFC3546 section 3.1. If set, the load balancer sends this string as the SNI hostname in the\nTLS connection to the backend, and requires that this string match a Subject Alternative Name (SAN) in the backend's\nserver certificate. With a Regional Internet NEG backend, if the SNI is specified here, the load balancer uses it\nregardless of whether the Regional Internet NEG is specified with FQDN or IP address and port."]
    pub fn set_sni(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sni = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alt_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alt_names = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionBackendServiceTlsSettingsEl {
    type O = BlockAssignable<ComputeRegionBackendServiceTlsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionBackendServiceTlsSettingsEl {}
impl BuildComputeRegionBackendServiceTlsSettingsEl {
    pub fn build(self) -> ComputeRegionBackendServiceTlsSettingsEl {
        ComputeRegionBackendServiceTlsSettingsEl {
            authentication_config: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionBackendServiceTlsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionBackendServiceTlsSettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionBackendServiceTlsSettingsElRef {
        ComputeRegionBackendServiceTlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionBackendServiceTlsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\nReference to the BackendAuthenticationConfig resource from the networksecurity.googleapis.com namespace.\nCan be used in authenticating TLS connections to the backend, as specified by the authenticationMode field.\nCan only be specified if authenticationMode is not NONE."]
    pub fn authentication_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sni` after provisioning.\nServer Name Indication - see RFC3546 section 3.1. If set, the load balancer sends this string as the SNI hostname in the\nTLS connection to the backend, and requires that this string match a Subject Alternative Name (SAN) in the backend's\nserver certificate. With a Regional Internet NEG backend, if the SNI is specified here, the load balancer uses it\nregardless of whether the Regional Internet NEG is specified with FQDN or IP address and port."]
    pub fn sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sni", self.base))
    }
    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(
        &self,
    ) -> ListRef<ComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionBackendServiceDynamic {
    backend: Option<DynamicBlock<ComputeRegionBackendServiceBackendEl>>,
    cdn_policy: Option<DynamicBlock<ComputeRegionBackendServiceCdnPolicyEl>>,
    circuit_breakers: Option<DynamicBlock<ComputeRegionBackendServiceCircuitBreakersEl>>,
    connection_tracking_policy:
        Option<DynamicBlock<ComputeRegionBackendServiceConnectionTrackingPolicyEl>>,
    consistent_hash: Option<DynamicBlock<ComputeRegionBackendServiceConsistentHashEl>>,
    custom_metrics: Option<DynamicBlock<ComputeRegionBackendServiceCustomMetricsEl>>,
    failover_policy: Option<DynamicBlock<ComputeRegionBackendServiceFailoverPolicyEl>>,
    ha_policy: Option<DynamicBlock<ComputeRegionBackendServiceHaPolicyEl>>,
    iap: Option<DynamicBlock<ComputeRegionBackendServiceIapEl>>,
    log_config: Option<DynamicBlock<ComputeRegionBackendServiceLogConfigEl>>,
    network_pass_through_lb_traffic_policy:
        Option<DynamicBlock<ComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl>>,
    outlier_detection: Option<DynamicBlock<ComputeRegionBackendServiceOutlierDetectionEl>>,
    params: Option<DynamicBlock<ComputeRegionBackendServiceParamsEl>>,
    strong_session_affinity_cookie:
        Option<DynamicBlock<ComputeRegionBackendServiceStrongSessionAffinityCookieEl>>,
    tls_settings: Option<DynamicBlock<ComputeRegionBackendServiceTlsSettingsEl>>,
}
