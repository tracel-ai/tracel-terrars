use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeTargetHttpsProxyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_manager_certificates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_map: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_keep_alive_timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_bind: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quic_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_tls_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_certificates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_early_data: Option<PrimField<String>>,
    url_map: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeTargetHttpsProxyTimeoutsEl>,
}
struct ComputeTargetHttpsProxy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeTargetHttpsProxyData>,
}
#[derive(Clone)]
pub struct ComputeTargetHttpsProxy(Rc<ComputeTargetHttpsProxy_>);
impl ComputeTargetHttpsProxy {
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
    #[doc = "Set the field `certificate_manager_certificates`.\nURLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.\nCertificate manager certificates only apply when the load balancing scheme is set to INTERNAL_MANAGED.\nFor EXTERNAL and EXTERNAL_MANAGED, use certificate_map instead.\nsslCertificates and certificateManagerCertificates fields can not be defined together.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}' or just the self_link 'projects/{project}/locations/{location}/certificates/{resourceName}'"]
    pub fn set_certificate_manager_certificates(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().certificate_manager_certificates = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_map`.\nA reference to the CertificateMap resource uri that identifies a certificate map\nassociated with the given target proxy. This field is only supported for EXTERNAL and EXTERNAL_MANAGED load balancing schemes.\nFor INTERNAL_MANAGED, use certificate_manager_certificates instead.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}'."]
    pub fn set_certificate_map(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_map = Some(v.into());
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
    #[doc = "Set the field `http_keep_alive_timeout_sec`.\nSpecifies how long to keep a connection open, after completing a response,\nwhile there is no matching traffic (in seconds). If an HTTP keepalive is\nnot specified, a default value will be used. For Global\nexternal HTTP(S) load balancer, the default value is 610 seconds, the\nminimum allowed value is 5 seconds and the maximum allowed value is 1200\nseconds. For cross-region internal HTTP(S) load balancer, the default\nvalue is 600 seconds, the minimum allowed value is 5 seconds, and the\nmaximum allowed value is 600 seconds. For Global external HTTP(S) load\nbalancer (classic), this option is not available publicly."]
    pub fn set_http_keep_alive_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_keep_alive_timeout_sec = Some(v.into());
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
    #[doc = "Set the field `proxy_bind`.\nThis field only applies when the forwarding rule that references\nthis target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED."]
    pub fn set_proxy_bind(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxy_bind = Some(v.into());
        self
    }
    #[doc = "Set the field `quic_override`.\nSpecifies the QUIC override policy for this resource. This determines\nwhether the load balancer will attempt to negotiate QUIC with clients\nor not. Can specify one of NONE, ENABLE, or DISABLE. If NONE is\nspecified, Google manages whether QUIC is used. Default value: \"NONE\" Possible values: [\"NONE\", \"ENABLE\", \"DISABLE\"]"]
    pub fn set_quic_override(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quic_override = Some(v.into());
        self
    }
    #[doc = "Set the field `server_tls_policy`.\nA URL referring to a networksecurity.ServerTlsPolicy\nresource that describes how the proxy should authenticate inbound\ntraffic. serverTlsPolicy only applies to a global TargetHttpsProxy\nattached to globalForwardingRules with the loadBalancingScheme\nset to INTERNAL_SELF_MANAGED or EXTERNAL or EXTERNAL_MANAGED.\nFor details which ServerTlsPolicy resources are accepted with\nINTERNAL_SELF_MANAGED and which with EXTERNAL, EXTERNAL_MANAGED\nloadBalancingScheme consult ServerTlsPolicy documentation.\nIf left blank, communications are not encrypted.\n\nIf you remove this field from your configuration at the same time as\ndeleting or recreating a referenced ServerTlsPolicy resource, you will\nreceive a resourceInUseByAnotherResource error. Use lifecycle.create_before_destroy\nwithin the ServerTlsPolicy resource to avoid this."]
    pub fn set_server_tls_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_tls_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_certificates`.\nURLs to SslCertificate resources that are used to authenticate connections between users and the load balancer.\nCurrently, you may specify up to 15 SSL certificates. sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.\nsslCertificates and certificateManagerCertificates can not be defined together."]
    pub fn set_ssl_certificates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ssl_certificates = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_policy`.\nA reference to the SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_early_data`.\nSpecifies whether TLS 1.3 0-RTT Data (“Early Data”) should be accepted for this service.\nEarly Data allows a TLS resumption handshake to include the initial application payload\n(a HTTP request) alongside the handshake, reducing the effective round trips to “zero”.\nThis applies to TLS 1.3 connections over TCP (HTTP/2) as well as over UDP (QUIC/h3). Possible values: [\"STRICT\", \"PERMISSIVE\", \"UNRESTRICTED\", \"DISABLED\"]"]
    pub fn set_tls_early_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tls_early_data = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeTargetHttpsProxyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `certificate_manager_certificates` after provisioning.\nURLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.\nCertificate manager certificates only apply when the load balancing scheme is set to INTERNAL_MANAGED.\nFor EXTERNAL and EXTERNAL_MANAGED, use certificate_map instead.\nsslCertificates and certificateManagerCertificates fields can not be defined together.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}' or just the self_link 'projects/{project}/locations/{location}/certificates/{resourceName}'"]
    pub fn certificate_manager_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_manager_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_map` after provisioning.\nA reference to the CertificateMap resource uri that identifies a certificate map\nassociated with the given target proxy. This field is only supported for EXTERNAL and EXTERNAL_MANAGED load balancing schemes.\nFor INTERNAL_MANAGED, use certificate_manager_certificates instead.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}'."]
    pub fn certificate_map(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_map", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking.\nThis field will be ignored when inserting a TargetHttpsProxy. An up-to-date fingerprint must be provided in order to\npatch the TargetHttpsProxy; otherwise, the request will fail with error 412 conditionNotMet.\nTo see the latest fingerprint, make a get() request to retrieve the TargetHttpsProxy.\nA base64-encoded string."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_keep_alive_timeout_sec` after provisioning.\nSpecifies how long to keep a connection open, after completing a response,\nwhile there is no matching traffic (in seconds). If an HTTP keepalive is\nnot specified, a default value will be used. For Global\nexternal HTTP(S) load balancer, the default value is 610 seconds, the\nminimum allowed value is 5 seconds and the maximum allowed value is 1200\nseconds. For cross-region internal HTTP(S) load balancer, the default\nvalue is 600 seconds, the minimum allowed value is 5 seconds, and the\nmaximum allowed value is 600 seconds. For Global external HTTP(S) load\nbalancer (classic), this option is not available publicly."]
    pub fn http_keep_alive_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_keep_alive_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `proxy_bind` after provisioning.\nThis field only applies when the forwarding rule that references\nthis target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED."]
    pub fn proxy_bind(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_bind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn proxy_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quic_override` after provisioning.\nSpecifies the QUIC override policy for this resource. This determines\nwhether the load balancer will attempt to negotiate QUIC with clients\nor not. Can specify one of NONE, ENABLE, or DISABLE. If NONE is\nspecified, Google manages whether QUIC is used. Default value: \"NONE\" Possible values: [\"NONE\", \"ENABLE\", \"DISABLE\"]"]
    pub fn quic_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quic_override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to a networksecurity.ServerTlsPolicy\nresource that describes how the proxy should authenticate inbound\ntraffic. serverTlsPolicy only applies to a global TargetHttpsProxy\nattached to globalForwardingRules with the loadBalancingScheme\nset to INTERNAL_SELF_MANAGED or EXTERNAL or EXTERNAL_MANAGED.\nFor details which ServerTlsPolicy resources are accepted with\nINTERNAL_SELF_MANAGED and which with EXTERNAL, EXTERNAL_MANAGED\nloadBalancingScheme consult ServerTlsPolicy documentation.\nIf left blank, communications are not encrypted.\n\nIf you remove this field from your configuration at the same time as\ndeleting or recreating a referenced ServerTlsPolicy resource, you will\nreceive a resourceInUseByAnotherResource error. Use lifecycle.create_before_destroy\nwithin the ServerTlsPolicy resource to avoid this."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_certificates` after provisioning.\nURLs to SslCertificate resources that are used to authenticate connections between users and the load balancer.\nCurrently, you may specify up to 15 SSL certificates. sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.\nsslCertificates and certificateManagerCertificates can not be defined together."]
    pub fn ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nA reference to the SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_early_data` after provisioning.\nSpecifies whether TLS 1.3 0-RTT Data (“Early Data”) should be accepted for this service.\nEarly Data allows a TLS resumption handshake to include the initial application payload\n(a HTTP request) alongside the handshake, reducing the effective round trips to “zero”.\nThis applies to TLS 1.3 connections over TCP (HTTP/2) as well as over UDP (QUIC/h3). Possible values: [\"STRICT\", \"PERMISSIVE\", \"UNRESTRICTED\", \"DISABLED\"]"]
    pub fn tls_early_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_early_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_map` after provisioning.\nA reference to the UrlMap resource that defines the mapping from URL\nto the BackendService."]
    pub fn url_map(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.url_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeTargetHttpsProxyTimeoutsElRef {
        ComputeTargetHttpsProxyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeTargetHttpsProxy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeTargetHttpsProxy {}
impl ToListMappable for ComputeTargetHttpsProxy {
    type O = ListRef<ComputeTargetHttpsProxyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeTargetHttpsProxy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_target_https_proxy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeTargetHttpsProxy {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "A reference to the UrlMap resource that defines the mapping from URL\nto the BackendService."]
    pub url_map: PrimField<String>,
}
impl BuildComputeTargetHttpsProxy {
    pub fn build(self, stack: &mut Stack) -> ComputeTargetHttpsProxy {
        let out = ComputeTargetHttpsProxy(Rc::new(ComputeTargetHttpsProxy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeTargetHttpsProxyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_manager_certificates: core::default::Default::default(),
                certificate_map: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                http_keep_alive_timeout_sec: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                proxy_bind: core::default::Default::default(),
                quic_override: core::default::Default::default(),
                server_tls_policy: core::default::Default::default(),
                ssl_certificates: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                tls_early_data: core::default::Default::default(),
                url_map: self.url_map,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeTargetHttpsProxyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeTargetHttpsProxyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeTargetHttpsProxyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_manager_certificates` after provisioning.\nURLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.\nCertificate manager certificates only apply when the load balancing scheme is set to INTERNAL_MANAGED.\nFor EXTERNAL and EXTERNAL_MANAGED, use certificate_map instead.\nsslCertificates and certificateManagerCertificates fields can not be defined together.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}' or just the self_link 'projects/{project}/locations/{location}/certificates/{resourceName}'"]
    pub fn certificate_manager_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_manager_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_map` after provisioning.\nA reference to the CertificateMap resource uri that identifies a certificate map\nassociated with the given target proxy. This field is only supported for EXTERNAL and EXTERNAL_MANAGED load balancing schemes.\nFor INTERNAL_MANAGED, use certificate_manager_certificates instead.\nAccepted format is '//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}'."]
    pub fn certificate_map(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_map", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking.\nThis field will be ignored when inserting a TargetHttpsProxy. An up-to-date fingerprint must be provided in order to\npatch the TargetHttpsProxy; otherwise, the request will fail with error 412 conditionNotMet.\nTo see the latest fingerprint, make a get() request to retrieve the TargetHttpsProxy.\nA base64-encoded string."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_keep_alive_timeout_sec` after provisioning.\nSpecifies how long to keep a connection open, after completing a response,\nwhile there is no matching traffic (in seconds). If an HTTP keepalive is\nnot specified, a default value will be used. For Global\nexternal HTTP(S) load balancer, the default value is 610 seconds, the\nminimum allowed value is 5 seconds and the maximum allowed value is 1200\nseconds. For cross-region internal HTTP(S) load balancer, the default\nvalue is 600 seconds, the minimum allowed value is 5 seconds, and the\nmaximum allowed value is 600 seconds. For Global external HTTP(S) load\nbalancer (classic), this option is not available publicly."]
    pub fn http_keep_alive_timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_keep_alive_timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `proxy_bind` after provisioning.\nThis field only applies when the forwarding rule that references\nthis target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED."]
    pub fn proxy_bind(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_bind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn proxy_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quic_override` after provisioning.\nSpecifies the QUIC override policy for this resource. This determines\nwhether the load balancer will attempt to negotiate QUIC with clients\nor not. Can specify one of NONE, ENABLE, or DISABLE. If NONE is\nspecified, Google manages whether QUIC is used. Default value: \"NONE\" Possible values: [\"NONE\", \"ENABLE\", \"DISABLE\"]"]
    pub fn quic_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quic_override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to a networksecurity.ServerTlsPolicy\nresource that describes how the proxy should authenticate inbound\ntraffic. serverTlsPolicy only applies to a global TargetHttpsProxy\nattached to globalForwardingRules with the loadBalancingScheme\nset to INTERNAL_SELF_MANAGED or EXTERNAL or EXTERNAL_MANAGED.\nFor details which ServerTlsPolicy resources are accepted with\nINTERNAL_SELF_MANAGED and which with EXTERNAL, EXTERNAL_MANAGED\nloadBalancingScheme consult ServerTlsPolicy documentation.\nIf left blank, communications are not encrypted.\n\nIf you remove this field from your configuration at the same time as\ndeleting or recreating a referenced ServerTlsPolicy resource, you will\nreceive a resourceInUseByAnotherResource error. Use lifecycle.create_before_destroy\nwithin the ServerTlsPolicy resource to avoid this."]
    pub fn server_tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_tls_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_certificates` after provisioning.\nURLs to SslCertificate resources that are used to authenticate connections between users and the load balancer.\nCurrently, you may specify up to 15 SSL certificates. sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.\nsslCertificates and certificateManagerCertificates can not be defined together."]
    pub fn ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nA reference to the SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_early_data` after provisioning.\nSpecifies whether TLS 1.3 0-RTT Data (“Early Data”) should be accepted for this service.\nEarly Data allows a TLS resumption handshake to include the initial application payload\n(a HTTP request) alongside the handshake, reducing the effective round trips to “zero”.\nThis applies to TLS 1.3 connections over TCP (HTTP/2) as well as over UDP (QUIC/h3). Possible values: [\"STRICT\", \"PERMISSIVE\", \"UNRESTRICTED\", \"DISABLED\"]"]
    pub fn tls_early_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_early_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_map` after provisioning.\nA reference to the UrlMap resource that defines the mapping from URL\nto the BackendService."]
    pub fn url_map(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.url_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeTargetHttpsProxyTimeoutsElRef {
        ComputeTargetHttpsProxyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeTargetHttpsProxyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeTargetHttpsProxyTimeoutsEl {
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
impl ToListMappable for ComputeTargetHttpsProxyTimeoutsEl {
    type O = BlockAssignable<ComputeTargetHttpsProxyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeTargetHttpsProxyTimeoutsEl {}
impl BuildComputeTargetHttpsProxyTimeoutsEl {
    pub fn build(self) -> ComputeTargetHttpsProxyTimeoutsEl {
        ComputeTargetHttpsProxyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeTargetHttpsProxyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeTargetHttpsProxyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeTargetHttpsProxyTimeoutsElRef {
        ComputeTargetHttpsProxyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeTargetHttpsProxyTimeoutsElRef {
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
