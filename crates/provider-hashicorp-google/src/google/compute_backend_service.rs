use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeBackendServiceData {
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
    compression_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_draining_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_request_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_response_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cdn: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_managed_migration_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_managed_migration_testing_percentage: Option<PrimField<f64>>,
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
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_lb_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<Vec<ComputeBackendServiceBackendEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdn_policy: Option<Vec<ComputeBackendServiceCdnPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_breakers: Option<Vec<ComputeBackendServiceCircuitBreakersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consistent_hash: Option<Vec<ComputeBackendServiceConsistentHashEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<Vec<ComputeBackendServiceCustomMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iap: Option<Vec<ComputeBackendServiceIapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality_lb_policies: Option<Vec<ComputeBackendServiceLocalityLbPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeBackendServiceLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_stream_duration: Option<Vec<ComputeBackendServiceMaxStreamDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outlier_detection: Option<Vec<ComputeBackendServiceOutlierDetectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeBackendServiceParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<Vec<ComputeBackendServiceSecuritySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strong_session_affinity_cookie: Option<Vec<ComputeBackendServiceStrongSessionAffinityCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeBackendServiceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<Vec<ComputeBackendServiceTlsSettingsEl>>,
    dynamic: ComputeBackendServiceDynamic,
}
struct ComputeBackendService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeBackendServiceData>,
}
#[derive(Clone)]
pub struct ComputeBackendService(Rc<ComputeBackendService_>);
impl ComputeBackendService {
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
    #[doc = "Set the field `compression_mode`.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn set_compression_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compression_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_draining_timeout_sec`.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn set_connection_draining_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connection_draining_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_request_headers`.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn set_custom_request_headers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_response_headers`.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn set_custom_response_headers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_response_headers = Some(v.into());
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
    #[doc = "Set the field `edge_security_policy`.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn set_edge_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_cdn`.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn set_enable_cdn(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cdn = Some(v.into());
        self
    }
    #[doc = "Set the field `external_managed_migration_state`.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn set_external_managed_migration_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_managed_migration_state = Some(v.into());
        self
    }
    #[doc = "Set the field `external_managed_migration_testing_percentage`.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn set_external_managed_migration_testing_percentage(
        self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .external_managed_migration_testing_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `health_checks`.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
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
    #[doc = "Set the field `load_balancing_scheme`.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `locality_lb_policy`.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn set_locality_lb_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().locality_lb_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
    pub fn set_port_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `security_policy`.\nThe security policy associated with this backend service."]
    pub fn set_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `service_lb_policy`.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global."]
    pub fn set_service_lb_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_lb_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `session_affinity`.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
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
        v: impl Into<BlockAssignable<ComputeBackendServiceBackendEl>>,
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
        v: impl Into<BlockAssignable<ComputeBackendServiceCdnPolicyEl>>,
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
        v: impl Into<BlockAssignable<ComputeBackendServiceCircuitBreakersEl>>,
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
    #[doc = "Set the field `consistent_hash`.\n"]
    pub fn set_consistent_hash(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceConsistentHashEl>>,
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
        v: impl Into<BlockAssignable<ComputeBackendServiceCustomMetricsEl>>,
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
    #[doc = "Set the field `iap`.\n"]
    pub fn set_iap(self, v: impl Into<BlockAssignable<ComputeBackendServiceIapEl>>) -> Self {
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
    #[doc = "Set the field `locality_lb_policies`.\n"]
    pub fn set_locality_lb_policies(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceLocalityLbPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().locality_lb_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.locality_lb_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceLogConfigEl>>,
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
    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceMaxStreamDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_stream_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_stream_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceOutlierDetectionEl>>,
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
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeBackendServiceParamsEl>>) -> Self {
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
    #[doc = "Set the field `security_settings`.\n"]
    pub fn set_security_settings(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceSecuritySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `strong_session_affinity_cookie`.\n"]
    pub fn set_strong_session_affinity_cookie(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceStrongSessionAffinityCookieEl>>,
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
    pub fn set_timeouts(self, v: impl Into<ComputeBackendServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_settings`.\n"]
    pub fn set_tls_settings(
        self,
        v: impl Into<BlockAssignable<ComputeBackendServiceTlsSettingsEl>>,
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
    #[doc = "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_mode", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `custom_request_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn custom_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.custom_request_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn custom_response_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.custom_response_headers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_managed_migration_state` after provisioning.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn external_managed_migration_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_managed_migration_testing_percentage` after provisioning.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn external_managed_migration_testing_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.external_managed_migration_testing_percentage",
                self.extract_ref()
            ),
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
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
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
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
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
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_lb_policy` after provisioning.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global."]
    pub fn service_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
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
    pub fn cdn_policy(&self) -> ListRef<ComputeBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> ListRef<ComputeBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> ListRef<ComputeBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<ComputeBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<ComputeBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\n"]
    pub fn locality_lb_policies(&self) -> ListRef<ComputeBackendServiceLocalityLbPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locality_lb_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(&self) -> ListRef<ComputeBackendServiceMaxStreamDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_stream_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<ComputeBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(&self) -> ListRef<ComputeBackendServiceSecuritySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<ComputeBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendServiceTimeoutsElRef {
        ComputeBackendServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> ListRef<ComputeBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
impl Referable for ComputeBackendService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeBackendService {}
impl ToListMappable for ComputeBackendService {
    type O = ListRef<ComputeBackendServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeBackendService_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_backend_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeBackendService {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeBackendService {
    pub fn build(self, stack: &mut Stack) -> ComputeBackendService {
        let out = ComputeBackendService(Rc::new(ComputeBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeBackendServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                affinity_cookie_ttl_sec: core::default::Default::default(),
                compression_mode: core::default::Default::default(),
                connection_draining_timeout_sec: core::default::Default::default(),
                custom_request_headers: core::default::Default::default(),
                custom_response_headers: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                enable_cdn: core::default::Default::default(),
                external_managed_migration_state: core::default::Default::default(),
                external_managed_migration_testing_percentage: core::default::Default::default(),
                health_checks: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_selection_policy: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                locality_lb_policy: core::default::Default::default(),
                name: self.name,
                port_name: core::default::Default::default(),
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                security_policy: core::default::Default::default(),
                service_lb_policy: core::default::Default::default(),
                session_affinity: core::default::Default::default(),
                timeout_sec: core::default::Default::default(),
                backend: core::default::Default::default(),
                cdn_policy: core::default::Default::default(),
                circuit_breakers: core::default::Default::default(),
                consistent_hash: core::default::Default::default(),
                custom_metrics: core::default::Default::default(),
                iap: core::default::Default::default(),
                locality_lb_policies: core::default::Default::default(),
                log_config: core::default::Default::default(),
                max_stream_duration: core::default::Default::default(),
                outlier_detection: core::default::Default::default(),
                params: core::default::Default::default(),
                security_settings: core::default::Default::default(),
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
pub struct ComputeBackendServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeBackendServiceRef {
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
    #[doc = "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_mode", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `custom_request_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nrequests."]
    pub fn custom_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.custom_request_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied\nresponses."]
    pub fn custom_response_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.custom_response_headers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe resource URL for the edge security policy associated with this backend service."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendService."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_managed_migration_state` after provisioning.\nSpecifies the canary migration state. Possible values are PREPARE, TEST_BY_PERCENTAGE, and\nTEST_ALL_TRAFFIC.\n\nTo begin the migration from EXTERNAL to EXTERNAL_MANAGED, the state must be changed to\nPREPARE. The state must be changed to TEST_ALL_TRAFFIC before the loadBalancingScheme can be\nchanged to EXTERNAL_MANAGED. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate\ntraffic by percentage using externalManagedMigrationTestingPercentage.\n\nRolling back a migration requires the states to be set in reverse order. So changing the\nscheme from EXTERNAL_MANAGED to EXTERNAL requires the state to be set to TEST_ALL_TRAFFIC at\nthe same time. Optionally, the TEST_BY_PERCENTAGE state can be used to migrate some traffic\nback to EXTERNAL or PREPARE can be used to migrate all traffic back to EXTERNAL. Possible values: [\"PREPARE\", \"TEST_BY_PERCENTAGE\", \"TEST_ALL_TRAFFIC\"]"]
    pub fn external_managed_migration_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_managed_migration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_managed_migration_testing_percentage` after provisioning.\nDetermines the fraction of requests that should be processed by the Global external\nApplication Load Balancer.\n\nThe value of this field must be in the range [0, 100].\n\nSession affinity options will slightly affect this routing behavior, for more details,\nsee: Session Affinity.\n\nThis value can only be set if the loadBalancingScheme in the backend service is set to\nEXTERNAL (when using the Classic ALB) and the migration state is TEST_BY_PERCENTAGE."]
    pub fn external_managed_migration_testing_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.external_managed_migration_testing_percentage",
                self.extract_ref()
            ),
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
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to the HttpHealthCheck or HttpsHealthCheck resource\nfor health checking this BackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend.\n\nFor internal load balancing, a URL to a HealthCheck resource must be specified instead."]
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
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nIndicates whether the backend service will be used with internal or\nexternal load balancing. A backend service created for one type of\nload balancing cannot be used with the other. For more information, refer to\n[Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service). Default value: \"EXTERNAL\" Possible values: [\"EXTERNAL\", \"INTERNAL_SELF_MANAGED\", \"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
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
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nName of backend port. The same name should appear in the instance\ngroups referenced by this service. Required when the load balancing\nscheme is EXTERNAL."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol this BackendService uses to communicate with backends.\nThe default is HTTP. Possible values are HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP\nor GRPC. Refer to the documentation for the load balancers or for Traffic Director\nfor more information. Must be set to GRPC when the backend service is referenced\nby a URL map that is bound to target gRPC proxy. Possible values: [\"HTTP\", \"HTTPS\", \"HTTP2\", \"TCP\", \"SSL\", \"UDP\", \"GRPC\", \"UNSPECIFIED\", \"H2C\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_lb_policy` after provisioning.\nURL to networkservices.ServiceLbPolicy resource.\nCan only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global."]
    pub fn service_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nType of session affinity to use. The default is NONE. Session affinity is\nnot applicable if the protocol is UDP. Possible values: [\"NONE\", \"CLIENT_IP\", \"CLIENT_IP_PORT_PROTO\", \"CLIENT_IP_PROTO\", \"GENERATED_COOKIE\", \"HEADER_FIELD\", \"HTTP_COOKIE\", \"STRONG_COOKIE_AFFINITY\"]"]
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
    pub fn cdn_policy(&self) -> ListRef<ComputeBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\n"]
    pub fn circuit_breakers(&self) -> ListRef<ComputeBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\n"]
    pub fn consistent_hash(&self) -> ListRef<ComputeBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<ComputeBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<ComputeBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\n"]
    pub fn locality_lb_policies(&self) -> ListRef<ComputeBackendServiceLocalityLbPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locality_lb_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(&self) -> ListRef<ComputeBackendServiceMaxStreamDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_stream_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<ComputeBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(&self) -> ListRef<ComputeBackendServiceSecuritySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\n"]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<ComputeBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendServiceTimeoutsElRef {
        ComputeBackendServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\n"]
    pub fn tls_settings(&self) -> ListRef<ComputeBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceBackendElCustomMetricsEl {
    dry_run: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_utilization: Option<PrimField<f64>>,
    name: PrimField<String>,
}
impl ComputeBackendServiceBackendElCustomMetricsEl {
    #[doc = "Set the field `max_utilization`.\nOptional parameter to define a target utilization for the Custom Metrics\nbalancing mode. The valid range is <code>[0.0, 1.0]</code>."]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceBackendElCustomMetricsEl {
    type O = BlockAssignable<ComputeBackendServiceBackendElCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceBackendElCustomMetricsEl {
    #[doc = "If true, the metric data is collected and reported to Cloud\nMonitoring, but is not used for load balancing."]
    pub dry_run: PrimField<bool>,
    #[doc = "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: PrimField<String>,
}
impl BuildComputeBackendServiceBackendElCustomMetricsEl {
    pub fn build(self) -> ComputeBackendServiceBackendElCustomMetricsEl {
        ComputeBackendServiceBackendElCustomMetricsEl {
            dry_run: self.dry_run,
            max_utilization: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct ComputeBackendServiceBackendElCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceBackendElCustomMetricsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceBackendElCustomMetricsElRef {
        ComputeBackendServiceBackendElCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceBackendElCustomMetricsElRef {
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
struct ComputeBackendServiceBackendElDynamic {
    custom_metrics: Option<DynamicBlock<ComputeBackendServiceBackendElCustomMetricsEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    balancing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_scaler: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
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
    preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<Vec<ComputeBackendServiceBackendElCustomMetricsEl>>,
    dynamic: ComputeBackendServiceBackendElDynamic,
}
impl ComputeBackendServiceBackendEl {
    #[doc = "Set the field `balancing_mode`.\nSpecifies the balancing mode for this backend.\n\nFor global HTTP(S) or TCP/SSL load balancing, the default is\nUTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S)),\nCUSTOM_METRICS (for HTTP(s)) and CONNECTION (for TCP/SSL).\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\", \"IN_FLIGHT\"]"]
    pub fn set_balancing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.balancing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_scaler`.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\nDefault value is 1, which means the group will serve up to 100%\nof its configured capacity (depending on balancingMode). A\nsetting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
    pub fn set_capacity_scaler(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capacity_scaler = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource.\nProvide this property when you create the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections`.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_endpoint`.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either\nmaxConnections or maxConnectionsPerEndpoint must be set."]
    pub fn set_max_connections_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_instance`.\nThe max number of simultaneous connections that a single\nbackend instance can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn set_max_connections_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate`.\nThe max requests per second (RPS) of the group.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. For RATE mode, either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn set_max_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_endpoint`.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set."]
    pub fn set_max_rate_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_instance`.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set."]
    pub fn set_max_rate_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_utilization`.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0]."]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
    #[doc = "Set the field `preference`.\nThis field indicates whether this backend should be fully utilized before sending traffic to backends\nwith default preference. This field cannot be set when loadBalancingScheme is set to 'EXTERNAL'. The possible values are:\n  - PREFERRED: Backends with this preference level will be filled up to their capacity limits first,\n    based on RTT.\n  - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and\n    traffic would be assigned based on the load balancing algorithm you use. This is the default Possible values: [\"PREFERRED\", \"DEFAULT\"]"]
    pub fn set_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preference = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceBackendElCustomMetricsEl>>,
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
impl ToListMappable for ComputeBackendServiceBackendEl {
    type O = BlockAssignable<ComputeBackendServiceBackendEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceBackendEl {
    #[doc = "The fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub group: PrimField<String>,
}
impl BuildComputeBackendServiceBackendEl {
    pub fn build(self) -> ComputeBackendServiceBackendEl {
        ComputeBackendServiceBackendEl {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            description: core::default::Default::default(),
            group: self.group,
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            preference: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceBackendElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceBackendElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceBackendElRef {
        ComputeBackendServiceBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceBackendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `balancing_mode` after provisioning.\nSpecifies the balancing mode for this backend.\n\nFor global HTTP(S) or TCP/SSL load balancing, the default is\nUTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S)),\nCUSTOM_METRICS (for HTTP(s)) and CONNECTION (for TCP/SSL).\n\nSee the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)\nfor an explanation of load balancing modes. Default value: \"UTILIZATION\" Possible values: [\"UTILIZATION\", \"RATE\", \"CONNECTION\", \"CUSTOM_METRICS\", \"IN_FLIGHT\"]"]
    pub fn balancing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.balancing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_scaler` after provisioning.\nA multiplier applied to the group's maximum servicing capacity\n(based on UTILIZATION, RATE or CONNECTION).\n\nDefault value is 1, which means the group will serve up to 100%\nof its configured capacity (depending on balancingMode). A\nsetting of 0 means the group is completely drained, offering\n0% of its available Capacity. Valid range is [0.0,1.0]."]
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
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe fully-qualified URL of an Instance Group or Network Endpoint\nGroup resource. In case of instance group this defines the list\nof instances that serve traffic. Member virtual machine\ninstances from each instance group must live in the same zone as\nthe instance group itself. No two backends in a backend service\nare allowed to use same Instance Group resource.\n\nFor Network Endpoint Groups this defines list of endpoints. All\nendpoints of Network Endpoint Group must be hosted on instances\nlocated in the same zone as the Network Endpoint Group.\n\nBackend services cannot mix Instance Group and\nNetwork Endpoint Group backends.\n\nNote that you must specify an Instance Group or Network Endpoint\nGroup resource using the fully-qualified URL, rather than a\npartial URL."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\nThe max number of simultaneous connections for the group. Can\nbe used with either CONNECTION or UTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or one\nof maxConnectionsPerInstance or maxConnectionsPerEndpoint,\nas appropriate for group type, must be set."]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_endpoint` after provisioning.\nThe max number of simultaneous connections that a single backend\nnetwork endpoint can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either\nmaxConnections or maxConnectionsPerEndpoint must be set."]
    pub fn max_connections_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_instance` after provisioning.\nThe max number of simultaneous connections that a single\nbackend instance can handle. This is used to calculate the\ncapacity of the group. Can be used in either CONNECTION or\nUTILIZATION balancing modes.\n\nFor CONNECTION mode, either maxConnections or\nmaxConnectionsPerInstance must be set."]
    pub fn max_connections_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate` after provisioning.\nThe max requests per second (RPS) of the group.\n\nCan be used with either RATE or UTILIZATION balancing modes,\nbut required if RATE mode. For RATE mode, either maxRate or one\nof maxRatePerInstance or maxRatePerEndpoint, as appropriate for\ngroup type, must be set."]
    pub fn max_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `max_rate_per_endpoint` after provisioning.\nThe max requests per second (RPS) that a single backend network\nendpoint can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerEndpoint must be set."]
    pub fn max_rate_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate_per_instance` after provisioning.\nThe max requests per second (RPS) that a single backend\ninstance can handle. This is used to calculate the capacity of\nthe group. Can be used in either balancing mode. For RATE mode,\neither maxRate or maxRatePerInstance must be set."]
    pub fn max_rate_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\nUsed when balancingMode is UTILIZATION. This ratio defines the\nCPU utilization target for the group. Valid range is [0.0, 1.0]."]
    pub fn max_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preference` after provisioning.\nThis field indicates whether this backend should be fully utilized before sending traffic to backends\nwith default preference. This field cannot be set when loadBalancingScheme is set to 'EXTERNAL'. The possible values are:\n  - PREFERRED: Backends with this preference level will be filled up to their capacity limits first,\n    based on RTT.\n  - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and\n    traffic would be assigned based on the load balancing algorithm you use. This is the default Possible values: [\"PREFERRED\", \"DEFAULT\"]"]
    pub fn preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<ComputeBackendServiceBackendElCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    header_name: PrimField<String>,
}
impl ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {}
impl ToListMappable for ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    type O = BlockAssignable<ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[doc = "The header field name to match on when bypassing cache. Values are case-insensitive."]
    pub header_name: PrimField<String>,
}
impl BuildComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    pub fn build(self) -> ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
        ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
            header_name: self.header_name,
        }
    }
}
pub struct ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
        ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe header field name to match on when bypassing cache. Values are case-insensitive."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_host: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_http_headers: Option<ListField<PrimField<String>>>,
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
impl ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_host`.\nIf true requests to different hosts will be cached separately."]
    pub fn set_include_host(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_host = Some(v.into());
        self
    }
    #[doc = "Set the field `include_http_headers`.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn set_include_http_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_http_headers = Some(v.into());
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
impl ToListMappable for ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {}
impl BuildComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
        ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
            include_host: core::default::Default::default(),
            include_http_headers: core::default::Default::default(),
            include_named_cookies: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            include_query_string: core::default::Default::default(),
            query_string_blacklist: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
        ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_host` after provisioning.\nIf true requests to different hosts will be cached separately."]
    pub fn include_host(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }
    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn include_http_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_http_headers", self.base),
        )
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
pub struct ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}
impl ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {}
impl BuildComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
        ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
        ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendServiceCdnPolicyElDynamic {
    bypass_cache_on_request_headers:
        Option<DynamicBlock<ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    cache_key_policy: Option<DynamicBlock<ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    negative_caching_policy:
        Option<DynamicBlock<ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceCdnPolicyEl {
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
    request_coalescing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_cache_on_request_headers:
        Option<Vec<ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<Vec<ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching_policy: Option<Vec<ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    dynamic: ComputeBackendServiceCdnPolicyElDynamic,
}
impl ComputeBackendServiceCdnPolicyEl {
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
    #[doc = "Set the field `request_coalescing`.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests\nto the origin."]
    pub fn set_request_coalescing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_coalescing = Some(v.into());
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
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bypass_cache_on_request_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bypass_cache_on_request_headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
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
        v: impl Into<BlockAssignable<ComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
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
impl ToListMappable for ComputeBackendServiceCdnPolicyEl {
    type O = BlockAssignable<ComputeBackendServiceCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCdnPolicyEl {}
impl BuildComputeBackendServiceCdnPolicyEl {
    pub fn build(self) -> ComputeBackendServiceCdnPolicyEl {
        ComputeBackendServiceCdnPolicyEl {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceCdnPolicyElRef {
        ComputeBackendServiceCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCdnPolicyElRef {
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
    #[doc = "Get a reference to the value of field `request_coalescing` after provisioning.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests\nto the origin."]
    pub fn request_coalescing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_coalescing", self.base),
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
    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> ListRef<ComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bypass_cache_on_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> ListRef<ComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> ListRef<ComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceCircuitBreakersEl {
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
impl ComputeBackendServiceCircuitBreakersEl {
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
impl ToListMappable for ComputeBackendServiceCircuitBreakersEl {
    type O = BlockAssignable<ComputeBackendServiceCircuitBreakersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCircuitBreakersEl {}
impl BuildComputeBackendServiceCircuitBreakersEl {
    pub fn build(self) -> ComputeBackendServiceCircuitBreakersEl {
        ComputeBackendServiceCircuitBreakersEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceCircuitBreakersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCircuitBreakersElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceCircuitBreakersElRef {
        ComputeBackendServiceCircuitBreakersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCircuitBreakersElRef {
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
pub struct ComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    type O = BlockAssignable<ComputeBackendServiceConsistentHashElHttpCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    pub fn build(self) -> ComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
        ComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
        ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
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
struct ComputeBackendServiceConsistentHashElHttpCookieElDynamic {
    ttl: Option<DynamicBlock<ComputeBackendServiceConsistentHashElHttpCookieElTtlEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceConsistentHashElHttpCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<ComputeBackendServiceConsistentHashElHttpCookieElTtlEl>>,
    dynamic: ComputeBackendServiceConsistentHashElHttpCookieElDynamic,
}
impl ComputeBackendServiceConsistentHashElHttpCookieEl {
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
        v: impl Into<BlockAssignable<ComputeBackendServiceConsistentHashElHttpCookieElTtlEl>>,
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
impl ToListMappable for ComputeBackendServiceConsistentHashElHttpCookieEl {
    type O = BlockAssignable<ComputeBackendServiceConsistentHashElHttpCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceConsistentHashElHttpCookieEl {}
impl BuildComputeBackendServiceConsistentHashElHttpCookieEl {
    pub fn build(self) -> ComputeBackendServiceConsistentHashElHttpCookieEl {
        ComputeBackendServiceConsistentHashElHttpCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceConsistentHashElHttpCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceConsistentHashElHttpCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceConsistentHashElHttpCookieElRef {
        ComputeBackendServiceConsistentHashElHttpCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceConsistentHashElHttpCookieElRef {
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
    pub fn ttl(&self) -> ListRef<ComputeBackendServiceConsistentHashElHttpCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendServiceConsistentHashElDynamic {
    http_cookie: Option<DynamicBlock<ComputeBackendServiceConsistentHashElHttpCookieEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceConsistentHashEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_ring_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_cookie: Option<Vec<ComputeBackendServiceConsistentHashElHttpCookieEl>>,
    dynamic: ComputeBackendServiceConsistentHashElDynamic,
}
impl ComputeBackendServiceConsistentHashEl {
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
        v: impl Into<BlockAssignable<ComputeBackendServiceConsistentHashElHttpCookieEl>>,
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
impl ToListMappable for ComputeBackendServiceConsistentHashEl {
    type O = BlockAssignable<ComputeBackendServiceConsistentHashEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceConsistentHashEl {}
impl BuildComputeBackendServiceConsistentHashEl {
    pub fn build(self) -> ComputeBackendServiceConsistentHashEl {
        ComputeBackendServiceConsistentHashEl {
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
            http_cookie: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceConsistentHashElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceConsistentHashElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceConsistentHashElRef {
        ComputeBackendServiceConsistentHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceConsistentHashElRef {
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
    pub fn http_cookie(&self) -> ListRef<ComputeBackendServiceConsistentHashElHttpCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_cookie", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceCustomMetricsEl {
    dry_run: PrimField<bool>,
    name: PrimField<String>,
}
impl ComputeBackendServiceCustomMetricsEl {}
impl ToListMappable for ComputeBackendServiceCustomMetricsEl {
    type O = BlockAssignable<ComputeBackendServiceCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceCustomMetricsEl {
    #[doc = "If true, the metric data is not used for load balancing."]
    pub dry_run: PrimField<bool>,
    #[doc = "Name of a custom utilization signal. The name must be 1-64 characters\nlong and match the regular expression [a-z]([-_.a-z0-9]*[a-z0-9])? which\nmeans the first character must be a lowercase letter, and all following\ncharacters must be a dash, period, underscore, lowercase letter, or\ndigit, except the last character, which cannot be a dash, period, or\nunderscore. For usage guidelines, see Custom Metrics balancing mode. This\nfield can only be used for a global or regional backend service with the\nloadBalancingScheme set to <code>EXTERNAL_MANAGED</code>,\n<code>INTERNAL_MANAGED</code> <code>INTERNAL_SELF_MANAGED</code>."]
    pub name: PrimField<String>,
}
impl BuildComputeBackendServiceCustomMetricsEl {
    pub fn build(self) -> ComputeBackendServiceCustomMetricsEl {
        ComputeBackendServiceCustomMetricsEl {
            dry_run: self.dry_run,
            name: self.name,
        }
    }
}
pub struct ComputeBackendServiceCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceCustomMetricsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceCustomMetricsElRef {
        ComputeBackendServiceCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceCustomMetricsElRef {
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
pub struct ComputeBackendServiceIapEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret: Option<PrimField<String>>,
}
impl ComputeBackendServiceIapEl {
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
impl ToListMappable for ComputeBackendServiceIapEl {
    type O = BlockAssignable<ComputeBackendServiceIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceIapEl {
    #[doc = "Whether the serving infrastructure will authenticate and authorize all incoming requests."]
    pub enabled: PrimField<bool>,
}
impl BuildComputeBackendServiceIapEl {
    pub fn build(self) -> ComputeBackendServiceIapEl {
        ComputeBackendServiceIapEl {
            enabled: self.enabled,
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceIapElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceIapElRef {
        ComputeBackendServiceIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceIapElRef {
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
pub struct ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    #[doc = "Set the field `data`.\nAn optional, arbitrary JSON object with configuration data, understood\nby a locally installed custom policy implementation."]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    type O = BlockAssignable<ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    #[doc = "Identifies the custom policy.\n\nThe value should match the type the custom implementation is registered\nwith on the gRPC clients. It should follow protocol buffer\nmessage naming conventions and include the full path (e.g.\nmyorg.CustomLbPolicy). The maximum length is 256 characters.\n\nNote that specifying the same custom policy more than once for a\nbackend is not a valid configuration and will be rejected."]
    pub name: PrimField<String>,
}
impl BuildComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    pub fn build(self) -> ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
        ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
            data: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
        ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\nAn optional, arbitrary JSON object with configuration data, understood\nby a locally installed custom policy implementation."]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifies the custom policy.\n\nThe value should match the type the custom implementation is registered\nwith on the gRPC clients. It should follow protocol buffer\nmessage naming conventions and include the full path (e.g.\nmyorg.CustomLbPolicy). The maximum length is 256 characters.\n\nNote that specifying the same custom policy more than once for a\nbackend is not a valid configuration and will be rejected."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    name: PrimField<String>,
}
impl ComputeBackendServiceLocalityLbPoliciesElPolicyEl {}
impl ToListMappable for ComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    type O = BlockAssignable<ComputeBackendServiceLocalityLbPoliciesElPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    #[doc = "The name of a locality load balancer policy to be used. The value\nshould be one of the predefined ones as supported by localityLbPolicy,\nalthough at the moment only ROUND_ROBIN is supported.\n\nThis field should only be populated when the customPolicy field is not\nused.\n\nNote that specifying the same policy more than once for a backend is\nnot a valid configuration and will be rejected.\n\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                  hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n              hashing to backends. The algorithm has the property that the\n              addition/removal of a host from a set of N hosts only affects\n              1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824 Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\"]"]
    pub name: PrimField<String>,
}
impl BuildComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    pub fn build(self) -> ComputeBackendServiceLocalityLbPoliciesElPolicyEl {
        ComputeBackendServiceLocalityLbPoliciesElPolicyEl { name: self.name }
    }
}
pub struct ComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
        ComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a locality load balancer policy to be used. The value\nshould be one of the predefined ones as supported by localityLbPolicy,\nalthough at the moment only ROUND_ROBIN is supported.\n\nThis field should only be populated when the customPolicy field is not\nused.\n\nNote that specifying the same policy more than once for a backend is\nnot a valid configuration and will be rejected.\n\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                  hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n              hashing to backends. The algorithm has the property that the\n              addition/removal of a host from a set of N hosts only affects\n              1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824 Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\"]"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendServiceLocalityLbPoliciesElDynamic {
    custom_policy: Option<DynamicBlock<ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>>,
    policy: Option<DynamicBlock<ComputeBackendServiceLocalityLbPoliciesElPolicyEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceLocalityLbPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_policy: Option<Vec<ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<Vec<ComputeBackendServiceLocalityLbPoliciesElPolicyEl>>,
    dynamic: ComputeBackendServiceLocalityLbPoliciesElDynamic,
}
impl ComputeBackendServiceLocalityLbPoliciesEl {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceLocalityLbPoliciesElPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeBackendServiceLocalityLbPoliciesEl {
    type O = BlockAssignable<ComputeBackendServiceLocalityLbPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceLocalityLbPoliciesEl {}
impl BuildComputeBackendServiceLocalityLbPoliciesEl {
    pub fn build(self) -> ComputeBackendServiceLocalityLbPoliciesEl {
        ComputeBackendServiceLocalityLbPoliciesEl {
            custom_policy: core::default::Default::default(),
            policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceLocalityLbPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceLocalityLbPoliciesElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceLocalityLbPoliciesElRef {
        ComputeBackendServiceLocalityLbPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceLocalityLbPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(
        &self,
    ) -> ListRef<ComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> ListRef<ComputeBackendServiceLocalityLbPoliciesElPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl ComputeBackendServiceLogConfigEl {
    #[doc = "Set the field `enable`.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_fields`.\nThis field can only be specified if logging is enabled for this backend service and \"logConfig.optionalMode\"\nwas set to CUSTOM. Contains a list of optional fields you want to include in the logs.\nFor example: serverInstance, serverGkeDetails.cluster, serverGkeDetails.pod.podNamespace\nFor example: orca_load_report, tls.protocol"]
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
impl ToListMappable for ComputeBackendServiceLogConfigEl {
    type O = BlockAssignable<ComputeBackendServiceLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceLogConfigEl {}
impl BuildComputeBackendServiceLogConfigEl {
    pub fn build(self) -> ComputeBackendServiceLogConfigEl {
        ComputeBackendServiceLogConfigEl {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceLogConfigElRef {
        ComputeBackendServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nWhether to enable logging for the load balancer traffic served by this backend service."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\nThis field can only be specified if logging is enabled for this backend service and \"logConfig.optionalMode\"\nwas set to CUSTOM. Contains a list of optional fields you want to include in the logs.\nFor example: serverInstance, serverGkeDetails.cluster, serverGkeDetails.pod.podNamespace\nFor example: orca_load_report, tls.protocol"]
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
pub struct ComputeBackendServiceMaxStreamDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeBackendServiceMaxStreamDurationEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution.\nDurations less than one second are represented with a 0 seconds field and a positive nanos field.\nMust be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceMaxStreamDurationEl {
    type O = BlockAssignable<ComputeBackendServiceMaxStreamDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceMaxStreamDurationEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. (int64 format)"]
    pub seconds: PrimField<String>,
}
impl BuildComputeBackendServiceMaxStreamDurationEl {
    pub fn build(self) -> ComputeBackendServiceMaxStreamDurationEl {
        ComputeBackendServiceMaxStreamDurationEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeBackendServiceMaxStreamDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceMaxStreamDurationElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceMaxStreamDurationElRef {
        ComputeBackendServiceMaxStreamDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceMaxStreamDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution.\nDurations less than one second are represented with a 0 seconds field and a positive nanos field.\nMust be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. (int64 format)"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    type O = BlockAssignable<ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    pub fn build(self) -> ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
        ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
        ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
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
pub struct ComputeBackendServiceOutlierDetectionElIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeBackendServiceOutlierDetectionElIntervalEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceOutlierDetectionElIntervalEl {
    type O = BlockAssignable<ComputeBackendServiceOutlierDetectionElIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceOutlierDetectionElIntervalEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeBackendServiceOutlierDetectionElIntervalEl {
    pub fn build(self) -> ComputeBackendServiceOutlierDetectionElIntervalEl {
        ComputeBackendServiceOutlierDetectionElIntervalEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeBackendServiceOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceOutlierDetectionElIntervalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceOutlierDetectionElIntervalElRef {
        ComputeBackendServiceOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceOutlierDetectionElIntervalElRef {
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
struct ComputeBackendServiceOutlierDetectionElDynamic {
    base_ejection_time:
        Option<DynamicBlock<ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    interval: Option<DynamicBlock<ComputeBackendServiceOutlierDetectionElIntervalEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceOutlierDetectionEl {
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
    base_ejection_time: Option<Vec<ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<Vec<ComputeBackendServiceOutlierDetectionElIntervalEl>>,
    dynamic: ComputeBackendServiceOutlierDetectionElDynamic,
}
impl ComputeBackendServiceOutlierDetectionEl {
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
        v: impl Into<BlockAssignable<ComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
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
        v: impl Into<BlockAssignable<ComputeBackendServiceOutlierDetectionElIntervalEl>>,
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
impl ToListMappable for ComputeBackendServiceOutlierDetectionEl {
    type O = BlockAssignable<ComputeBackendServiceOutlierDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceOutlierDetectionEl {}
impl BuildComputeBackendServiceOutlierDetectionEl {
    pub fn build(self) -> ComputeBackendServiceOutlierDetectionEl {
        ComputeBackendServiceOutlierDetectionEl {
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
pub struct ComputeBackendServiceOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceOutlierDetectionElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceOutlierDetectionElRef {
        ComputeBackendServiceOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceOutlierDetectionElRef {
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
    ) -> ListRef<ComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_ejection_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> ListRef<ComputeBackendServiceOutlierDetectionElIntervalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeBackendServiceParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceParamsEl {
    type O = BlockAssignable<ComputeBackendServiceParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceParamsEl {}
impl BuildComputeBackendServiceParamsEl {
    pub fn build(self) -> ComputeBackendServiceParamsEl {
        ComputeBackendServiceParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceParamsElRef {
        ComputeBackendServiceParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the backend service. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_region: Option<PrimField<String>>,
}
impl ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    #[doc = "Set the field `access_key`.\nThe access key used for s3 bucket authentication.\nRequired for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request."]
    pub fn set_access_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key = Some(v.into());
        self
    }
    #[doc = "Set the field `access_key_id`.\nThe identifier of an access key used for s3 bucket authentication."]
    pub fn set_access_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `access_key_version`.\nThe optional version identifier for the access key. You can use this to keep track of different iterations of your access key."]
    pub fn set_access_key_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key_version = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_region`.\nThe name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.\nFor example, \"us-east-1\" for AWS or \"us-ashburn-1\" for OCI."]
    pub fn set_origin_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_region = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    type O = BlockAssignable<ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {}
impl BuildComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    pub fn build(self) -> ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
        ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
            access_key: core::default::Default::default(),
            access_key_id: core::default::Default::default(),
            access_key_version: core::default::Default::default(),
            origin_region: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
        ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key` after provisioning.\nThe access key used for s3 bucket authentication.\nRequired for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request."]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nThe identifier of an access key used for s3 bucket authentication."]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_key_version` after provisioning.\nThe optional version identifier for the access key. You can use this to keep track of different iterations of your access key."]
    pub fn access_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_region` after provisioning.\nThe name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.\nFor example, \"us-east-1\" for AWS or \"us-ashburn-1\" for OCI."]
    pub fn origin_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_region", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendServiceSecuritySettingsElDynamic {
    aws_v4_authentication:
        Option<DynamicBlock<ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceSecuritySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_v4_authentication:
        Option<Vec<ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>>,
    dynamic: ComputeBackendServiceSecuritySettingsElDynamic,
}
impl ComputeBackendServiceSecuritySettingsEl {
    #[doc = "Set the field `client_tls_policy`.\nClientTlsPolicy is a resource that specifies how a client should authenticate\nconnections to backends of a service. This resource itself does not affect\nconfiguration unless it is attached to a backend service resource."]
    pub fn set_client_tls_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_alt_names`.\nA list of alternate names to verify the subject identity in the certificate.\nIf specified, the client will verify that the server certificate's subject\nalt name matches one of the specified values."]
    pub fn set_subject_alt_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.subject_alt_names = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_v4_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_v4_authentication = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeBackendServiceSecuritySettingsEl {
    type O = BlockAssignable<ComputeBackendServiceSecuritySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceSecuritySettingsEl {}
impl BuildComputeBackendServiceSecuritySettingsEl {
    pub fn build(self) -> ComputeBackendServiceSecuritySettingsEl {
        ComputeBackendServiceSecuritySettingsEl {
            client_tls_policy: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
            aws_v4_authentication: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceSecuritySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceSecuritySettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceSecuritySettingsElRef {
        ComputeBackendServiceSecuritySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceSecuritySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_tls_policy` after provisioning.\nClientTlsPolicy is a resource that specifies how a client should authenticate\nconnections to backends of a service. This resource itself does not affect\nconfiguration unless it is attached to a backend service resource."]
    pub fn client_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_tls_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\nA list of alternate names to verify the subject identity in the certificate.\nIf specified, the client will verify that the server certificate's subject\nalt name matches one of the specified values."]
    pub fn subject_alt_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> ListRef<ComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_v4_authentication", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}
impl ComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    type O = BlockAssignable<ComputeBackendServiceStrongSessionAffinityCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}
impl BuildComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    pub fn build(self) -> ComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
        ComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
        ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
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
struct ComputeBackendServiceStrongSessionAffinityCookieElDynamic {
    ttl: Option<DynamicBlock<ComputeBackendServiceStrongSessionAffinityCookieElTtlEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceStrongSessionAffinityCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<ComputeBackendServiceStrongSessionAffinityCookieElTtlEl>>,
    dynamic: ComputeBackendServiceStrongSessionAffinityCookieElDynamic,
}
impl ComputeBackendServiceStrongSessionAffinityCookieEl {
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
        v: impl Into<BlockAssignable<ComputeBackendServiceStrongSessionAffinityCookieElTtlEl>>,
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
impl ToListMappable for ComputeBackendServiceStrongSessionAffinityCookieEl {
    type O = BlockAssignable<ComputeBackendServiceStrongSessionAffinityCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceStrongSessionAffinityCookieEl {}
impl BuildComputeBackendServiceStrongSessionAffinityCookieEl {
    pub fn build(self) -> ComputeBackendServiceStrongSessionAffinityCookieEl {
        ComputeBackendServiceStrongSessionAffinityCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceStrongSessionAffinityCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceStrongSessionAffinityCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceStrongSessionAffinityCookieElRef {
        ComputeBackendServiceStrongSessionAffinityCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceStrongSessionAffinityCookieElRef {
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
    pub fn ttl(&self) -> ListRef<ComputeBackendServiceStrongSessionAffinityCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeBackendServiceTimeoutsEl {
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
impl ToListMappable for ComputeBackendServiceTimeoutsEl {
    type O = BlockAssignable<ComputeBackendServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceTimeoutsEl {}
impl BuildComputeBackendServiceTimeoutsEl {
    pub fn build(self) -> ComputeBackendServiceTimeoutsEl {
        ComputeBackendServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceTimeoutsElRef {
        ComputeBackendServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceTimeoutsElRef {
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
pub struct ComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_resource_identifier: Option<PrimField<String>>,
}
impl ComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
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
impl ToListMappable for ComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    type O = BlockAssignable<ComputeBackendServiceTlsSettingsElSubjectAltNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceTlsSettingsElSubjectAltNamesEl {}
impl BuildComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    pub fn build(self) -> ComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
        ComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
        ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
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
struct ComputeBackendServiceTlsSettingsElDynamic {
    subject_alt_names: Option<DynamicBlock<ComputeBackendServiceTlsSettingsElSubjectAltNamesEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendServiceTlsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names: Option<Vec<ComputeBackendServiceTlsSettingsElSubjectAltNamesEl>>,
    dynamic: ComputeBackendServiceTlsSettingsElDynamic,
}
impl ComputeBackendServiceTlsSettingsEl {
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
        v: impl Into<BlockAssignable<ComputeBackendServiceTlsSettingsElSubjectAltNamesEl>>,
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
impl ToListMappable for ComputeBackendServiceTlsSettingsEl {
    type O = BlockAssignable<ComputeBackendServiceTlsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendServiceTlsSettingsEl {}
impl BuildComputeBackendServiceTlsSettingsEl {
    pub fn build(self) -> ComputeBackendServiceTlsSettingsEl {
        ComputeBackendServiceTlsSettingsEl {
            authentication_config: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendServiceTlsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendServiceTlsSettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceTlsSettingsElRef {
        ComputeBackendServiceTlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendServiceTlsSettingsElRef {
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
    ) -> ListRef<ComputeBackendServiceTlsSettingsElSubjectAltNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendServiceDynamic {
    backend: Option<DynamicBlock<ComputeBackendServiceBackendEl>>,
    cdn_policy: Option<DynamicBlock<ComputeBackendServiceCdnPolicyEl>>,
    circuit_breakers: Option<DynamicBlock<ComputeBackendServiceCircuitBreakersEl>>,
    consistent_hash: Option<DynamicBlock<ComputeBackendServiceConsistentHashEl>>,
    custom_metrics: Option<DynamicBlock<ComputeBackendServiceCustomMetricsEl>>,
    iap: Option<DynamicBlock<ComputeBackendServiceIapEl>>,
    locality_lb_policies: Option<DynamicBlock<ComputeBackendServiceLocalityLbPoliciesEl>>,
    log_config: Option<DynamicBlock<ComputeBackendServiceLogConfigEl>>,
    max_stream_duration: Option<DynamicBlock<ComputeBackendServiceMaxStreamDurationEl>>,
    outlier_detection: Option<DynamicBlock<ComputeBackendServiceOutlierDetectionEl>>,
    params: Option<DynamicBlock<ComputeBackendServiceParamsEl>>,
    security_settings: Option<DynamicBlock<ComputeBackendServiceSecuritySettingsEl>>,
    strong_session_affinity_cookie:
        Option<DynamicBlock<ComputeBackendServiceStrongSessionAffinityCookieEl>>,
    tls_settings: Option<DynamicBlock<ComputeBackendServiceTlsSettingsEl>>,
}
