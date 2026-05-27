use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesEdgeCacheServiceData {
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
    disable_http2: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_quic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_ssl_certificates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_tls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<NetworkServicesEdgeCacheServiceLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing: Option<Vec<NetworkServicesEdgeCacheServiceRoutingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesEdgeCacheServiceTimeoutsEl>,
    dynamic: NetworkServicesEdgeCacheServiceDynamic,
}
struct NetworkServicesEdgeCacheService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesEdgeCacheServiceData>,
}
#[derive(Clone)]
pub struct NetworkServicesEdgeCacheService(Rc<NetworkServicesEdgeCacheService_>);
impl NetworkServicesEdgeCacheService {
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
    #[doc = "Set the field `disable_http2`.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn set_disable_http2(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_http2 = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_quic`.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn set_disable_quic(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_quic = Some(v.into());
        self
    }
    #[doc = "Set the field `edge_security_policy`.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn set_edge_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `edge_ssl_certificates`.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn set_edge_ssl_certificates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().edge_ssl_certificates = Some(v.into());
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `require_tls`.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn set_require_tls(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_tls = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_policy`.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheServiceLogConfigEl>>,
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
    #[doc = "Set the field `routing`.\n"]
    pub fn set_routing(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheServiceRoutingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesEdgeCacheServiceTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `disable_http2` after provisioning.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn disable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_http2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_quic` after provisioning.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn disable_quic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_quic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_ssl_certificates` after provisioning.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn edge_ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edge_ssl_certificates", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\nThe IPv4 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv4_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\nThe IPv6 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
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
    #[doc = "Get a reference to the value of field `require_tls` after provisioning.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn require_tls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_tls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<NetworkServicesEdgeCacheServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesEdgeCacheService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesEdgeCacheService {}
impl ToListMappable for NetworkServicesEdgeCacheService {
    type O = ListRef<NetworkServicesEdgeCacheServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesEdgeCacheService_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_edge_cache_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesEdgeCacheService {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheService {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesEdgeCacheService {
        let out = NetworkServicesEdgeCacheService(Rc::new(NetworkServicesEdgeCacheService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesEdgeCacheServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_http2: core::default::Default::default(),
                disable_quic: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                edge_ssl_certificates: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                require_tls: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                log_config: core::default::Default::default(),
                routing: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesEdgeCacheServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesEdgeCacheServiceRef {
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
    #[doc = "Get a reference to the value of field `disable_http2` after provisioning.\nDisables HTTP/2.\n\nHTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use and reduces connection setup overhead by sending multiple streams over the same connection.\n\nSome legacy HTTP clients may have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from being advertised and negotiated."]
    pub fn disable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_http2", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_quic` after provisioning.\nHTTP/3 (IETF QUIC) and Google QUIC are enabled by default."]
    pub fn disable_quic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_quic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nResource URL that points at the Cloud Armor edge security policy that is applied on each request against the EdgeCacheService."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_ssl_certificates` after provisioning.\nURLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService.\n\nNote that only \"global\" certificates with a \"scope\" of \"EDGE_CACHE\" can be attached to an EdgeCacheService."]
    pub fn edge_ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edge_ssl_certificates", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\nThe IPv4 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv4_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\nThe IPv6 addresses associated with this service. Addresses are static for the lifetime of the service."]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
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
    #[doc = "Get a reference to the value of field `require_tls` after provisioning.\nRequire TLS (HTTPS) for all clients connecting to this service.\n\nClients who connect over HTTP (port 80) will receive a HTTP 301 to the same URL over HTTPS (port 443).\nYou must have at least one (1) edgeSslCertificate specified to enable this."]
    pub fn require_tls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_tls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nURL of the SslPolicy resource that will be associated with the EdgeCacheService.\n\nIf not set, the EdgeCacheService has no SSL policy configured, and will default to the \"COMPATIBLE\" policy."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<NetworkServicesEdgeCacheServiceLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing` after provisioning.\n"]
    pub fn routing(&self) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl NetworkServicesEdgeCacheServiceLogConfigEl {
    #[doc = "Set the field `enable`.\nSpecifies whether to enable logging for traffic served by this service."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\nConfigures the sampling rate of requests, where 1.0 means all logged requests are reported and 0.0 means no logged requests are reported. The default value is 1.0, and the value of the field must be in [0, 1].\n\nThis field can only be specified if logging is enabled for this service."]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceLogConfigEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceLogConfigEl {}
impl BuildNetworkServicesEdgeCacheServiceLogConfigEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceLogConfigEl {
        NetworkServicesEdgeCacheServiceLogConfigEl {
            enable: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceLogConfigElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheServiceLogConfigElRef {
        NetworkServicesEdgeCacheServiceLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nSpecifies whether to enable logging for traffic served by this service."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\nConfigures the sampling rate of requests, where 1.0 means all logged requests are reported and 0.0 means no logged requests are reported. The default value is 1.0, and the value of the field must be in [0, 1].\n\nThis field can only be specified if logging is enabled for this service."]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    hosts: ListField<PrimField<String>>,
    path_matcher: PrimField<String>,
}
impl NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[doc = "Set the field `description`.\nA human-readable description of the hostRule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    #[doc = "The list of host patterns to match.\n\nHost patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.\n\nWhen multiple hosts are specified, hosts are matched in the following priority:\n\n  1. Exact domain names: ''www.foo.com''.\n  2. Suffix domain wildcards: ''*.foo.com'' or ''*-bar.foo.com''.\n  3. Prefix domain wildcards: ''foo.*'' or ''foo-*''.\n  4. Special wildcard ''*'' matching any domain.\n\n  Notes:\n\n    The wildcard will not match the empty string. e.g. ''*-bar.foo.com'' will match ''baz-bar.foo.com'' but not ''-bar.foo.com''. The longest wildcards match first. Only a single host in the entire service can match on ''*''. A domain must be unique across all configured hosts within a service.\n\n    Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the \":authority\" header, from the incoming request.\n\n    You may specify up to 10 hosts."]
    pub hosts: ListField<PrimField<String>>,
    #[doc = "The name of the pathMatcher associated with this hostRule."]
    pub path_matcher: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElHostRuleEl {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the hostRule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match.\n\nHost patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.\n\nWhen multiple hosts are specified, hosts are matched in the following priority:\n\n  1. Exact domain names: ''www.foo.com''.\n  2. Suffix domain wildcards: ''*.foo.com'' or ''*-bar.foo.com''.\n  3. Prefix domain wildcards: ''foo.*'' or ''foo-*''.\n  4. Special wildcard ''*'' matching any domain.\n\n  Notes:\n\n    The wildcard will not match the empty string. e.g. ''*-bar.foo.com'' will match ''baz-bar.foo.com'' but not ''-bar.foo.com''. The longest wildcards match first. Only a single host in the entire service can match on ''*''. A domain must be unique across all configured hosts within a service.\n\n    Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the \":authority\" header, from the incoming request.\n\n    You may specify up to 10 hosts."]
    pub fn hosts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the pathMatcher associated with this hostRule."]
    pub fn path_matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl { # [doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl
{
    #[doc = "The name of the header to add."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl { header_name : self . header_name , header_value : self . header_value , replace : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl
{
    header_name: PrimField<String>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl
{
    #[doc = "The name of the header to remove."]
    pub header_name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl { header_name : self . header_name , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to remove."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl { # [doc = "Set the field `replace`.\nWhether to replace all existing headers with the same name."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl
{
    #[doc = "The name of the header to add."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl { header_name : self . header_name , header_value : self . header_value , replace : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl
{
    header_name: PrimField<String>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl
{
    #[doc = "Headers to remove from the response prior to sending it back to the client.\n\nResponse headers are only sent to the client, and do not have an effect on the cache serving the response."]
    pub header_name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl { header_name : self . header_name , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nHeaders to remove from the response prior to sending it back to the client.\n\nResponse headers are only sent to the client, and do not have an effect on the cache serving the response."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElDynamic { request_header_to_add : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl >> , request_header_to_remove : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl >> , response_header_to_add : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl >> , response_header_to_remove : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl >> , }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl { # [serde (skip_serializing_if = "Option::is_none")] request_header_to_add : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_header_to_remove : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_header_to_add : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_header_to_remove : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl > > , dynamic : NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElDynamic , }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    #[doc = "Set the field `request_header_to_add`.\n"]
    pub fn set_request_header_to_add(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_header_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_header_to_remove`.\n"]
    pub fn set_request_header_to_remove(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_header_to_remove = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_to_remove = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_header_to_add`.\n"]
    pub fn set_response_header_to_add(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_header_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_header_to_remove`.\n"]
    pub fn set_response_header_to_remove(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_header_to_remove = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_to_remove = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl {
            request_header_to_add: core::default::Default::default(),
            request_header_to_remove: core::default::Default::default(),
            response_header_to_add: core::default::Default::default(),
            response_header_to_remove: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_header_to_add` after provisioning.\n"]    pub fn request_header_to_add (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header_to_remove` after provisioning.\n"]    pub fn request_header_to_remove (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRequestHeaderToRemoveElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_header_to_add` after provisioning.\n"]    pub fn response_header_to_add (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_header_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_header_to_remove` after provisioning.\n"]    pub fn response_header_to_remove (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElResponseHeaderToRemoveElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_header_to_remove", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    header_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix_match: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    #[doc = "Set the field `exact_match`.\nThe value of the header should exactly match contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_match`.\nIf set to false (default), the headerMatch is considered a match if the match criteria above are met.\nIf set to true, the headerMatch is considered a match if the match criteria above are NOT met."]
    pub fn set_invert_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_match = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nA header with the contents of headerName must exist. The match takes place whether or not the request's header has a value."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn set_suffix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix_match = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl
{
    #[doc = "The header name to match on."]
    pub header_name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef { shared : shared , base : base . to_string () , }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the header should exactly match contents of exactMatch."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe header name to match on."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_match` after provisioning.\nIf set to false (default), the headerMatch is considered a match if the match criteria above are met.\nIf set to true, the headerMatch is considered a match if the match criteria above are NOT met."]
    pub fn invert_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `present_match` after provisioning.\nA header with the contents of headerName must exist. The match takes place whether or not the request's header has a value."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.present_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn suffix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
}
impl
    NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl
{
    #[doc = "Set the field `exact_match`.\nThe queryParameterMatch matches if the value of the parameter exactly matches the contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nSpecifies that the queryParameterMatch matches if the request contains the query parameter, irrespective of whether the parameter has a value or not."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl
{
    #[doc = "The name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl { exact_match : core :: default :: Default :: default () , name : self . name , present_match : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter exactly matches the contents of exactMatch."] pub fn exact_match (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.exact_match" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the queryParameterMatch matches if the request contains the query parameter, irrespective of whether the parameter has a value or not."] pub fn present_match (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.present_match" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElDynamic { header_match : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl >> , query_parameter_match : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl >> , }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl { # [serde (skip_serializing_if = "Option::is_none")] full_path_match : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ignore_case : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] path_template_match : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] prefix_match : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] header_match : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl > > , # [serde (skip_serializing_if = "Option::is_none")] query_parameter_match : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl > > , dynamic : NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElDynamic , }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    #[doc = "Set the field `full_path_match`.\nFor satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL."]
    pub fn set_full_path_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path_match = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `path_template_match`.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn set_path_template_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_match = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nFor satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `header_match`.\n"]
    pub fn set_header_match(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_match = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_parameter_match`.\n"]
    pub fn set_query_parameter_match(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameter_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter_match = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            path_template_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            header_match: core::default::Default::default(),
            query_parameter_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `full_path_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL."]
    pub fn full_path_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_path_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `path_template_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn path_template_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_template_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nFor satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `header_match` after provisioning.\n"]
    pub fn header_match(
        &self,
    ) -> ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElHeaderMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.header_match", self.base))
    }
    #[doc = "Get a reference to the value of field `query_parameter_match` after provisioning.\n"]    pub fn query_parameter_match (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElQueryParameterMatchElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_parameter_match", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl
{
    actions: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copied_parameters: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_query_parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_ttl: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl { # [doc = "Set the field `copied_parameters`.\nThe parameters to copy from the verified token to the generated token.\n\nOnly the following parameters may be copied:\n\n  * 'PathGlobs'\n  * 'paths'\n  * 'acl'\n  * 'URLPrefix'\n  * 'IPRanges'\n  * 'SessionID'\n  * 'id'\n  * 'Data'\n  * 'data'\n  * 'payload'\n  * 'Headers'\n\nYou may specify up to 6 parameters to copy.  A given parameter is be copied only if the parameter exists in the verified token.  Parameter names are matched exactly as specified.  The order of the parameters does not matter.  Duplicates are not allowed.\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified."] pub fn set_copied_parameters (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . copied_parameters = Some (v . into ()) ; self } # [doc = "Set the field `keyset`.\nThe keyset to use for signature generation.\n\nThe following are both valid paths to an EdgeCacheKeyset resource:\n\n  * 'projects/project/locations/global/edgeCacheKeysets/yourKeyset'\n  * 'yourKeyset'\n\nThis must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise."] pub fn set_keyset (mut self , v : impl Into < PrimField < String > >) -> Self { self . keyset = Some (v . into ()) ; self } # [doc = "Set the field `token_query_parameter`.\nThe query parameter in which to put the generated token.\n\nIf not specified, defaults to 'edge-cache-token'.\n\nIf specified, the name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nThis field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified."] pub fn set_token_query_parameter (mut self , v : impl Into < PrimField < String > >) -> Self { self . token_query_parameter = Some (v . into ()) ; self } # [doc = "Set the field `token_ttl`.\nThe duration the token is valid starting from the moment the token is first generated.\n\nDefaults to '86400s' (1 day).\n\nThe TTL must be >= 0 and <= 604,800 seconds (1 week).\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."] pub fn set_token_ttl (mut self , v : impl Into < PrimField < String > >) -> Self { self . token_ttl = Some (v . into ()) ; self } }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl
{
    #[doc = "The actions to take to add signatures to responses. Possible values: [\"GENERATE_COOKIE\", \"GENERATE_TOKEN_HLS_COOKIELESS\", \"PROPAGATE_TOKEN_HLS_COOKIELESS\"]"]
    pub actions: ListField<PrimField<String>>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl { actions : self . actions , copied_parameters : core :: default :: Default :: default () , keyset : core :: default :: Default :: default () , token_query_parameter : core :: default :: Default :: default () , token_ttl : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\nThe actions to take to add signatures to responses. Possible values: [\"GENERATE_COOKIE\", \"GENERATE_TOKEN_HLS_COOKIELESS\", \"PROPAGATE_TOKEN_HLS_COOKIELESS\"]"] pub fn actions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } # [doc = "Get a reference to the value of field `copied_parameters` after provisioning.\nThe parameters to copy from the verified token to the generated token.\n\nOnly the following parameters may be copied:\n\n  * 'PathGlobs'\n  * 'paths'\n  * 'acl'\n  * 'URLPrefix'\n  * 'IPRanges'\n  * 'SessionID'\n  * 'id'\n  * 'Data'\n  * 'data'\n  * 'payload'\n  * 'Headers'\n\nYou may specify up to 6 parameters to copy.  A given parameter is be copied only if the parameter exists in the verified token.  Parameter names are matched exactly as specified.  The order of the parameters does not matter.  Duplicates are not allowed.\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified."] pub fn copied_parameters (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.copied_parameters" , self . base)) } # [doc = "Get a reference to the value of field `keyset` after provisioning.\nThe keyset to use for signature generation.\n\nThe following are both valid paths to an EdgeCacheKeyset resource:\n\n  * 'projects/project/locations/global/edgeCacheKeysets/yourKeyset'\n  * 'yourKeyset'\n\nThis must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise."] pub fn keyset (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.keyset" , self . base)) } # [doc = "Get a reference to the value of field `token_query_parameter` after provisioning.\nThe query parameter in which to put the generated token.\n\nIf not specified, defaults to 'edge-cache-token'.\n\nIf specified, the name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nThis field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified."] pub fn token_query_parameter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token_query_parameter" , self . base)) } # [doc = "Get a reference to the value of field `token_ttl` after provisioning.\nThe duration the token is valid starting from the moment the token is first generated.\n\nDefaults to '86400s' (1 day).\n\nThe TTL must be >= 0 and <= 604,800 seconds (1 week).\n\nThis field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."] pub fn token_ttl (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token_ttl" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_host: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_query_string: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_query_parameters: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_protocol: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cookie_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_header_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_query_parameters: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl { # [doc = "Set the field `exclude_host`.\nIf true, requests to different hosts will be cached separately.\n\nNote: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched."] pub fn set_exclude_host (mut self , v : impl Into < PrimField < bool > >) -> Self { self . exclude_host = Some (v . into ()) ; self } # [doc = "Set the field `exclude_query_string`.\nIf true, exclude query string parameters from the cache key\n\nIf false (the default), include the query string parameters in\nthe cache key according to includeQueryParameters and\nexcludeQueryParameters. If neither includeQueryParameters nor\nexcludeQueryParameters is set, the entire query string will be\nincluded."] pub fn set_exclude_query_string (mut self , v : impl Into < PrimField < bool > >) -> Self { self . exclude_query_string = Some (v . into ()) ; self } # [doc = "Set the field `excluded_query_parameters`.\nNames of query string parameters to exclude from cache keys. All other parameters will be included.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."] pub fn set_excluded_query_parameters (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . excluded_query_parameters = Some (v . into ()) ; self } # [doc = "Set the field `include_protocol`.\nIf true, http and https requests will be cached separately."] pub fn set_include_protocol (mut self , v : impl Into < PrimField < bool > >) -> Self { self . include_protocol = Some (v . into ()) ; self } # [doc = "Set the field `included_cookie_names`.\nNames of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.\n\nCookie names:\n  - must be valid RFC 6265 \"cookie-name\" tokens\n  - are case sensitive\n  - cannot start with \"Edge-Cache-\" (case insensitive)\n\n  Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.\n\n  You may specify up to three cookie names."] pub fn set_included_cookie_names (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_cookie_names = Some (v . into ()) ; self } # [doc = "Set the field `included_header_names`.\nNames of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.\n\n- Header names must be valid HTTP RFC 7230 header field values.\n- Header field names are case insensitive\n- To include the HTTP method, use \":method\"\n\nNote that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance."] pub fn set_included_header_names (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_header_names = Some (v . into ()) ; self } # [doc = "Set the field `included_query_parameters`.\nNames of query string parameters to include in cache keys. All other parameters will be excluded.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."] pub fn set_included_query_parameters (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_query_parameters = Some (v . into ()) ; self } }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl
{}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl { exclude_host : core :: default :: Default :: default () , exclude_query_string : core :: default :: Default :: default () , excluded_query_parameters : core :: default :: Default :: default () , include_protocol : core :: default :: Default :: default () , included_cookie_names : core :: default :: Default :: default () , included_header_names : core :: default :: Default :: default () , included_query_parameters : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `exclude_host` after provisioning.\nIf true, requests to different hosts will be cached separately.\n\nNote: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched."] pub fn exclude_host (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.exclude_host" , self . base)) } # [doc = "Get a reference to the value of field `exclude_query_string` after provisioning.\nIf true, exclude query string parameters from the cache key\n\nIf false (the default), include the query string parameters in\nthe cache key according to includeQueryParameters and\nexcludeQueryParameters. If neither includeQueryParameters nor\nexcludeQueryParameters is set, the entire query string will be\nincluded."] pub fn exclude_query_string (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.exclude_query_string" , self . base)) } # [doc = "Get a reference to the value of field `excluded_query_parameters` after provisioning.\nNames of query string parameters to exclude from cache keys. All other parameters will be included.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."] pub fn excluded_query_parameters (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.excluded_query_parameters" , self . base)) } # [doc = "Get a reference to the value of field `include_protocol` after provisioning.\nIf true, http and https requests will be cached separately."] pub fn include_protocol (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.include_protocol" , self . base)) } # [doc = "Get a reference to the value of field `included_cookie_names` after provisioning.\nNames of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.\n\nCookie names:\n  - must be valid RFC 6265 \"cookie-name\" tokens\n  - are case sensitive\n  - cannot start with \"Edge-Cache-\" (case insensitive)\n\n  Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.\n\n  You may specify up to three cookie names."] pub fn included_cookie_names (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_cookie_names" , self . base)) } # [doc = "Get a reference to the value of field `included_header_names` after provisioning.\nNames of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.\n\n- Header names must be valid HTTP RFC 7230 header field values.\n- Header field names are case insensitive\n- To include the HTTP method, use \":method\"\n\nNote that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance."] pub fn included_header_names (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_header_names" , self . base)) } # [doc = "Get a reference to the value of field `included_query_parameters` after provisioning.\nNames of query string parameters to include in cache keys. All other parameters will be excluded.\n\nEither specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters."] pub fn included_query_parameters (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_query_parameters" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_signature_algorithms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_query_parameter: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl { # [doc = "Set the field `allowed_signature_algorithms`.\nThe allowed signature algorithms to use.\n\nDefaults to using only ED25519.\n\nYou may specify up to 3 signature algorithms to use. Possible values: [\"ED25519\", \"HMAC_SHA_256\", \"HMAC_SHA1\"]"] pub fn set_allowed_signature_algorithms (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . allowed_signature_algorithms = Some (v . into ()) ; self } # [doc = "Set the field `token_query_parameter`.\nThe query parameter in which to find the token.\n\nThe name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nDefaults to 'edge-cache-token'."] pub fn set_token_query_parameter (mut self , v : impl Into < PrimField < String > >) -> Self { self . token_query_parameter = Some (v . into ()) ; self } }
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl { type O = BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl
{}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl { pub fn build (self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl { allowed_signature_algorithms : core :: default :: Default :: default () , token_query_parameter : core :: default :: Default :: default () , } } }
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef { fn new (shared : StackShared , base : String) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef { NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef { shared : shared , base : base . to_string () , } } }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allowed_signature_algorithms` after provisioning.\nThe allowed signature algorithms to use.\n\nDefaults to using only ED25519.\n\nYou may specify up to 3 signature algorithms to use. Possible values: [\"ED25519\", \"HMAC_SHA_256\", \"HMAC_SHA1\"]"] pub fn allowed_signature_algorithms (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.allowed_signature_algorithms" , self . base)) } # [doc = "Get a reference to the value of field `token_query_parameter` after provisioning.\nThe query parameter in which to find the token.\n\nThe name must be 1-64 characters long and match the regular expression '[a-zA-Z]([a-zA-Z0-9_-])*' which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.\n\nDefaults to 'edge-cache-token'."] pub fn token_query_parameter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token_query_parameter" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElDynamic { add_signatures : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl >> , cache_key_policy : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl >> , signed_token_options : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl >> , }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl { # [serde (skip_serializing_if = "Option::is_none")] cache_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] client_ttl : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] default_ttl : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] max_ttl : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] negative_caching : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] negative_caching_policy : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] signed_request_keyset : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] signed_request_maximum_expiration_ttl : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] signed_request_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] add_signatures : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl > > , # [serde (skip_serializing_if = "Option::is_none")] cache_key_policy : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl > > , # [serde (skip_serializing_if = "Option::is_none")] signed_token_options : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl > > , dynamic : NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElDynamic , }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    #[doc = "Set the field `cache_mode`.\nCache modes allow users to control the behaviour of the cache, what content it should cache automatically, whether to respect origin headers, or whether to unconditionally cache all responses.\n\nFor all cache modes, Cache-Control headers will be passed to the client. Use clientTtl to override what is sent to the client. Possible values: [\"CACHE_ALL_STATIC\", \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"BYPASS_CACHE\"]"]
    pub fn set_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ttl`.\nSpecifies a separate client (e.g. browser client) TTL, separate from the TTL used by the edge caches. Leaving this empty will use the same cache TTL for both the CDN and the client-facing response.\n\n- The TTL must be > 0 and <= 86400s (1 day)\n- The clientTtl cannot be larger than the defaultTtl (if set)\n- Fractions of a second are not allowed.\n\nOmit this field to use the defaultTtl, or the max-age set by the origin, as the client-facing TTL.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_client_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses that do not have an existing valid TTL (max-age or s-max-age).\n\nDefaults to 3600s (1 hour).\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\" (equivalent to must-revalidate)\n- The value of defaultTTL cannot be set to a value greater than that of maxTTL.\n- Fractions of a second are not allowed.\n- When the cacheMode is set to FORCE_CACHE_ALL, the defaultTTL will overwrite the TTL set in all responses.\n\nNote that infrequently accessed objects may be evicted from the cache before the defined TTL. Objects that expire will be revalidated with the origin.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin.\n\nDefaults to 86400s (1 day).\n\nCache directives that attempt to set a max-age or s-maxage higher than this, or an Expires header more than maxTtl seconds in the future will be capped at the value of maxTTL, as if it were the value of an s-maxage Cache-Control directive.\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\"\n- The value of maxTtl must be equal to or greater than defaultTtl.\n- Fractions of a second are not allowed.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects. This can reduce the load on your origin and improve end-user experience by reducing response latency.\n\nBy default, the CDNPolicy will apply the following default TTLs to these status codes:\n\n- HTTP 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n- HTTP 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n- HTTP 405 (Method Not Found), 414 (URI Too Long), 501 (Not Implemented): 60s\n\nThese defaults can be overridden in negativeCachingPolicy"]
    pub fn set_negative_caching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negative_caching = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching_policy`.\nSets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.\n\n- Omitting the policy and leaving negativeCaching enabled will use the default TTLs for each status code, defined in negativeCaching.\n- TTLs must be >= 0 (where 0 is \"always revalidate\") and <= 86400s (1 day)\n\nNote that when specifying an explicit negativeCachingPolicy, you should take care to specify a cache TTL for all response codes that you wish to cache. The CDNPolicy will not apply any default negative caching when a policy exists."]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.negative_caching_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_request_keyset`.\nThe EdgeCacheKeyset containing the set of public keys used to validate signed requests at the edge."]
    pub fn set_signed_request_keyset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signed_request_keyset = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_request_maximum_expiration_ttl`.\nLimit how far into the future the expiration time of a signed request may be.\n\nWhen set, a signed request is rejected if its expiration time is later than now + signedRequestMaximumExpirationTtl, where now is the time at which the signed request is first handled by the CDN.\n\n- The TTL must be > 0.\n- Fractions of a second are not allowed.\n\nBy default, signedRequestMaximumExpirationTtl is not set and the expiration time of a signed request may be arbitrarily far into future."]
    pub fn set_signed_request_maximum_expiration_ttl(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.signed_request_maximum_expiration_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_request_mode`.\nWhether to enforce signed requests. The default value is DISABLED, which means all content is public, and does not authorize access.\n\nYou must also set a signedRequestKeyset to enable signed requests.\n\nWhen set to REQUIRE_SIGNATURES, all matching requests will have their signature validated. Requests that were not signed with the corresponding private key, or that are otherwise invalid (expired, do not match the signature, IP address, or header) will be rejected with a HTTP 403 and (if enabled) logged. Possible values: [\"DISABLED\", \"REQUIRE_SIGNATURES\", \"REQUIRE_TOKENS\"]"]
    pub fn set_signed_request_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signed_request_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `add_signatures`.\n"]
    pub fn set_add_signatures(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.add_signatures = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.add_signatures = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyEl >>,
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
    #[doc = "Set the field `signed_token_options`.\n"]
    pub fn set_signed_token_options(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.signed_token_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.signed_token_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl
{}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            signed_request_keyset: core::default::Default::default(),
            signed_request_maximum_expiration_ttl: core::default::Default::default(),
            signed_request_mode: core::default::Default::default(),
            add_signatures: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            signed_token_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef { shared : shared , base : base . to_string () , }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\nCache modes allow users to control the behaviour of the cache, what content it should cache automatically, whether to respect origin headers, or whether to unconditionally cache all responses.\n\nFor all cache modes, Cache-Control headers will be passed to the client. Use clientTtl to override what is sent to the client. Possible values: [\"CACHE_ALL_STATIC\", \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"BYPASS_CACHE\"]"]
    pub fn cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies a separate client (e.g. browser client) TTL, separate from the TTL used by the edge caches. Leaving this empty will use the same cache TTL for both the CDN and the client-facing response.\n\n- The TTL must be > 0 and <= 86400s (1 day)\n- The clientTtl cannot be larger than the defaultTtl (if set)\n- Fractions of a second are not allowed.\n\nOmit this field to use the defaultTtl, or the max-age set by the origin, as the client-facing TTL.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn client_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses that do not have an existing valid TTL (max-age or s-max-age).\n\nDefaults to 3600s (1 hour).\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\" (equivalent to must-revalidate)\n- The value of defaultTTL cannot be set to a value greater than that of maxTTL.\n- Fractions of a second are not allowed.\n- When the cacheMode is set to FORCE_CACHE_ALL, the defaultTTL will overwrite the TTL set in all responses.\n\nNote that infrequently accessed objects may be evicted from the cache before the defined TTL. Objects that expire will be revalidated with the origin.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\" or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn default_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin.\n\nDefaults to 86400s (1 day).\n\nCache directives that attempt to set a max-age or s-maxage higher than this, or an Expires header more than maxTtl seconds in the future will be capped at the value of maxTTL, as if it were the value of an s-maxage Cache-Control directive.\n\n- The TTL must be >= 0 and <= 31,536,000 seconds (1 year)\n- Setting a TTL of \"0\" means \"always revalidate\"\n- The value of maxTtl must be equal to or greater than defaultTtl.\n- Fractions of a second are not allowed.\n\nWhen the cache mode is set to \"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", or \"BYPASS_CACHE\", you must omit this field.\n\nA duration in seconds terminated by 's'. Example: \"3s\"."]
    pub fn max_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects. This can reduce the load on your origin and improve end-user experience by reducing response latency.\n\nBy default, the CDNPolicy will apply the following default TTLs to these status codes:\n\n- HTTP 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n- HTTP 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n- HTTP 405 (Method Not Found), 414 (URI Too Long), 501 (Not Implemented): 60s\n\nThese defaults can be overridden in negativeCachingPolicy"]
    pub fn negative_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negative_caching", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\nSets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.\n\n- Omitting the policy and leaving negativeCaching enabled will use the default TTLs for each status code, defined in negativeCaching.\n- TTLs must be >= 0 (where 0 is \"always revalidate\") and <= 86400s (1 day)\n\nNote that when specifying an explicit negativeCachingPolicy, you should take care to specify a cache TTL for all response codes that you wish to cache. The CDNPolicy will not apply any default negative caching when a policy exists."]
    pub fn negative_caching_policy(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_request_keyset` after provisioning.\nThe EdgeCacheKeyset containing the set of public keys used to validate signed requests at the edge."]
    pub fn signed_request_keyset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_request_keyset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_request_maximum_expiration_ttl` after provisioning.\nLimit how far into the future the expiration time of a signed request may be.\n\nWhen set, a signed request is rejected if its expiration time is later than now + signedRequestMaximumExpirationTtl, where now is the time at which the signed request is first handled by the CDN.\n\n- The TTL must be > 0.\n- Fractions of a second are not allowed.\n\nBy default, signedRequestMaximumExpirationTtl is not set and the expiration time of a signed request may be arbitrarily far into future."]
    pub fn signed_request_maximum_expiration_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_request_maximum_expiration_ttl", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_request_mode` after provisioning.\nWhether to enforce signed requests. The default value is DISABLED, which means all content is public, and does not authorize access.\n\nYou must also set a signedRequestKeyset to enable signed requests.\n\nWhen set to REQUIRE_SIGNATURES, all matching requests will have their signature validated. Requests that were not signed with the corresponding private key, or that are otherwise invalid (expired, do not match the signature, IP address, or header) will be rejected with a HTTP 403 and (if enabled) logged. Possible values: [\"DISABLED\", \"REQUIRE_SIGNATURES\", \"REQUIRE_TOKENS\"]"]
    pub fn signed_request_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_request_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `add_signatures` after provisioning.\n"]    pub fn add_signatures (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElAddSignaturesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.add_signatures", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]    pub fn cache_key_policy (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElCacheKeyPolicyElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_token_options` after provisioning.\n"]    pub fn signed_token_options (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElSignedTokenOptionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.signed_token_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    max_age: PrimField<String>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\n\nThis translates to the Access-Control-Allow-Credentials response header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods response header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\n\nThis translates to the Access-Control-Allow-Origin response header."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl
{
    #[doc = "Specifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).\n\n- Setting the value to -1 forces a pre-flight check for all requests (not recommended)\n- A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.\n- This translates to the Access-Control-Max-Age header.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub max_age: PrimField<String>,
}
impl
    BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl
{
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: self.max_age,
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef { shared : shared , base : base . to_string () , }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\n\nThis translates to the Access-Control-Allow-Credentials response header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods response header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\n\nThis translates to the Access-Control-Allow-Origin response header."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers response header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).\n\n- Setting the value to -1 forces a pre-flight check for all requests (not recommended)\n- A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.\n- This translates to the Access-Control-Max-Age header.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_rewrite: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_template_rewrite`.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn set_path_template_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl
{}
impl
    BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl
{
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef
    {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef { shared : shared , base : base . to_string () , }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_template_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn path_template_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_template_rewrite", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElDynamic { cdn_policy : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl >> , cors_policy : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl >> , url_rewrite : Option < DynamicBlock < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl >> , }
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl { # [serde (skip_serializing_if = "Option::is_none")] compression_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cdn_policy : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl > > , # [serde (skip_serializing_if = "Option::is_none")] cors_policy : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl > > , # [serde (skip_serializing_if = "Option::is_none")] url_rewrite : Option < Vec < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl > > , dynamic : NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElDynamic , }
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    #[doc = "Set the field `compression_mode`.\nSetting the compression mode to automatic enables dynamic compression for every eligible response.\n\nWhen dynamic compression is enabled, it is recommended to also set a cache policy to maximize efficiency. Possible values: [\"DISABLED\", \"AUTOMATIC\"]"]
    pub fn set_compression_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cdn_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cdn_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v : impl Into < BlockAssignable < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteEl >>,
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
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl {
            compression_mode: core::default::Default::default(),
            cdn_policy: core::default::Default::default(),
            cors_policy: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression_mode` after provisioning.\nSetting the compression mode to automatic enables dynamic compression for every eligible response.\n\nWhen dynamic compression is enabled, it is recommended to also set a cache policy to maximize efficiency. Possible values: [\"DISABLED\", \"AUTOMATIC\"]"]
    pub fn compression_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(
        &self,
    ) -> ListRef<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCdnPolicyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]    pub fn cors_policy (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElCorsPolicyElRef >{
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]    pub fn url_rewrite (& self) -> ListRef < NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElUrlRewriteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_methods: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {
    #[doc = "Set the field `allowed_methods`.\nThe non-empty set of HTTP methods that are allowed for this route.\n\nAny combination of \"GET\", \"HEAD\", \"OPTIONS\", \"PUT\", \"POST\", \"DELETE\", and \"PATCH\"."]
    pub fn set_allowed_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_methods = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl {
            allowed_methods: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\nThe non-empty set of HTTP methods that are allowed for this route.\n\nAny combination of \"GET\", \"HEAD\", \"OPTIONS\", \"PUT\", \"POST\", \"DELETE\", and \"PATCH\"."]
    pub fn allowed_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_methods", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_query: Option<PrimField<bool>>,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request.\n\nThis can only be set if there is at least one (1) edgeSslCertificate set on the service."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was supplied in the request.\n\npathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.\n\nThe path value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the routeRule, retaining the remaining portion of the URL before redirecting the request.\n\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction.\n\nThe supported values are:\n\n- 'MOVED_PERMANENTLY_DEFAULT', which is the default value and corresponds to 301.\n- 'FOUND', which corresponds to 302.\n- 'SEE_OTHER' which corresponds to 303.\n- 'TEMPORARY_REDIRECT', which corresponds to 307. in this case, the request method will be retained.\n- 'PERMANENT_REDIRECT', which corresponds to 308. in this case, the request method will be retained. Possible values: [\"MOVED_PERMANENTLY_DEFAULT\", \"FOUND\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\", \"PERMANENT_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
    #[doc = "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained."]
    pub fn set_strip_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strip_query = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl
{
    type O = BlockAssignable<
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
    pub fn build(
        self,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request.\n\nThis can only be set if there is at least one (1) edgeSslCertificate set on the service."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was supplied in the request.\n\npathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.\n\nThe path value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the routeRule, retaining the remaining portion of the URL before redirecting the request.\n\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction.\n\nThe supported values are:\n\n- 'MOVED_PERMANENTLY_DEFAULT', which is the default value and corresponds to 301.\n- 'FOUND', which corresponds to 302.\n- 'SEE_OTHER' which corresponds to 303.\n- 'TEMPORARY_REDIRECT', which corresponds to 307. in this case, the request method will be retained.\n- 'PERMANENT_REDIRECT', which corresponds to 308. in this case, the request method will be retained. Possible values: [\"MOVED_PERMANENTLY_DEFAULT\", \"FOUND\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\", \"PERMANENT_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElDynamic {
    header_action: Option<
        DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl,
        >,
    >,
    match_rule: Option<
        DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>,
    >,
    route_action: Option<
        DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl>,
    >,
    route_methods: Option<
        DynamicBlock<
            NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl,
        >,
    >,
    url_redirect: Option<
        DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
    priority: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action:
        Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_rule:
        Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_action:
        Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_methods:
        Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_redirect:
        Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElDynamic,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[doc = "Set the field `description`.\nA human-readable description of the routeRule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `origin`.\nThe Origin resource that requests to this route should fetch from when a matching response is not in cache. Origins can be defined as short names (\"my-origin\") or fully-qualified resource URLs - e.g. \"networkservices.googleapis.com/projects/my-project/global/edgecacheorigins/my-origin\"\n\nOnly one of origin or urlRedirect can be set."]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionEl,
            >,
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
    #[doc = "Set the field `match_rule`.\n"]
    pub fn set_match_rule(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `route_methods`.\n"]
    pub fn set_route_methods(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_methods = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_methods = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    #[doc = "The priority of this route rule, where 1 is the highest priority.\n\nYou cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers\nto which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules."]
    pub priority: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl {
            description: core::default::Default::default(),
            origin: core::default::Default::default(),
            priority: self.priority,
            header_action: core::default::Default::default(),
            match_rule: core::default::Default::default(),
            route_action: core::default::Default::default(),
            route_methods: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the routeRule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\nThe Origin resource that requests to this route should fetch from when a matching response is not in cache. Origins can be defined as short names (\"my-origin\") or fully-qualified resource URLs - e.g. \"networkservices.googleapis.com/projects/my-project/global/edgecacheorigins/my-origin\"\n\nOnly one of origin or urlRedirect can be set."]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route rule, where 1 is the highest priority.\n\nYou cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers\nto which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules."]
    pub fn priority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElHeaderActionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_rule` after provisioning.\n"]
    pub fn match_rule(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElMatchRuleElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.match_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteActionElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }
    #[doc = "Get a reference to the value of field `route_methods` after provisioning.\n"]
    pub fn route_methods(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRouteMethodsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.route_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElUrlRedirectElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElDynamic {
    route_rule:
        Option<DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_rule: Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElPathMatcherElDynamic,
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `route_rule`.\n"]
    pub fn set_route_rule(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl {
            description: core::default::Default::default(),
            name: self.name,
            route_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
        NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `route_rule` after provisioning.\n"]
    pub fn route_rule(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRouteRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_rule", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheServiceRoutingElDynamic {
    host_rule: Option<DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    path_matcher: Option<DynamicBlock<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceRoutingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rule: Option<Vec<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_matcher: Option<Vec<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
    dynamic: NetworkServicesEdgeCacheServiceRoutingElDynamic,
}
impl NetworkServicesEdgeCacheServiceRoutingEl {
    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElHostRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `path_matcher`.\n"]
    pub fn set_path_matcher(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheServiceRoutingElPathMatcherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_matcher = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheServiceRoutingEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceRoutingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceRoutingEl {}
impl BuildNetworkServicesEdgeCacheServiceRoutingEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceRoutingEl {
        NetworkServicesEdgeCacheServiceRoutingEl {
            host_rule: core::default::Default::default(),
            path_matcher: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceRoutingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceRoutingElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheServiceRoutingElRef {
        NetworkServicesEdgeCacheServiceRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceRoutingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rule` after provisioning.\n"]
    pub fn host_rule(&self) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElHostRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheServiceRoutingElPathMatcherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheServiceTimeoutsEl {
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
impl ToListMappable for NetworkServicesEdgeCacheServiceTimeoutsEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheServiceTimeoutsEl {}
impl BuildNetworkServicesEdgeCacheServiceTimeoutsEl {
    pub fn build(self) -> NetworkServicesEdgeCacheServiceTimeoutsEl {
        NetworkServicesEdgeCacheServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheServiceTimeoutsElRef {
        NetworkServicesEdgeCacheServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheServiceTimeoutsElRef {
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
struct NetworkServicesEdgeCacheServiceDynamic {
    log_config: Option<DynamicBlock<NetworkServicesEdgeCacheServiceLogConfigEl>>,
    routing: Option<DynamicBlock<NetworkServicesEdgeCacheServiceRoutingEl>>,
}
