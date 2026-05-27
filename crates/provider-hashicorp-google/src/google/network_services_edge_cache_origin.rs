use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesEdgeCacheOriginData {
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
    failover_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<PrimField<f64>>,
    name: PrimField<String>,
    origin_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_v4_authentication: Option<Vec<NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flex_shielding: Option<Vec<NetworkServicesEdgeCacheOriginFlexShieldingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_override_action: Option<Vec<NetworkServicesEdgeCacheOriginOriginOverrideActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_redirect: Option<Vec<NetworkServicesEdgeCacheOriginOriginRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<NetworkServicesEdgeCacheOriginTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesEdgeCacheOriginTimeoutsEl>,
    dynamic: NetworkServicesEdgeCacheOriginDynamic,
}
struct NetworkServicesEdgeCacheOrigin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesEdgeCacheOriginData>,
}
#[derive(Clone)]
pub struct NetworkServicesEdgeCacheOrigin(Rc<NetworkServicesEdgeCacheOrigin_>);
impl NetworkServicesEdgeCacheOrigin {
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
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `failover_origin`.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn set_failover_origin(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().failover_origin = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `max_attempts`.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn set_max_attempts(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn set_retry_conditions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().retry_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws_v4_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws_v4_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `flex_shielding`.\n"]
    pub fn set_flex_shielding(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginFlexShieldingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flex_shielding = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flex_shielding = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `origin_override_action`.\n"]
    pub fn set_origin_override_action(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_override_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_override_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `origin_redirect`.\n"]
    pub fn set_origin_redirect(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginOriginRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_redirect = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesEdgeCacheOriginTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
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
    #[doc = "Get a reference to the value of field `failover_origin` after provisioning.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn failover_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_attempts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_address` after provisioning.\nA fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub fn origin_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_v4_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flex_shielding` after provisioning.\n"]
    pub fn flex_shielding(&self) -> ListRef<NetworkServicesEdgeCacheOriginFlexShieldingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flex_shielding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_override_action` after provisioning.\n"]
    pub fn origin_override_action(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginOriginOverrideActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin_override_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_redirect` after provisioning.\n"]
    pub fn origin_redirect(&self) -> ListRef<NetworkServicesEdgeCacheOriginOriginRedirectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<NetworkServicesEdgeCacheOriginTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheOriginTimeoutsElRef {
        NetworkServicesEdgeCacheOriginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesEdgeCacheOrigin {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesEdgeCacheOrigin {}
impl ToListMappable for NetworkServicesEdgeCacheOrigin {
    type O = ListRef<NetworkServicesEdgeCacheOriginRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesEdgeCacheOrigin_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_edge_cache_origin".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesEdgeCacheOrigin {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: PrimField<String>,
    #[doc = "A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub origin_address: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheOrigin {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesEdgeCacheOrigin {
        let out = NetworkServicesEdgeCacheOrigin(Rc::new(NetworkServicesEdgeCacheOrigin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesEdgeCacheOriginData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                failover_origin: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_attempts: core::default::Default::default(),
                name: self.name,
                origin_address: self.origin_address,
                port: core::default::Default::default(),
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                retry_conditions: core::default::Default::default(),
                aws_v4_authentication: core::default::Default::default(),
                flex_shielding: core::default::Default::default(),
                origin_override_action: core::default::Default::default(),
                origin_redirect: core::default::Default::default(),
                timeout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesEdgeCacheOriginRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesEdgeCacheOriginRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
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
    #[doc = "Get a reference to the value of field `failover_origin` after provisioning.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn failover_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failover_origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_attempts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_address` after provisioning.\nA fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub fn origin_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_v4_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flex_shielding` after provisioning.\n"]
    pub fn flex_shielding(&self) -> ListRef<NetworkServicesEdgeCacheOriginFlexShieldingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flex_shielding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_override_action` after provisioning.\n"]
    pub fn origin_override_action(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginOriginOverrideActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin_override_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_redirect` after provisioning.\n"]
    pub fn origin_redirect(&self) -> ListRef<NetworkServicesEdgeCacheOriginOriginRedirectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<NetworkServicesEdgeCacheOriginTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheOriginTimeoutsElRef {
        NetworkServicesEdgeCacheOriginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
    access_key_id: PrimField<String>,
    origin_region: PrimField<String>,
    secret_access_key_version: PrimField<String>,
}
impl NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {}
impl ToListMappable for NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
    #[doc = "The access key ID your origin uses to identify the key."]
    pub access_key_id: PrimField<String>,
    #[doc = "The name of the AWS region that your origin is in."]
    pub origin_region: PrimField<String>,
    #[doc = "The Secret Manager secret version of the secret access key used by your origin.\n\nThis is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require."]
    pub secret_access_key_version: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
        NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl {
            access_key_id: self.access_key_id,
            origin_region: self.origin_region,
            secret_access_key_version: self.secret_access_key_version,
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef {
        NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginAwsV4AuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nThe access key ID your origin uses to identify the key."]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_region` after provisioning.\nThe name of the AWS region that your origin is in."]
    pub fn origin_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_region", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_access_key_version` after provisioning.\nThe Secret Manager secret version of the secret access key used by your origin.\n\nThis is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require."]
    pub fn secret_access_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginFlexShieldingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flex_shielding_regions: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesEdgeCacheOriginFlexShieldingEl {
    #[doc = "Set the field `flex_shielding_regions`.\nWhenever possible, content will be fetched from origin and cached in or\nnear the specified origin. Best effort.\n\nYou must specify exactly one FlexShieldingRegion. Possible values: [\"AFRICA_SOUTH1\", \"ME_CENTRAL1\"]"]
    pub fn set_flex_shielding_regions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.flex_shielding_regions = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginFlexShieldingEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginFlexShieldingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginFlexShieldingEl {}
impl BuildNetworkServicesEdgeCacheOriginFlexShieldingEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginFlexShieldingEl {
        NetworkServicesEdgeCacheOriginFlexShieldingEl {
            flex_shielding_regions: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginFlexShieldingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginFlexShieldingElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheOriginFlexShieldingElRef {
        NetworkServicesEdgeCacheOriginFlexShieldingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginFlexShieldingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `flex_shielding_regions` after provisioning.\nWhenever possible, content will be fetched from origin and cached in or\nnear the specified origin. Best effort.\n\nYou must specify exactly one FlexShieldingRegion. Possible values: [\"AFRICA_SOUTH1\", \"ME_CENTRAL1\"]"]
    pub fn flex_shielding_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flex_shielding_regions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl {
    #[doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name.\n\nBy default, added header values are appended\nto the response or request headers with the\nsame field names. The added values are\nseparated by commas.\n\nTo overwrite existing values, set 'replace' to 'true'."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl
{
    #[doc = "The name of the header to add."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl
    {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef
    {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name.\n\nBy default, added header values are appended\nto the response or request headers with the\nsame field names. The added values are\nseparated by commas.\n\nTo overwrite existing values, set 'replace' to 'true'."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<
            NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<
            NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    dynamic: NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElDynamic,
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {}
impl BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl {
            request_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<
        NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRequestHeadersToAddElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected\norigin, the request's host header is replaced with\ncontents of the hostRewrite.\n\nThis value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {}
impl BuildNetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected\norigin, the request's host header is replaced with\ncontents of the hostRewrite.\n\nThis value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheOriginOriginOverrideActionElDynamic {
    header_action:
        Option<DynamicBlock<NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl>>,
    url_rewrite:
        Option<DynamicBlock<NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl>>,
    dynamic: NetworkServicesEdgeCacheOriginOriginOverrideActionElDynamic,
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionEl {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginOriginOverrideActionEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginOriginOverrideActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginOriginOverrideActionEl {}
impl BuildNetworkServicesEdgeCacheOriginOriginOverrideActionEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginOriginOverrideActionEl {
        NetworkServicesEdgeCacheOriginOriginOverrideActionEl {
            header_action: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginOriginOverrideActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginOriginOverrideActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheOriginOriginOverrideActionElRef {
        NetworkServicesEdgeCacheOriginOriginOverrideActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginOriginOverrideActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginOriginOverrideActionElHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheOriginOriginOverrideActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginOriginRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_conditions: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesEdgeCacheOriginOriginRedirectEl {
    #[doc = "Set the field `redirect_conditions`.\nThe set of redirect response codes that the CDN\nfollows. Values of\n[RedirectConditions](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#redirectconditions)\nare accepted."]
    pub fn set_redirect_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.redirect_conditions = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginOriginRedirectEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginOriginRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginOriginRedirectEl {}
impl BuildNetworkServicesEdgeCacheOriginOriginRedirectEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginOriginRedirectEl {
        NetworkServicesEdgeCacheOriginOriginRedirectEl {
            redirect_conditions: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginOriginRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginOriginRedirectElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheOriginOriginRedirectElRef {
        NetworkServicesEdgeCacheOriginOriginRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginOriginRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `redirect_conditions` after provisioning.\nThe set of redirect response codes that the CDN\nfollows. Values of\n[RedirectConditions](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#redirectconditions)\nare accepted."]
    pub fn redirect_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_conditions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_timeout: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheOriginTimeoutEl {
    #[doc = "Set the field `connect_timeout`.\nThe maximum duration to wait for a single origin connection to be established, including DNS lookup, TLS handshake and TCP/QUIC connection establishment.\n\nDefaults to 5 seconds. The timeout must be a value between 1s and 15s.\n\nThe connectTimeout capped by the deadline set by the request's maxAttemptsTimeout.  The last connection attempt may have a smaller connectTimeout in order to adhere to the overall maxAttemptsTimeout."]
    pub fn set_connect_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connect_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `max_attempts_timeout`.\nThe maximum time across all connection attempts to the origin, including failover origins, before returning an error to the client. A HTTP 504 will be returned if the timeout is reached before a response is returned.\n\nDefaults to 15 seconds. The timeout must be a value between 1s and 30s.\n\nIf a failoverOrigin is specified, the maxAttemptsTimeout of the first configured origin sets the deadline for all connection attempts across all failoverOrigins."]
    pub fn set_max_attempts_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_attempts_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `read_timeout`.\nThe maximum duration to wait between reads of a single HTTP connection/stream.\n\nDefaults to 15 seconds.  The timeout must be a value between 1s and 30s.\n\nThe readTimeout is capped by the responseTimeout.  All reads of the HTTP connection/stream must be completed by the deadline set by the responseTimeout.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn set_read_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `response_timeout`.\nThe maximum duration to wait for the last byte of a response to arrive when reading from the HTTP connection/stream.\n\nDefaults to 30 seconds. The timeout must be a value between 1s and 120s.\n\nThe responseTimeout starts after the connection has been established.\n\nThis also applies to HTTP Chunked Transfer Encoding responses, and/or when an open-ended Range request is made to the origin. Origins that take longer to write additional bytes to the response than the configured responseTimeout will result in an error being returned to the client.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn set_response_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheOriginTimeoutEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginTimeoutEl {}
impl BuildNetworkServicesEdgeCacheOriginTimeoutEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginTimeoutEl {
        NetworkServicesEdgeCacheOriginTimeoutEl {
            connect_timeout: core::default::Default::default(),
            max_attempts_timeout: core::default::Default::default(),
            read_timeout: core::default::Default::default(),
            response_timeout: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginTimeoutElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheOriginTimeoutElRef {
        NetworkServicesEdgeCacheOriginTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connect_timeout` after provisioning.\nThe maximum duration to wait for a single origin connection to be established, including DNS lookup, TLS handshake and TCP/QUIC connection establishment.\n\nDefaults to 5 seconds. The timeout must be a value between 1s and 15s.\n\nThe connectTimeout capped by the deadline set by the request's maxAttemptsTimeout.  The last connection attempt may have a smaller connectTimeout in order to adhere to the overall maxAttemptsTimeout."]
    pub fn connect_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_attempts_timeout` after provisioning.\nThe maximum time across all connection attempts to the origin, including failover origins, before returning an error to the client. A HTTP 504 will be returned if the timeout is reached before a response is returned.\n\nDefaults to 15 seconds. The timeout must be a value between 1s and 30s.\n\nIf a failoverOrigin is specified, the maxAttemptsTimeout of the first configured origin sets the deadline for all connection attempts across all failoverOrigins."]
    pub fn max_attempts_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_attempts_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_timeout` after provisioning.\nThe maximum duration to wait between reads of a single HTTP connection/stream.\n\nDefaults to 15 seconds.  The timeout must be a value between 1s and 30s.\n\nThe readTimeout is capped by the responseTimeout.  All reads of the HTTP connection/stream must be completed by the deadline set by the responseTimeout.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn read_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `response_timeout` after provisioning.\nThe maximum duration to wait for the last byte of a response to arrive when reading from the HTTP connection/stream.\n\nDefaults to 30 seconds. The timeout must be a value between 1s and 120s.\n\nThe responseTimeout starts after the connection has been established.\n\nThis also applies to HTTP Chunked Transfer Encoding responses, and/or when an open-ended Range request is made to the origin. Origins that take longer to write additional bytes to the response than the configured responseTimeout will result in an error being returned to the client.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn response_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_timeout", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOriginTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheOriginTimeoutsEl {
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
impl ToListMappable for NetworkServicesEdgeCacheOriginTimeoutsEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheOriginTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheOriginTimeoutsEl {}
impl BuildNetworkServicesEdgeCacheOriginTimeoutsEl {
    pub fn build(self) -> NetworkServicesEdgeCacheOriginTimeoutsEl {
        NetworkServicesEdgeCacheOriginTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheOriginTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheOriginTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheOriginTimeoutsElRef {
        NetworkServicesEdgeCacheOriginTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheOriginTimeoutsElRef {
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
struct NetworkServicesEdgeCacheOriginDynamic {
    aws_v4_authentication:
        Option<DynamicBlock<NetworkServicesEdgeCacheOriginAwsV4AuthenticationEl>>,
    flex_shielding: Option<DynamicBlock<NetworkServicesEdgeCacheOriginFlexShieldingEl>>,
    origin_override_action:
        Option<DynamicBlock<NetworkServicesEdgeCacheOriginOriginOverrideActionEl>>,
    origin_redirect: Option<DynamicBlock<NetworkServicesEdgeCacheOriginOriginRedirectEl>>,
    timeout: Option<DynamicBlock<NetworkServicesEdgeCacheOriginTimeoutEl>>,
}
