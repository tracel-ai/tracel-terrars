use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeBackendServiceData {
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
struct DataComputeBackendService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeBackendServiceData>,
}
#[derive(Clone)]
pub struct DataComputeBackendService(Rc<DataComputeBackendService_>);
impl DataComputeBackendService {
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
    #[doc = "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.affinity_cookie_ttl_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this BackendService."]
    pub fn backend(&self) -> SetRef<DataComputeBackendServiceBackendElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED."]
    pub fn circuit_breakers(&self) -> ListRef<DataComputeBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing. This field only applies if the load_balancing_scheme is set to\nINTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is\nset to MAGLEV or RING_HASH."]
    pub fn consistent_hash(&self) -> ListRef<DataComputeBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\nList of custom metrics that are used for the WEIGHTED_ROUND_ROBIN locality_lb_policy."]
    pub fn custom_metrics(&self) -> ListRef<DataComputeBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, the Google-managed OAuth client is used."]
    pub fn iap(&self) -> ListRef<DataComputeBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\nA list of locality load balancing policies to be used in order of\npreference. Either the policy or the customPolicy field should be set.\nOverrides any value set in the localityLbPolicy field.\n\nlocalityLbPolicies is only supported when the BackendService is referenced\nby a URL Map that is referenced by a target gRPC proxy that has the\nvalidateForProxyless field set to true."]
    pub fn locality_lb_policies(
        &self,
    ) -> ListRef<DataComputeBackendServiceLocalityLbPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locality_lb_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locality_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> ListRef<DataComputeBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\nSpecifies the default maximum duration (timeout) for streams to this service. Duration is computed from the\nbeginning of the stream until the response has been completely processed, including all retries. A stream that\ndoes not complete in this duration is closed.\nIf not specified, there will be no timeout limit, i.e. the maximum duration is infinite.\nThis value can be overridden in the PathMatcher configuration of the UrlMap that references this backend service.\nThis field is only allowed when the loadBalancingScheme of the backend service is INTERNAL_SELF_MANAGED."]
    pub fn max_stream_duration(&self) -> ListRef<DataComputeBackendServiceMaxStreamDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_stream_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nApplicable backend service types can be a global backend service with the\nloadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED."]
    pub fn outlier_detection(&self) -> ListRef<DataComputeBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nThe security settings that apply to this backend service. This field is applicable to either\na regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C, and\nload_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the\nload_balancing_scheme set to INTERNAL_SELF_MANAGED."]
    pub fn security_settings(&self) -> ListRef<DataComputeBackendServiceSecuritySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<DataComputeBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\nConfiguration for Backend Authenticated TLS and mTLS. May only be specified when the backend protocol is SSL, HTTPS or HTTP2."]
    pub fn tls_settings(&self) -> ListRef<DataComputeBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeBackendService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeBackendService {}
impl ToListMappable for DataComputeBackendService {
    type O = ListRef<DataComputeBackendServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeBackendService_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_backend_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeBackendService {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeBackendService {
    pub fn build(self, stack: &mut Stack) -> DataComputeBackendService {
        let out = DataComputeBackendService(Rc::new(DataComputeBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeBackendServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeBackendServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeBackendServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.affinity_cookie_ttl_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this BackendService."]
    pub fn backend(&self) -> SetRef<DataComputeBackendServiceBackendElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED."]
    pub fn circuit_breakers(&self) -> ListRef<DataComputeBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing. This field only applies if the load_balancing_scheme is set to\nINTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is\nset to MAGLEV or RING_HASH."]
    pub fn consistent_hash(&self) -> ListRef<DataComputeBackendServiceConsistentHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consistent_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\nList of custom metrics that are used for the WEIGHTED_ROUND_ROBIN locality_lb_policy."]
    pub fn custom_metrics(&self) -> ListRef<DataComputeBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, the Google-managed OAuth client is used."]
    pub fn iap(&self) -> ListRef<DataComputeBackendServiceIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `locality_lb_policies` after provisioning.\nA list of locality load balancing policies to be used in order of\npreference. Either the policy or the customPolicy field should be set.\nOverrides any value set in the localityLbPolicy field.\n\nlocalityLbPolicies is only supported when the BackendService is referenced\nby a URL Map that is referenced by a target gRPC proxy that has the\nvalidateForProxyless field set to true."]
    pub fn locality_lb_policies(
        &self,
    ) -> ListRef<DataComputeBackendServiceLocalityLbPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locality_lb_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locality_lb_policy` after provisioning.\nThe load balancing algorithm used within the scope of the locality.\nThe possible values are:\n\n* 'ROUND_ROBIN': This is a simple policy in which each healthy backend\n                 is selected in round robin order.\n\n* 'LEAST_REQUEST': An O(1) algorithm which selects two random healthy\n                   hosts and picks the host which has fewer active requests.\n\n* 'RING_HASH': The ring/modulo hash load balancer implements consistent\n               hashing to backends. The algorithm has the property that the\n               addition/removal of a host from a set of N hosts only affects\n               1/N of the requests.\n\n* 'RANDOM': The load balancer selects a random healthy host.\n\n* 'ORIGINAL_DESTINATION': Backend host is selected based on the client\n                          connection metadata, i.e., connections are opened\n                          to the same address as the destination address of\n                          the incoming connection before the connection\n                          was redirected to the load balancer.\n\n* 'MAGLEV': used as a drop in replacement for the ring hash load balancer.\n            Maglev is not as stable as ring hash but has faster table lookup\n            build times and host selection times. For more information about\n            Maglev, refer to https://ai.google/research/pubs/pub44824\n\n* 'WEIGHTED_MAGLEV': Per-instance weighted Load Balancing via health check\n                     reported weights. Only applicable to loadBalancingScheme\n                     EXTERNAL. If set, the Backend Service must\n                     configure a non legacy HTTP-based Health Check, and\n                     health check replies are expected to contain\n                     non-standard HTTP response header field\n                     X-Load-Balancing-Endpoint-Weight to specify the\n                     per-instance weights. If set, Load Balancing is weight\n                     based on the per-instance weights reported in the last\n                     processed health check replies, as long as every\n                     instance either reported a valid weight or had\n                     UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains\n                     equal-weight.\n\n* 'WEIGHTED_ROUND_ROBIN': Per-endpoint weighted round-robin Load Balancing using weights computed\n                          from Backend reported Custom Metrics. If set, the Backend Service\n                          responses are expected to contain non-standard HTTP response header field\n                          X-Endpoint-Load-Metrics. The reported metrics\n                          to use for computing the weights are specified via the\n                          backends[].customMetrics fields.\n\nlocality_lb_policy is applicable to either:\n\n* A regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C,\n  and loadBalancingScheme set to INTERNAL_MANAGED.\n* A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.\n* A regional backend service with loadBalancingScheme set to EXTERNAL (External Network\n  Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External\n  Network Load Balancing. The default is MAGLEV.\n\nIf session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,\nor RING_HASH, session affinity settings will not take effect.\n\nOnly ROUND_ROBIN and RING_HASH are supported when the backend service is referenced\nby a URL map that is bound to target gRPC proxy that has validate_for_proxyless\nfield set to true. Possible values: [\"ROUND_ROBIN\", \"LEAST_REQUEST\", \"RING_HASH\", \"RANDOM\", \"ORIGINAL_DESTINATION\", \"MAGLEV\", \"WEIGHTED_MAGLEV\", \"WEIGHTED_ROUND_ROBIN\"]"]
    pub fn locality_lb_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locality_lb_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> ListRef<DataComputeBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\nSpecifies the default maximum duration (timeout) for streams to this service. Duration is computed from the\nbeginning of the stream until the response has been completely processed, including all retries. A stream that\ndoes not complete in this duration is closed.\nIf not specified, there will be no timeout limit, i.e. the maximum duration is infinite.\nThis value can be overridden in the PathMatcher configuration of the UrlMap that references this backend service.\nThis field is only allowed when the loadBalancingScheme of the backend service is INTERNAL_SELF_MANAGED."]
    pub fn max_stream_duration(&self) -> ListRef<DataComputeBackendServiceMaxStreamDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_stream_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nApplicable backend service types can be a global backend service with the\nloadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED."]
    pub fn outlier_detection(&self) -> ListRef<DataComputeBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nThe security settings that apply to this backend service. This field is applicable to either\na regional backend service with the service_protocol set to HTTP, HTTPS, HTTP2 or H2C, and\nload_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the\nload_balancing_scheme set to INTERNAL_SELF_MANAGED."]
    pub fn security_settings(&self) -> ListRef<DataComputeBackendServiceSecuritySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<DataComputeBackendServiceStrongSessionAffinityCookieElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strong_session_affinity_cookie", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nThe backend service timeout has a different meaning depending on the type of load balancer.\nFor more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).\nThe default is 30 seconds.\nThe full range of timeout values allowed goes from 1 through 2,147,483,647 seconds."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_settings` after provisioning.\nConfiguration for Backend Authenticated TLS and mTLS. May only be specified when the backend protocol is SSL, HTTPS or HTTP2."]
    pub fn tls_settings(&self) -> ListRef<DataComputeBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceBackendElCustomMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_utilization: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeBackendServiceBackendElCustomMetricsEl {
    #[doc = "Set the field `dry_run`.\n"]
    pub fn set_dry_run(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dry_run = Some(v.into());
        self
    }
    #[doc = "Set the field `max_utilization`.\n"]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceBackendElCustomMetricsEl {
    type O = BlockAssignable<DataComputeBackendServiceBackendElCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceBackendElCustomMetricsEl {}
impl BuildDataComputeBackendServiceBackendElCustomMetricsEl {
    pub fn build(self) -> DataComputeBackendServiceBackendElCustomMetricsEl {
        DataComputeBackendServiceBackendElCustomMetricsEl {
            dry_run: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceBackendElCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceBackendElCustomMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceBackendElCustomMetricsElRef {
        DataComputeBackendServiceBackendElCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceBackendElCustomMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\n"]
    pub fn dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }
    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\n"]
    pub fn max_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    balancing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_scaler: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<ListField<DataComputeBackendServiceBackendElCustomMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
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
}
impl DataComputeBackendServiceBackendEl {
    #[doc = "Set the field `balancing_mode`.\n"]
    pub fn set_balancing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.balancing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_scaler`.\n"]
    pub fn set_capacity_scaler(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capacity_scaler = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_metrics`.\n"]
    pub fn set_custom_metrics(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceBackendElCustomMetricsEl>>,
    ) -> Self {
        self.custom_metrics = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_endpoint`.\n"]
    pub fn set_max_connections_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_per_instance`.\n"]
    pub fn set_max_connections_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate`.\n"]
    pub fn set_max_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_endpoint`.\n"]
    pub fn set_max_rate_per_endpoint(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rate_per_instance`.\n"]
    pub fn set_max_rate_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rate_per_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `max_utilization`.\n"]
    pub fn set_max_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_utilization = Some(v.into());
        self
    }
    #[doc = "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preference = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceBackendEl {
    type O = BlockAssignable<DataComputeBackendServiceBackendEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceBackendEl {}
impl BuildDataComputeBackendServiceBackendEl {
    pub fn build(self) -> DataComputeBackendServiceBackendEl {
        DataComputeBackendServiceBackendEl {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            description: core::default::Default::default(),
            group: core::default::Default::default(),
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            preference: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceBackendElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceBackendElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceBackendElRef {
        DataComputeBackendServiceBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceBackendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `balancing_mode` after provisioning.\n"]
    pub fn balancing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.balancing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_scaler` after provisioning.\n"]
    pub fn capacity_scaler(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_scaler", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_metrics` after provisioning.\n"]
    pub fn custom_metrics(&self) -> ListRef<DataComputeBackendServiceBackendElCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_endpoint` after provisioning.\n"]
    pub fn max_connections_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_connections_per_instance` after provisioning.\n"]
    pub fn max_connections_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate` after provisioning.\n"]
    pub fn max_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `max_rate_per_endpoint` after provisioning.\n"]
    pub fn max_rate_per_endpoint(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rate_per_instance` after provisioning.\n"]
    pub fn max_rate_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rate_per_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_utilization` after provisioning.\n"]
    pub fn max_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
}
impl DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    type O = BlockAssignable<DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {}
impl BuildDataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
    pub fn build(self) -> DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
        DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl {
            header_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
        DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
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
impl DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_host`.\n"]
    pub fn set_include_host(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_host = Some(v.into());
        self
    }
    #[doc = "Set the field `include_http_headers`.\n"]
    pub fn set_include_http_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `include_named_cookies`.\n"]
    pub fn set_include_named_cookies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_named_cookies = Some(v.into());
        self
    }
    #[doc = "Set the field `include_protocol`.\n"]
    pub fn set_include_protocol(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `include_query_string`.\n"]
    pub fn set_include_query_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_query_string = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_blacklist`.\n"]
    pub fn set_query_string_blacklist(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.query_string_blacklist = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_whitelist`.\n"]
    pub fn set_query_string_whitelist(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.query_string_whitelist = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {}
impl BuildDataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
        DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl {
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
pub struct DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
        DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_host` after provisioning.\n"]
    pub fn include_host(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }
    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\n"]
    pub fn include_http_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_http_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_named_cookies` after provisioning.\n"]
    pub fn include_named_cookies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_named_cookies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_protocol` after provisioning.\n"]
    pub fn include_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_protocol", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_query_string` after provisioning.\n"]
    pub fn include_query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_query_string", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_blacklist` after provisioning.\n"]
    pub fn query_string_blacklist(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_string_blacklist", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\n"]
    pub fn query_string_whitelist(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_string_whitelist", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {}
impl BuildDataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
        DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
        DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCdnPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_cache_on_request_headers:
        Option<ListField<DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<ListField<DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
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
    negative_caching_policy:
        Option<ListField<DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_coalescing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceCdnPolicyEl {
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    ) -> Self {
        self.bypass_cache_on_request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
    ) -> Self {
        self.cache_key_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_mode`.\n"]
    pub fn set_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ttl`.\n"]
    pub fn set_client_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching`.\n"]
    pub fn set_negative_caching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negative_caching = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        self.negative_caching_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `request_coalescing`.\n"]
    pub fn set_request_coalescing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_coalescing = Some(v.into());
        self
    }
    #[doc = "Set the field `serve_while_stale`.\n"]
    pub fn set_serve_while_stale(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.serve_while_stale = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_url_cache_max_age_sec`.\n"]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCdnPolicyEl {
    type O = BlockAssignable<DataComputeBackendServiceCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCdnPolicyEl {}
impl BuildDataComputeBackendServiceCdnPolicyEl {
    pub fn build(self) -> DataComputeBackendServiceCdnPolicyEl {
        DataComputeBackendServiceCdnPolicyEl {
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceCdnPolicyElRef {
        DataComputeBackendServiceCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> ListRef<DataComputeBackendServiceCdnPolicyElBypassCacheOnRequestHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bypass_cache_on_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> ListRef<DataComputeBackendServiceCdnPolicyElCacheKeyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\n"]
    pub fn cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\n"]
    pub fn client_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\n"]
    pub fn negative_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negative_caching", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> ListRef<DataComputeBackendServiceCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_coalescing` after provisioning.\n"]
    pub fn request_coalescing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_coalescing", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\n"]
    pub fn serve_while_stale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serve_while_stale", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\n"]
    pub fn signed_url_cache_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url_cache_max_age_sec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCircuitBreakersEl {
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
impl DataComputeBackendServiceCircuitBreakersEl {
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pending_requests = Some(v.into());
        self
    }
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
    #[doc = "Set the field `max_requests_per_connection`.\n"]
    pub fn set_max_requests_per_connection(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests_per_connection = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCircuitBreakersEl {
    type O = BlockAssignable<DataComputeBackendServiceCircuitBreakersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCircuitBreakersEl {}
impl BuildDataComputeBackendServiceCircuitBreakersEl {
    pub fn build(self) -> DataComputeBackendServiceCircuitBreakersEl {
        DataComputeBackendServiceCircuitBreakersEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceCircuitBreakersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCircuitBreakersElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceCircuitBreakersElRef {
        DataComputeBackendServiceCircuitBreakersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCircuitBreakersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pending_requests", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
    #[doc = "Get a reference to the value of field `max_requests_per_connection` after provisioning.\n"]
    pub fn max_requests_per_connection(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_requests_per_connection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    type O = BlockAssignable<DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {}
impl BuildDataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
    pub fn build(self) -> DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
        DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
        DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceConsistentHashElHttpCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<ListField<DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl>>,
}
impl DataComputeBackendServiceConsistentHashElHttpCookieEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceConsistentHashElHttpCookieElTtlEl>>,
    ) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceConsistentHashElHttpCookieEl {
    type O = BlockAssignable<DataComputeBackendServiceConsistentHashElHttpCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceConsistentHashElHttpCookieEl {}
impl BuildDataComputeBackendServiceConsistentHashElHttpCookieEl {
    pub fn build(self) -> DataComputeBackendServiceConsistentHashElHttpCookieEl {
        DataComputeBackendServiceConsistentHashElHttpCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceConsistentHashElHttpCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceConsistentHashElHttpCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceConsistentHashElHttpCookieElRef {
        DataComputeBackendServiceConsistentHashElHttpCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceConsistentHashElHttpCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<DataComputeBackendServiceConsistentHashElHttpCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceConsistentHashEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_cookie: Option<ListField<DataComputeBackendServiceConsistentHashElHttpCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_ring_size: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceConsistentHashEl {
    #[doc = "Set the field `http_cookie`.\n"]
    pub fn set_http_cookie(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceConsistentHashElHttpCookieEl>>,
    ) -> Self {
        self.http_cookie = Some(v.into());
        self
    }
    #[doc = "Set the field `http_header_name`.\n"]
    pub fn set_http_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_ring_size`.\n"]
    pub fn set_minimum_ring_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_ring_size = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceConsistentHashEl {
    type O = BlockAssignable<DataComputeBackendServiceConsistentHashEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceConsistentHashEl {}
impl BuildDataComputeBackendServiceConsistentHashEl {
    pub fn build(self) -> DataComputeBackendServiceConsistentHashEl {
        DataComputeBackendServiceConsistentHashEl {
            http_cookie: core::default::Default::default(),
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceConsistentHashElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceConsistentHashElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceConsistentHashElRef {
        DataComputeBackendServiceConsistentHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceConsistentHashElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_cookie` after provisioning.\n"]
    pub fn http_cookie(&self) -> ListRef<DataComputeBackendServiceConsistentHashElHttpCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_cookie", self.base))
    }
    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_header_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_ring_size` after provisioning.\n"]
    pub fn minimum_ring_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_ring_size", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceCustomMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeBackendServiceCustomMetricsEl {
    #[doc = "Set the field `dry_run`.\n"]
    pub fn set_dry_run(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dry_run = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceCustomMetricsEl {
    type O = BlockAssignable<DataComputeBackendServiceCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceCustomMetricsEl {}
impl BuildDataComputeBackendServiceCustomMetricsEl {
    pub fn build(self) -> DataComputeBackendServiceCustomMetricsEl {
        DataComputeBackendServiceCustomMetricsEl {
            dry_run: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceCustomMetricsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceCustomMetricsElRef {
        DataComputeBackendServiceCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceCustomMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dry_run` after provisioning.\n"]
    pub fn dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dry_run", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceIapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret_sha256: Option<PrimField<String>>,
}
impl DataComputeBackendServiceIapEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_client_id`.\n"]
    pub fn set_oauth2_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_client_secret`.\n"]
    pub fn set_oauth2_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_client_secret_sha256`.\n"]
    pub fn set_oauth2_client_secret_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_client_secret_sha256 = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceIapEl {
    type O = BlockAssignable<DataComputeBackendServiceIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceIapEl {}
impl BuildDataComputeBackendServiceIapEl {
    pub fn build(self) -> DataComputeBackendServiceIapEl {
        DataComputeBackendServiceIapEl {
            enabled: core::default::Default::default(),
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
            oauth2_client_secret_sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceIapElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceIapElRef {
        DataComputeBackendServiceIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceIapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\n"]
    pub fn oauth2_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret` after provisioning.\n"]
    pub fn oauth2_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\n"]
    pub fn oauth2_client_secret_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret_sha256", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    type O = BlockAssignable<DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {}
impl BuildDataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
    pub fn build(self) -> DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
        DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl {
            data: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
        DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    type O = BlockAssignable<DataComputeBackendServiceLocalityLbPoliciesElPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceLocalityLbPoliciesElPolicyEl {}
impl BuildDataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
    pub fn build(self) -> DataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
        DataComputeBackendServiceLocalityLbPoliciesElPolicyEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
        DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceLocalityLbPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_policy: Option<ListField<DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<ListField<DataComputeBackendServiceLocalityLbPoliciesElPolicyEl>>,
}
impl DataComputeBackendServiceLocalityLbPoliciesEl {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyEl>>,
    ) -> Self {
        self.custom_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceLocalityLbPoliciesElPolicyEl>>,
    ) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceLocalityLbPoliciesEl {
    type O = BlockAssignable<DataComputeBackendServiceLocalityLbPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceLocalityLbPoliciesEl {}
impl BuildDataComputeBackendServiceLocalityLbPoliciesEl {
    pub fn build(self) -> DataComputeBackendServiceLocalityLbPoliciesEl {
        DataComputeBackendServiceLocalityLbPoliciesEl {
            custom_policy: core::default::Default::default(),
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceLocalityLbPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceLocalityLbPoliciesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceLocalityLbPoliciesElRef {
        DataComputeBackendServiceLocalityLbPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceLocalityLbPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(
        &self,
    ) -> ListRef<DataComputeBackendServiceLocalityLbPoliciesElCustomPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> ListRef<DataComputeBackendServiceLocalityLbPoliciesElPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceLogConfigEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_fields`.\n"]
    pub fn set_optional_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.optional_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_mode`.\n"]
    pub fn set_optional_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optional_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceLogConfigEl {
    type O = BlockAssignable<DataComputeBackendServiceLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceLogConfigEl {}
impl BuildDataComputeBackendServiceLogConfigEl {
    pub fn build(self) -> DataComputeBackendServiceLogConfigEl {
        DataComputeBackendServiceLogConfigEl {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceLogConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceLogConfigElRef {
        DataComputeBackendServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.optional_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `optional_mode` after provisioning.\n"]
    pub fn optional_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optional_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceMaxStreamDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl DataComputeBackendServiceMaxStreamDurationEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceMaxStreamDurationEl {
    type O = BlockAssignable<DataComputeBackendServiceMaxStreamDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceMaxStreamDurationEl {}
impl BuildDataComputeBackendServiceMaxStreamDurationEl {
    pub fn build(self) -> DataComputeBackendServiceMaxStreamDurationEl {
        DataComputeBackendServiceMaxStreamDurationEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceMaxStreamDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceMaxStreamDurationElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceMaxStreamDurationElRef {
        DataComputeBackendServiceMaxStreamDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceMaxStreamDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    type O = BlockAssignable<DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {}
impl BuildDataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    pub fn build(self) -> DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
        DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
        DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceOutlierDetectionElIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceOutlierDetectionElIntervalEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceOutlierDetectionElIntervalEl {
    type O = BlockAssignable<DataComputeBackendServiceOutlierDetectionElIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceOutlierDetectionElIntervalEl {}
impl BuildDataComputeBackendServiceOutlierDetectionElIntervalEl {
    pub fn build(self) -> DataComputeBackendServiceOutlierDetectionElIntervalEl {
        DataComputeBackendServiceOutlierDetectionElIntervalEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceOutlierDetectionElIntervalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceOutlierDetectionElIntervalElRef {
        DataComputeBackendServiceOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceOutlierDetectionElIntervalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceOutlierDetectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ejection_time:
        Option<ListField<DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
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
    interval: Option<ListField<DataComputeBackendServiceOutlierDetectionElIntervalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ejection_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_minimum_hosts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_request_volume: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_stdev_factor: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceOutlierDetectionEl {
    #[doc = "Set the field `base_ejection_time`.\n"]
    pub fn set_base_ejection_time(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
    ) -> Self {
        self.base_ejection_time = Some(v.into());
        self
    }
    #[doc = "Set the field `consecutive_errors`.\n"]
    pub fn set_consecutive_errors(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.consecutive_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `consecutive_gateway_failure`.\n"]
    pub fn set_consecutive_gateway_failure(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.consecutive_gateway_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_consecutive_errors`.\n"]
    pub fn set_enforcing_consecutive_errors(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.enforcing_consecutive_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_consecutive_gateway_failure`.\n"]
    pub fn set_enforcing_consecutive_gateway_failure(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.enforcing_consecutive_gateway_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcing_success_rate`.\n"]
    pub fn set_enforcing_success_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.enforcing_success_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceOutlierDetectionElIntervalEl>>,
    ) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ejection_percent`.\n"]
    pub fn set_max_ejection_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ejection_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_minimum_hosts`.\n"]
    pub fn set_success_rate_minimum_hosts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_minimum_hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_request_volume`.\n"]
    pub fn set_success_rate_request_volume(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_request_volume = Some(v.into());
        self
    }
    #[doc = "Set the field `success_rate_stdev_factor`.\n"]
    pub fn set_success_rate_stdev_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_rate_stdev_factor = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceOutlierDetectionEl {
    type O = BlockAssignable<DataComputeBackendServiceOutlierDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceOutlierDetectionEl {}
impl BuildDataComputeBackendServiceOutlierDetectionEl {
    pub fn build(self) -> DataComputeBackendServiceOutlierDetectionEl {
        DataComputeBackendServiceOutlierDetectionEl {
            base_ejection_time: core::default::Default::default(),
            consecutive_errors: core::default::Default::default(),
            consecutive_gateway_failure: core::default::Default::default(),
            enforcing_consecutive_errors: core::default::Default::default(),
            enforcing_consecutive_gateway_failure: core::default::Default::default(),
            enforcing_success_rate: core::default::Default::default(),
            interval: core::default::Default::default(),
            max_ejection_percent: core::default::Default::default(),
            success_rate_minimum_hosts: core::default::Default::default(),
            success_rate_request_volume: core::default::Default::default(),
            success_rate_stdev_factor: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceOutlierDetectionElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceOutlierDetectionElRef {
        DataComputeBackendServiceOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceOutlierDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_ejection_time` after provisioning.\n"]
    pub fn base_ejection_time(
        &self,
    ) -> ListRef<DataComputeBackendServiceOutlierDetectionElBaseEjectionTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_ejection_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consecutive_errors` after provisioning.\n"]
    pub fn consecutive_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consecutive_errors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consecutive_gateway_failure` after provisioning.\n"]
    pub fn consecutive_gateway_failure(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consecutive_gateway_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_consecutive_errors` after provisioning.\n"]
    pub fn enforcing_consecutive_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_consecutive_errors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_consecutive_gateway_failure` after provisioning.\n"]
    pub fn enforcing_consecutive_gateway_failure(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_consecutive_gateway_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcing_success_rate` after provisioning.\n"]
    pub fn enforcing_success_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcing_success_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> ListRef<DataComputeBackendServiceOutlierDetectionElIntervalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\n"]
    pub fn max_ejection_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ejection_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_minimum_hosts` after provisioning.\n"]
    pub fn success_rate_minimum_hosts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_minimum_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_request_volume` after provisioning.\n"]
    pub fn success_rate_request_volume(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_request_volume", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_rate_stdev_factor` after provisioning.\n"]
    pub fn success_rate_stdev_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_rate_stdev_factor", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeBackendServiceParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceParamsEl {
    type O = BlockAssignable<DataComputeBackendServiceParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceParamsEl {}
impl BuildDataComputeBackendServiceParamsEl {
    pub fn build(self) -> DataComputeBackendServiceParamsEl {
        DataComputeBackendServiceParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceParamsElRef {
        DataComputeBackendServiceParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceParamsElRef {
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
pub struct DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_region: Option<PrimField<String>>,
}
impl DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    #[doc = "Set the field `access_key`.\n"]
    pub fn set_access_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key = Some(v.into());
        self
    }
    #[doc = "Set the field `access_key_id`.\n"]
    pub fn set_access_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `access_key_version`.\n"]
    pub fn set_access_key_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key_version = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_region`.\n"]
    pub fn set_origin_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_region = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    type O = BlockAssignable<DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {}
impl BuildDataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
    pub fn build(self) -> DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
        DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl {
            access_key: core::default::Default::default(),
            access_key_id: core::default::Default::default(),
            access_key_version: core::default::Default::default(),
            origin_region: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
        DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_key_version` after provisioning.\n"]
    pub fn access_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_region` after provisioning.\n"]
    pub fn origin_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_region", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceSecuritySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_v4_authentication:
        Option<ListField<DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names: Option<ListField<PrimField<String>>>,
}
impl DataComputeBackendServiceSecuritySettingsEl {
    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationEl>>,
    ) -> Self {
        self.aws_v4_authentication = Some(v.into());
        self
    }
    #[doc = "Set the field `client_tls_policy`.\n"]
    pub fn set_client_tls_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.subject_alt_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceSecuritySettingsEl {
    type O = BlockAssignable<DataComputeBackendServiceSecuritySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceSecuritySettingsEl {}
impl BuildDataComputeBackendServiceSecuritySettingsEl {
    pub fn build(self) -> DataComputeBackendServiceSecuritySettingsEl {
        DataComputeBackendServiceSecuritySettingsEl {
            aws_v4_authentication: core::default::Default::default(),
            client_tls_policy: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceSecuritySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceSecuritySettingsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceSecuritySettingsElRef {
        DataComputeBackendServiceSecuritySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceSecuritySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> ListRef<DataComputeBackendServiceSecuritySettingsElAwsV4AuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_v4_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_tls_policy` after provisioning.\n"]
    pub fn client_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_tls_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    type O = BlockAssignable<DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {}
impl BuildDataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
    pub fn build(self) -> DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
        DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
        DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceStrongSessionAffinityCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<ListField<DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl>>,
}
impl DataComputeBackendServiceStrongSessionAffinityCookieEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceStrongSessionAffinityCookieElTtlEl>>,
    ) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceStrongSessionAffinityCookieEl {
    type O = BlockAssignable<DataComputeBackendServiceStrongSessionAffinityCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceStrongSessionAffinityCookieEl {}
impl BuildDataComputeBackendServiceStrongSessionAffinityCookieEl {
    pub fn build(self) -> DataComputeBackendServiceStrongSessionAffinityCookieEl {
        DataComputeBackendServiceStrongSessionAffinityCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceStrongSessionAffinityCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceStrongSessionAffinityCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceStrongSessionAffinityCookieElRef {
        DataComputeBackendServiceStrongSessionAffinityCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceStrongSessionAffinityCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<DataComputeBackendServiceStrongSessionAffinityCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_resource_identifier: Option<PrimField<String>>,
}
impl DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `uniform_resource_identifier`.\n"]
    pub fn set_uniform_resource_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uniform_resource_identifier = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    type O = BlockAssignable<DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {}
impl BuildDataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
    pub fn build(self) -> DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
        DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
        DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `uniform_resource_identifier` after provisioning.\n"]
    pub fn uniform_resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uniform_resource_identifier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendServiceTlsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names: Option<ListField<DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl>>,
}
impl DataComputeBackendServiceTlsSettingsEl {
    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sni`.\n"]
    pub fn set_sni(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sni = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_alt_names`.\n"]
    pub fn set_subject_alt_names(
        mut self,
        v: impl Into<ListField<DataComputeBackendServiceTlsSettingsElSubjectAltNamesEl>>,
    ) -> Self {
        self.subject_alt_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendServiceTlsSettingsEl {
    type O = BlockAssignable<DataComputeBackendServiceTlsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendServiceTlsSettingsEl {}
impl BuildDataComputeBackendServiceTlsSettingsEl {
    pub fn build(self) -> DataComputeBackendServiceTlsSettingsEl {
        DataComputeBackendServiceTlsSettingsEl {
            authentication_config: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendServiceTlsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendServiceTlsSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendServiceTlsSettingsElRef {
        DataComputeBackendServiceTlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendServiceTlsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sni", self.base))
    }
    #[doc = "Get a reference to the value of field `subject_alt_names` after provisioning.\n"]
    pub fn subject_alt_names(
        &self,
    ) -> ListRef<DataComputeBackendServiceTlsSettingsElSubjectAltNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
}
