use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionBackendServiceData {
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
struct DataComputeRegionBackendService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionBackendServiceData>,
}
#[derive(Clone)]
pub struct DataComputeRegionBackendService(Rc<DataComputeRegionBackendService_>);
impl DataComputeRegionBackendService {
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
    #[doc = "Set the field `region`.\nThe Region in which the created backend service should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `affinity_cookie_ttl_sec` after provisioning.\nLifetime of cookies in seconds if session_affinity is\nGENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts\nonly until the end of the browser session (or equivalent). The\nmaximum allowed value for TTL is one day.\n\nWhen the load balancing scheme is INTERNAL, this field is not used."]
    pub fn affinity_cookie_ttl_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.affinity_cookie_ttl_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this RegionBackendService."]
    pub fn backend(&self) -> SetRef<DataComputeRegionBackendServiceBackendElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeRegionBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the 'load_balancing_scheme' is set to INTERNAL_MANAGED\nand the 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C."]
    pub fn circuit_breakers(&self) -> ListRef<DataComputeRegionBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_tracking_policy` after provisioning.\nConnection Tracking configuration for this BackendService.\nThis is available only for Layer 4 Internal Load Balancing and\nNetwork Load Balancing."]
    pub fn connection_tracking_policy(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceConnectionTrackingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing.\nThis field only applies when all of the following are true -\n  * 'load_balancing_scheme' is set to INTERNAL_MANAGED\n  * 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C\n  * 'locality_lb_policy' is set to MAGLEV or RING_HASH"]
    pub fn consistent_hash(&self) -> ListRef<DataComputeRegionBackendServiceConsistentHashElRef> {
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
    pub fn custom_metrics(&self) -> ListRef<DataComputeRegionBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `failover_policy` after provisioning.\nPolicy for failovers."]
    pub fn failover_policy(&self) -> ListRef<DataComputeRegionBackendServiceFailoverPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ha_policy` after provisioning.\nConfigures self-managed High Availability (HA) for External and Internal Protocol Forwarding.\nThe backends of this regional backend service must only specify zonal network endpoint groups\n(NEGs) of type GCE_VM_IP. Note that haPolicy is not for load balancing, and therefore cannot\nbe specified with sessionAffinity, connectionTrackingPolicy, and failoverPolicy. haPolicy\nrequires customers to be responsible for tracking backend endpoint health and electing a\nleader among the healthy endpoints. Therefore, haPolicy cannot be specified with healthChecks.\nhaPolicy can only be specified for External Passthrough Network Load Balancers and Internal\nPassthrough Network Load Balancers."]
    pub fn ha_policy(&self) -> ListRef<DataComputeRegionBackendServiceHaPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ha_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to HealthCheck resources for health checking\nthis RegionBackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend."]
    pub fn health_checks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.health_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, Google-managed OAuth client is used."]
    pub fn iap(&self) -> ListRef<DataComputeRegionBackendServiceIapElRef> {
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
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> ListRef<DataComputeRegionBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network_pass_through_lb_traffic_policy` after provisioning.\nConfigures traffic steering properties of internal passthrough Network Load Balancers."]
    pub fn network_pass_through_lb_traffic_policy(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.network_pass_through_lb_traffic_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nThis field is applicable only when the 'load_balancing_scheme' is set\nto INTERNAL_MANAGED and the 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C."]
    pub fn outlier_detection(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeRegionBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef> {
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
    pub fn tls_settings(&self) -> ListRef<DataComputeRegionBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRegionBackendService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionBackendService {}
impl ToListMappable for DataComputeRegionBackendService {
    type O = ListRef<DataComputeRegionBackendServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionBackendService_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_backend_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionBackendService {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeRegionBackendService {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionBackendService {
        let out = DataComputeRegionBackendService(Rc::new(DataComputeRegionBackendService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRegionBackendServiceData {
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
pub struct DataComputeRegionBackendServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionBackendServiceRef {
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
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe set of backends that serve this RegionBackendService."]
    pub fn backend(&self) -> SetRef<DataComputeRegionBackendServiceBackendElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this BackendService."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeRegionBackendServiceCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_breakers` after provisioning.\nSettings controlling the volume of connections to a backend service. This field\nis applicable only when the 'load_balancing_scheme' is set to INTERNAL_MANAGED\nand the 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C."]
    pub fn circuit_breakers(&self) -> ListRef<DataComputeRegionBackendServiceCircuitBreakersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_breakers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout_sec` after provisioning.\nTime for which instance will be drained (not accept new\nconnections, but still work to finish started)."]
    pub fn connection_draining_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_tracking_policy` after provisioning.\nConnection Tracking configuration for this BackendService.\nThis is available only for Layer 4 Internal Load Balancing and\nNetwork Load Balancing."]
    pub fn connection_tracking_policy(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceConnectionTrackingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consistent_hash` after provisioning.\nConsistent Hash-based load balancing can be used to provide soft session\naffinity based on HTTP headers, cookies or other properties. This load balancing\npolicy is applicable only for HTTP connections. The affinity to a particular\ndestination host will be lost when one or more hosts are added/removed from the\ndestination service. This field specifies parameters that control consistent\nhashing.\nThis field only applies when all of the following are true -\n  * 'load_balancing_scheme' is set to INTERNAL_MANAGED\n  * 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C\n  * 'locality_lb_policy' is set to MAGLEV or RING_HASH"]
    pub fn consistent_hash(&self) -> ListRef<DataComputeRegionBackendServiceConsistentHashElRef> {
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
    pub fn custom_metrics(&self) -> ListRef<DataComputeRegionBackendServiceCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `failover_policy` after provisioning.\nPolicy for failovers."]
    pub fn failover_policy(&self) -> ListRef<DataComputeRegionBackendServiceFailoverPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ha_policy` after provisioning.\nConfigures self-managed High Availability (HA) for External and Internal Protocol Forwarding.\nThe backends of this regional backend service must only specify zonal network endpoint groups\n(NEGs) of type GCE_VM_IP. Note that haPolicy is not for load balancing, and therefore cannot\nbe specified with sessionAffinity, connectionTrackingPolicy, and failoverPolicy. haPolicy\nrequires customers to be responsible for tracking backend endpoint health and electing a\nleader among the healthy endpoints. Therefore, haPolicy cannot be specified with healthChecks.\nhaPolicy can only be specified for External Passthrough Network Load Balancers and Internal\nPassthrough Network Load Balancers."]
    pub fn ha_policy(&self) -> ListRef<DataComputeRegionBackendServiceHaPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ha_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\nThe set of URLs to HealthCheck resources for health checking\nthis RegionBackendService. Currently at most one health\ncheck can be specified.\n\nA health check must be specified unless the backend service uses an internet\nor serverless NEG as a backend."]
    pub fn health_checks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.health_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\nSettings for enabling Cloud Identity Aware Proxy.\nIf OAuth client is not set, Google-managed OAuth client is used."]
    pub fn iap(&self) -> ListRef<DataComputeRegionBackendServiceIapElRef> {
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
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nThis field denotes the logging options for the load balancer traffic served by this backend service.\nIf logging is enabled, logs will be exported to Stackdriver."]
    pub fn log_config(&self) -> ListRef<DataComputeRegionBackendServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network_pass_through_lb_traffic_policy` after provisioning.\nConfigures traffic steering properties of internal passthrough Network Load Balancers."]
    pub fn network_pass_through_lb_traffic_policy(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.network_pass_through_lb_traffic_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\nSettings controlling eviction of unhealthy hosts from the load balancing pool.\nThis field is applicable only when the 'load_balancing_scheme' is set\nto INTERNAL_MANAGED and the 'protocol' is set to HTTP, HTTPS, HTTP2 or H2C."]
    pub fn outlier_detection(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceOutlierDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outlier_detection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeRegionBackendServiceParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `strong_session_affinity_cookie` after provisioning.\nDescribes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY."]
    pub fn strong_session_affinity_cookie(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef> {
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
    pub fn tls_settings(&self) -> ListRef<DataComputeRegionBackendServiceTlsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceBackendElCustomMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_utilization: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceBackendElCustomMetricsEl {
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
impl ToListMappable for DataComputeRegionBackendServiceBackendElCustomMetricsEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceBackendElCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceBackendElCustomMetricsEl {}
impl BuildDataComputeRegionBackendServiceBackendElCustomMetricsEl {
    pub fn build(self) -> DataComputeRegionBackendServiceBackendElCustomMetricsEl {
        DataComputeRegionBackendServiceBackendElCustomMetricsEl {
            dry_run: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceBackendElCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceBackendElCustomMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceBackendElCustomMetricsElRef {
        DataComputeRegionBackendServiceBackendElCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceBackendElCustomMetricsElRef {
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
pub struct DataComputeRegionBackendServiceBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    balancing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_scaler: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metrics: Option<ListField<DataComputeRegionBackendServiceBackendElCustomMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover: Option<PrimField<bool>>,
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
}
impl DataComputeRegionBackendServiceBackendEl {
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
        v: impl Into<ListField<DataComputeRegionBackendServiceBackendElCustomMetricsEl>>,
    ) -> Self {
        self.custom_metrics = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `failover`.\n"]
    pub fn set_failover(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover = Some(v.into());
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
}
impl ToListMappable for DataComputeRegionBackendServiceBackendEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceBackendEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceBackendEl {}
impl BuildDataComputeRegionBackendServiceBackendEl {
    pub fn build(self) -> DataComputeRegionBackendServiceBackendEl {
        DataComputeRegionBackendServiceBackendEl {
            balancing_mode: core::default::Default::default(),
            capacity_scaler: core::default::Default::default(),
            custom_metrics: core::default::Default::default(),
            description: core::default::Default::default(),
            failover: core::default::Default::default(),
            group: core::default::Default::default(),
            max_connections: core::default::Default::default(),
            max_connections_per_endpoint: core::default::Default::default(),
            max_connections_per_instance: core::default::Default::default(),
            max_rate: core::default::Default::default(),
            max_rate_per_endpoint: core::default::Default::default(),
            max_rate_per_instance: core::default::Default::default(),
            max_utilization: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceBackendElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceBackendElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceBackendElRef {
        DataComputeRegionBackendServiceBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceBackendElRef {
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
    pub fn custom_metrics(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceBackendElCustomMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_metrics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `failover` after provisioning.\n"]
    pub fn failover(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
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
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
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
impl DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_host`.\n"]
    pub fn set_include_host(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_host = Some(v.into());
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
impl ToListMappable for DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {}
impl BuildDataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
        DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl {
            include_host: core::default::Default::default(),
            include_named_cookies: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            include_query_string: core::default::Default::default(),
            query_string_blacklist: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
        DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_host` after provisioning.\n"]
    pub fn include_host(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
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
pub struct DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {}
impl BuildDataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
        DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
        DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceCdnPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<ListField<DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
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
        Option<ListField<DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceCdnPolicyEl {
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<ListField<DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyEl>>,
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
        v: impl Into<ListField<DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        self.negative_caching_policy = Some(v.into());
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
impl ToListMappable for DataComputeRegionBackendServiceCdnPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceCdnPolicyEl {}
impl BuildDataComputeRegionBackendServiceCdnPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceCdnPolicyEl {
        DataComputeRegionBackendServiceCdnPolicyEl {
            cache_key_policy: core::default::Default::default(),
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceCdnPolicyElRef {
        DataComputeRegionBackendServiceCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceCdnPolicyElCacheKeyPolicyElRef> {
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
    ) -> ListRef<DataComputeRegionBackendServiceCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
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
pub struct DataComputeRegionBackendServiceCircuitBreakersEl {
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
impl DataComputeRegionBackendServiceCircuitBreakersEl {
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
impl ToListMappable for DataComputeRegionBackendServiceCircuitBreakersEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceCircuitBreakersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceCircuitBreakersEl {}
impl BuildDataComputeRegionBackendServiceCircuitBreakersEl {
    pub fn build(self) -> DataComputeRegionBackendServiceCircuitBreakersEl {
        DataComputeRegionBackendServiceCircuitBreakersEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
            max_requests: core::default::Default::default(),
            max_requests_per_connection: core::default::Default::default(),
            max_retries: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceCircuitBreakersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceCircuitBreakersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceCircuitBreakersElRef {
        DataComputeRegionBackendServiceCircuitBreakersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceCircuitBreakersElRef {
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
pub struct DataComputeRegionBackendServiceConnectionTrackingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_persistence_on_unhealthy_backends: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_strong_affinity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_mode: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceConnectionTrackingPolicyEl {
    #[doc = "Set the field `connection_persistence_on_unhealthy_backends`.\n"]
    pub fn set_connection_persistence_on_unhealthy_backends(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.connection_persistence_on_unhealthy_backends = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_strong_affinity`.\n"]
    pub fn set_enable_strong_affinity(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_strong_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_timeout_sec`.\n"]
    pub fn set_idle_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `tracking_mode`.\n"]
    pub fn set_tracking_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tracking_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceConnectionTrackingPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceConnectionTrackingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceConnectionTrackingPolicyEl {}
impl BuildDataComputeRegionBackendServiceConnectionTrackingPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceConnectionTrackingPolicyEl {
        DataComputeRegionBackendServiceConnectionTrackingPolicyEl {
            connection_persistence_on_unhealthy_backends: core::default::Default::default(),
            enable_strong_affinity: core::default::Default::default(),
            idle_timeout_sec: core::default::Default::default(),
            tracking_mode: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceConnectionTrackingPolicyElRef {
        DataComputeRegionBackendServiceConnectionTrackingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceConnectionTrackingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_persistence_on_unhealthy_backends` after provisioning.\n"]
    pub fn connection_persistence_on_unhealthy_backends(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_persistence_on_unhealthy_backends", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_strong_affinity` after provisioning.\n"]
    pub fn enable_strong_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_strong_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_timeout_sec` after provisioning.\n"]
    pub fn idle_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_mode` after provisioning.\n"]
    pub fn tracking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tracking_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
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
impl ToListMappable for DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {}
impl BuildDataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
    pub fn build(self) -> DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
        DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
        DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef {
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
pub struct DataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<ListField<DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>>,
}
impl DataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
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
        v: impl Into<ListField<DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlEl>>,
    ) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceConsistentHashElHttpCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceConsistentHashElHttpCookieEl {}
impl BuildDataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
    pub fn build(self) -> DataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
        DataComputeRegionBackendServiceConsistentHashElHttpCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
        DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef {
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
    pub fn ttl(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceConsistentHashElHttpCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceConsistentHashEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_cookie: Option<ListField<DataComputeRegionBackendServiceConsistentHashElHttpCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_ring_size: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceConsistentHashEl {
    #[doc = "Set the field `http_cookie`.\n"]
    pub fn set_http_cookie(
        mut self,
        v: impl Into<ListField<DataComputeRegionBackendServiceConsistentHashElHttpCookieEl>>,
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
impl ToListMappable for DataComputeRegionBackendServiceConsistentHashEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceConsistentHashEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceConsistentHashEl {}
impl BuildDataComputeRegionBackendServiceConsistentHashEl {
    pub fn build(self) -> DataComputeRegionBackendServiceConsistentHashEl {
        DataComputeRegionBackendServiceConsistentHashEl {
            http_cookie: core::default::Default::default(),
            http_header_name: core::default::Default::default(),
            minimum_ring_size: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceConsistentHashElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceConsistentHashElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceConsistentHashElRef {
        DataComputeRegionBackendServiceConsistentHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceConsistentHashElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_cookie` after provisioning.\n"]
    pub fn http_cookie(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceConsistentHashElHttpCookieElRef> {
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
pub struct DataComputeRegionBackendServiceCustomMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceCustomMetricsEl {
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
impl ToListMappable for DataComputeRegionBackendServiceCustomMetricsEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceCustomMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceCustomMetricsEl {}
impl BuildDataComputeRegionBackendServiceCustomMetricsEl {
    pub fn build(self) -> DataComputeRegionBackendServiceCustomMetricsEl {
        DataComputeRegionBackendServiceCustomMetricsEl {
            dry_run: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceCustomMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceCustomMetricsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceCustomMetricsElRef {
        DataComputeRegionBackendServiceCustomMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceCustomMetricsElRef {
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
pub struct DataComputeRegionBackendServiceFailoverPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_connection_drain_on_failover: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_traffic_if_unhealthy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_ratio: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceFailoverPolicyEl {
    #[doc = "Set the field `disable_connection_drain_on_failover`.\n"]
    pub fn set_disable_connection_drain_on_failover(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_connection_drain_on_failover = Some(v.into());
        self
    }
    #[doc = "Set the field `drop_traffic_if_unhealthy`.\n"]
    pub fn set_drop_traffic_if_unhealthy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.drop_traffic_if_unhealthy = Some(v.into());
        self
    }
    #[doc = "Set the field `failover_ratio`.\n"]
    pub fn set_failover_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failover_ratio = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceFailoverPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceFailoverPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceFailoverPolicyEl {}
impl BuildDataComputeRegionBackendServiceFailoverPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceFailoverPolicyEl {
        DataComputeRegionBackendServiceFailoverPolicyEl {
            disable_connection_drain_on_failover: core::default::Default::default(),
            drop_traffic_if_unhealthy: core::default::Default::default(),
            failover_ratio: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceFailoverPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceFailoverPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceFailoverPolicyElRef {
        DataComputeRegionBackendServiceFailoverPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceFailoverPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_connection_drain_on_failover` after provisioning.\n"]
    pub fn disable_connection_drain_on_failover(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_connection_drain_on_failover", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `drop_traffic_if_unhealthy` after provisioning.\n"]
    pub fn drop_traffic_if_unhealthy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.drop_traffic_if_unhealthy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failover_ratio` after provisioning.\n"]
    pub fn failover_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_ratio", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {}
impl BuildDataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
    pub fn build(self) -> DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
        DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl {
            instance: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
        DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceHaPolicyElLeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_endpoint:
        Option<ListField<DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>>,
}
impl DataComputeRegionBackendServiceHaPolicyElLeaderEl {
    #[doc = "Set the field `backend_group`.\n"]
    pub fn set_backend_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_group = Some(v.into());
        self
    }
    #[doc = "Set the field `network_endpoint`.\n"]
    pub fn set_network_endpoint(
        mut self,
        v: impl Into<ListField<DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointEl>>,
    ) -> Self {
        self.network_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceHaPolicyElLeaderEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceHaPolicyElLeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceHaPolicyElLeaderEl {}
impl BuildDataComputeRegionBackendServiceHaPolicyElLeaderEl {
    pub fn build(self) -> DataComputeRegionBackendServiceHaPolicyElLeaderEl {
        DataComputeRegionBackendServiceHaPolicyElLeaderEl {
            backend_group: core::default::Default::default(),
            network_endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceHaPolicyElLeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceHaPolicyElLeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceHaPolicyElLeaderElRef {
        DataComputeRegionBackendServiceHaPolicyElLeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceHaPolicyElLeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_group` after provisioning.\n"]
    pub fn backend_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_endpoint` after provisioning.\n"]
    pub fn network_endpoint(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceHaPolicyElLeaderElNetworkEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceHaPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_ip_move: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leader: Option<ListField<DataComputeRegionBackendServiceHaPolicyElLeaderEl>>,
}
impl DataComputeRegionBackendServiceHaPolicyEl {
    #[doc = "Set the field `fast_ip_move`.\n"]
    pub fn set_fast_ip_move(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fast_ip_move = Some(v.into());
        self
    }
    #[doc = "Set the field `leader`.\n"]
    pub fn set_leader(
        mut self,
        v: impl Into<ListField<DataComputeRegionBackendServiceHaPolicyElLeaderEl>>,
    ) -> Self {
        self.leader = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceHaPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceHaPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceHaPolicyEl {}
impl BuildDataComputeRegionBackendServiceHaPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceHaPolicyEl {
        DataComputeRegionBackendServiceHaPolicyEl {
            fast_ip_move: core::default::Default::default(),
            leader: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceHaPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceHaPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceHaPolicyElRef {
        DataComputeRegionBackendServiceHaPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceHaPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fast_ip_move` after provisioning.\n"]
    pub fn fast_ip_move(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fast_ip_move", self.base))
    }
    #[doc = "Get a reference to the value of field `leader` after provisioning.\n"]
    pub fn leader(&self) -> ListRef<DataComputeRegionBackendServiceHaPolicyElLeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.leader", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceIapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_secret_sha256: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceIapEl {
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
impl ToListMappable for DataComputeRegionBackendServiceIapEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceIapEl {}
impl BuildDataComputeRegionBackendServiceIapEl {
    pub fn build(self) -> DataComputeRegionBackendServiceIapEl {
        DataComputeRegionBackendServiceIapEl {
            enabled: core::default::Default::default(),
            oauth2_client_id: core::default::Default::default(),
            oauth2_client_secret: core::default::Default::default(),
            oauth2_client_secret_sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceIapElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceIapElRef {
        DataComputeRegionBackendServiceIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceIapElRef {
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
pub struct DataComputeRegionBackendServiceLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceLogConfigEl {
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
impl ToListMappable for DataComputeRegionBackendServiceLogConfigEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceLogConfigEl {}
impl BuildDataComputeRegionBackendServiceLogConfigEl {
    pub fn build(self) -> DataComputeRegionBackendServiceLogConfigEl {
        DataComputeRegionBackendServiceLogConfigEl {
            enable: core::default::Default::default(),
            optional_fields: core::default::Default::default(),
            optional_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceLogConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceLogConfigElRef {
        DataComputeRegionBackendServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceLogConfigElRef {
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
pub struct DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    spillover: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spillover_ratio: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    #[doc = "Set the field `spillover`.\n"]
    pub fn set_spillover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spillover = Some(v.into());
        self
    }
    #[doc = "Set the field `spillover_ratio`.\n"]
    pub fn set_spillover_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.spillover_ratio = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl
{
    type O = BlockAssignable<
        DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
}
impl BuildDataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
    pub fn build(
        self,
    ) -> DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
        DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl {
            spillover: core::default::Default::default(),
            spillover_ratio: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
        DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `spillover` after provisioning.\n"]
    pub fn spillover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spillover", self.base))
    }
    #[doc = "Get a reference to the value of field `spillover_ratio` after provisioning.\n"]
    pub fn spillover_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spillover_ratio", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    zonal_affinity: Option<
        ListField<
            DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl,
        >,
    >,
}
impl DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    #[doc = "Set the field `zonal_affinity`.\n"]
    pub fn set_zonal_affinity(
        mut self,
        v: impl Into<
            ListField<
                DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityEl,
            >,
        >,
    ) -> Self {
        self.zonal_affinity = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {}
impl BuildDataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
    pub fn build(self) -> DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
        DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyEl {
            zonal_affinity: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
        DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `zonal_affinity` after provisioning.\n"]
    pub fn zonal_affinity(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceNetworkPassThroughLbTrafficPolicyElZonalAffinityElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zonal_affinity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
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
impl ToListMappable for DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {}
impl BuildDataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
    pub fn build(self) -> DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
        DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
        DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef {
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
pub struct DataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
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
impl ToListMappable for DataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceOutlierDetectionElIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceOutlierDetectionElIntervalEl {}
impl BuildDataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
    pub fn build(self) -> DataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
        DataComputeRegionBackendServiceOutlierDetectionElIntervalEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
        DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef {
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
pub struct DataComputeRegionBackendServiceOutlierDetectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ejection_time:
        Option<ListField<DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
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
    interval: Option<ListField<DataComputeRegionBackendServiceOutlierDetectionElIntervalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ejection_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_minimum_hosts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_request_volume: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_rate_stdev_factor: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceOutlierDetectionEl {
    #[doc = "Set the field `base_ejection_time`.\n"]
    pub fn set_base_ejection_time(
        mut self,
        v: impl Into<ListField<DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeEl>>,
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
        v: impl Into<ListField<DataComputeRegionBackendServiceOutlierDetectionElIntervalEl>>,
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
impl ToListMappable for DataComputeRegionBackendServiceOutlierDetectionEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceOutlierDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceOutlierDetectionEl {}
impl BuildDataComputeRegionBackendServiceOutlierDetectionEl {
    pub fn build(self) -> DataComputeRegionBackendServiceOutlierDetectionEl {
        DataComputeRegionBackendServiceOutlierDetectionEl {
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
pub struct DataComputeRegionBackendServiceOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceOutlierDetectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceOutlierDetectionElRef {
        DataComputeRegionBackendServiceOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceOutlierDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_ejection_time` after provisioning.\n"]
    pub fn base_ejection_time(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceOutlierDetectionElBaseEjectionTimeElRef> {
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
    pub fn interval(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceOutlierDetectionElIntervalElRef> {
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
pub struct DataComputeRegionBackendServiceParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeRegionBackendServiceParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceParamsEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceParamsEl {}
impl BuildDataComputeRegionBackendServiceParamsEl {
    pub fn build(self) -> DataComputeRegionBackendServiceParamsEl {
        DataComputeRegionBackendServiceParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceParamsElRef {
        DataComputeRegionBackendServiceParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceParamsElRef {
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
pub struct DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
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
impl ToListMappable for DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {}
impl BuildDataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
    pub fn build(self) -> DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
        DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
        DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef {
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
pub struct DataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<ListField<DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>>,
}
impl DataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
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
        v: impl Into<ListField<DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlEl>>,
    ) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceStrongSessionAffinityCookieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceStrongSessionAffinityCookieEl {}
impl BuildDataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
    pub fn build(self) -> DataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
        DataComputeRegionBackendServiceStrongSessionAffinityCookieEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
        DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceStrongSessionAffinityCookieElRef {
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
    pub fn ttl(
        &self,
    ) -> ListRef<DataComputeRegionBackendServiceStrongSessionAffinityCookieElTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_resource_identifier: Option<PrimField<String>>,
}
impl DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
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
impl ToListMappable for DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {}
impl BuildDataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
    pub fn build(self) -> DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
        DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl {
            dns_name: core::default::Default::default(),
            uniform_resource_identifier: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
        DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef {
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
pub struct DataComputeRegionBackendServiceTlsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alt_names:
        Option<ListField<DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>>,
}
impl DataComputeRegionBackendServiceTlsSettingsEl {
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
        v: impl Into<ListField<DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesEl>>,
    ) -> Self {
        self.subject_alt_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionBackendServiceTlsSettingsEl {
    type O = BlockAssignable<DataComputeRegionBackendServiceTlsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionBackendServiceTlsSettingsEl {}
impl BuildDataComputeRegionBackendServiceTlsSettingsEl {
    pub fn build(self) -> DataComputeRegionBackendServiceTlsSettingsEl {
        DataComputeRegionBackendServiceTlsSettingsEl {
            authentication_config: core::default::Default::default(),
            sni: core::default::Default::default(),
            subject_alt_names: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionBackendServiceTlsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionBackendServiceTlsSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionBackendServiceTlsSettingsElRef {
        DataComputeRegionBackendServiceTlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionBackendServiceTlsSettingsElRef {
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
    ) -> ListRef<DataComputeRegionBackendServiceTlsSettingsElSubjectAltNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alt_names", self.base),
        )
    }
}
