use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LbListenerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alpn_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    load_balancer_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_issuer_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_leaf_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name:
        Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_subject_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_validity_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_tls_cipher_suite_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_request_x_amzn_tls_version_header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_allow_credentials_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_allow_headers_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_allow_methods_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_allow_origin_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_expose_headers_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_access_control_max_age_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_content_security_policy_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_server_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_strict_transport_security_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_x_content_type_options_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_http_response_x_frame_options_header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_idle_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action: Option<Vec<LbListenerDefaultActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutual_authentication: Option<Vec<LbListenerMutualAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LbListenerTimeoutsEl>,
    dynamic: LbListenerDynamic,
}
struct LbListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LbListenerData>,
}
#[derive(Clone)]
pub struct LbListener(Rc<LbListener_>);
impl LbListener {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `alpn_policy`.\n"]
    pub fn set_alpn_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alpn_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_issuer_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_leaf_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_subject_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_mtls_clientcert_validity_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_tls_cipher_suite_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_tls_cipher_suite_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_tls_cipher_suite_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_request_x_amzn_tls_version_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_tls_version_header_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_request_x_amzn_tls_version_header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_allow_credentials_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_credentials_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_allow_credentials_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_allow_headers_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_headers_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_allow_headers_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_allow_methods_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_methods_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_allow_methods_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_allow_origin_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_origin_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_allow_origin_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_expose_headers_header_value`.\n"]
    pub fn set_routing_http_response_access_control_expose_headers_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_expose_headers_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_access_control_max_age_header_value`.\n"]
    pub fn set_routing_http_response_access_control_max_age_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_access_control_max_age_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_content_security_policy_header_value`.\n"]
    pub fn set_routing_http_response_content_security_policy_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_content_security_policy_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_server_enabled`.\n"]
    pub fn set_routing_http_response_server_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_server_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_strict_transport_security_header_value`.\n"]
    pub fn set_routing_http_response_strict_transport_security_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_strict_transport_security_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_x_content_type_options_header_value`.\n"]
    pub fn set_routing_http_response_x_content_type_options_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_x_content_type_options_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_http_response_x_frame_options_header_value`.\n"]
    pub fn set_routing_http_response_x_frame_options_header_value(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .routing_http_response_x_frame_options_header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_policy`.\n"]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `tcp_idle_timeout_seconds`.\n"]
    pub fn set_tcp_idle_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_idle_timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mutual_authentication`.\n"]
    pub fn set_mutual_authentication(
        self,
        v: impl Into<BlockAssignable<LbListenerMutualAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mutual_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mutual_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LbListenerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alpn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_issuer_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_leaf_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_subject_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_validity_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_tls_cipher_suite_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_cipher_suite_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_tls_cipher_suite_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_tls_version_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_version_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_tls_version_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_credentials_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_credentials_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_credentials_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_headers_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_headers_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_methods_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_methods_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_methods_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_origin_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_origin_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_origin_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_expose_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_expose_headers_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_expose_headers_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_max_age_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_max_age_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_max_age_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_content_security_policy_header_value` after provisioning.\n"]
    pub fn routing_http_response_content_security_policy_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_content_security_policy_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_server_enabled` after provisioning.\n"]
    pub fn routing_http_response_server_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_server_enabled",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_strict_transport_security_header_value` after provisioning.\n"]
    pub fn routing_http_response_strict_transport_security_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_strict_transport_security_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_x_content_type_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_content_type_options_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_x_content_type_options_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_x_frame_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_frame_options_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_x_frame_options_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_idle_timeout_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<LbListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> ListRef<LbListenerMutualAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mutual_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LbListener {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LbListener {}
impl ToListMappable for LbListener {
    type O = ListRef<LbListenerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LbListener_ {
    fn extract_resource_type(&self) -> String {
        "aws_lb_listener".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLbListener {
    pub tf_id: String,
    #[doc = ""]
    pub load_balancer_arn: PrimField<String>,
}
impl BuildLbListener {
    pub fn build(self, stack: &mut Stack) -> LbListener {
        let out = LbListener(Rc::new(LbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LbListenerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alpn_policy: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: self.load_balancer_arn,
                port: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_issuer_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_leaf_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_subject_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_validity_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_tls_cipher_suite_header_name:
                    core::default::Default::default(),
                routing_http_request_x_amzn_tls_version_header_name:
                    core::default::Default::default(),
                routing_http_response_access_control_allow_credentials_header_value:
                    core::default::Default::default(),
                routing_http_response_access_control_allow_headers_header_value:
                    core::default::Default::default(),
                routing_http_response_access_control_allow_methods_header_value:
                    core::default::Default::default(),
                routing_http_response_access_control_allow_origin_header_value:
                    core::default::Default::default(),
                routing_http_response_access_control_expose_headers_header_value:
                    core::default::Default::default(),
                routing_http_response_access_control_max_age_header_value:
                    core::default::Default::default(),
                routing_http_response_content_security_policy_header_value:
                    core::default::Default::default(),
                routing_http_response_server_enabled: core::default::Default::default(),
                routing_http_response_strict_transport_security_header_value:
                    core::default::Default::default(),
                routing_http_response_x_content_type_options_header_value:
                    core::default::Default::default(),
                routing_http_response_x_frame_options_header_value: core::default::Default::default(
                ),
                ssl_policy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tcp_idle_timeout_seconds: core::default::Default::default(),
                default_action: core::default::Default::default(),
                mutual_authentication: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LbListenerRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LbListenerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alpn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_issuer_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_leaf_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_subject_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_mtls_clientcert_validity_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_tls_cipher_suite_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_cipher_suite_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_tls_cipher_suite_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_request_x_amzn_tls_version_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_version_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_request_x_amzn_tls_version_header_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_credentials_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_credentials_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_credentials_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_headers_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_headers_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_methods_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_methods_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_methods_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_allow_origin_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_origin_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_allow_origin_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_expose_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_expose_headers_header_value(
        &self,
    ) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_expose_headers_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_access_control_max_age_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_max_age_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_access_control_max_age_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_content_security_policy_header_value` after provisioning.\n"]
    pub fn routing_http_response_content_security_policy_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_content_security_policy_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_server_enabled` after provisioning.\n"]
    pub fn routing_http_response_server_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_server_enabled",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_strict_transport_security_header_value` after provisioning.\n"]
    pub fn routing_http_response_strict_transport_security_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_strict_transport_security_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_x_content_type_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_content_type_options_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_x_content_type_options_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `routing_http_response_x_frame_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_frame_options_header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.routing_http_response_x_frame_options_header_value",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_idle_timeout_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<LbListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> ListRef<LbListenerMutualAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mutual_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElAuthenticateCognitoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    user_pool_arn: PrimField<String>,
    user_pool_client_id: PrimField<String>,
    user_pool_domain: PrimField<String>,
}
impl LbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }
    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }
    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElAuthenticateCognitoEl {
    type O = BlockAssignable<LbListenerDefaultActionElAuthenticateCognitoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: PrimField<String>,
    #[doc = ""]
    pub user_pool_client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool_domain: PrimField<String>,
}
impl BuildLbListenerDefaultActionElAuthenticateCognitoEl {
    pub fn build(self) -> LbListenerDefaultActionElAuthenticateCognitoEl {
        LbListenerDefaultActionElAuthenticateCognitoEl {
            authentication_request_extra_params: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            user_pool_arn: self.user_pool_arn,
            user_pool_client_id: self.user_pool_client_id,
            user_pool_domain: self.user_pool_domain,
        }
    }
}
pub struct LbListenerDefaultActionElAuthenticateCognitoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElAuthenticateCognitoElRef {
        LbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElAuthenticateCognitoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.authentication_request_extra_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_unauthenticated_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_cookie_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_client_id` after provisioning.\n"]
    pub fn user_pool_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_client_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_domain` after provisioning.\n"]
    pub fn user_pool_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_domain", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    authorization_endpoint: PrimField<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    issuer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    token_endpoint: PrimField<String>,
    user_info_endpoint: PrimField<String>,
}
impl LbListenerDefaultActionElAuthenticateOidcEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }
    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }
    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElAuthenticateOidcEl {
    type O = BlockAssignable<LbListenerDefaultActionElAuthenticateOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElAuthenticateOidcEl {
    #[doc = ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub token_endpoint: PrimField<String>,
    #[doc = ""]
    pub user_info_endpoint: PrimField<String>,
}
impl BuildLbListenerDefaultActionElAuthenticateOidcEl {
    pub fn build(self) -> LbListenerDefaultActionElAuthenticateOidcEl {
        LbListenerDefaultActionElAuthenticateOidcEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: self.authorization_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            issuer: self.issuer,
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
        }
    }
}
pub struct LbListenerDefaultActionElAuthenticateOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElAuthenticateOidcElRef {
        LbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElAuthenticateOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.authentication_request_extra_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_unauthenticated_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_cookie_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_info_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElFixedResponseEl {
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}
impl LbListenerDefaultActionElFixedResponseEl {
    #[doc = "Set the field `message_body`.\n"]
    pub fn set_message_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_body = Some(v.into());
        self
    }
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElFixedResponseEl {
    type O = BlockAssignable<LbListenerDefaultActionElFixedResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: PrimField<String>,
}
impl BuildLbListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> LbListenerDefaultActionElFixedResponseEl {
        LbListenerDefaultActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElFixedResponseElRef {
        LbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
    #[doc = "Get a reference to the value of field `message_body` after provisioning.\n"]
    pub fn message_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElForwardElStickinessEl {
    duration: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl LbListenerDefaultActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElForwardElStickinessEl {
    type O = BlockAssignable<LbListenerDefaultActionElForwardElStickinessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: PrimField<f64>,
}
impl BuildLbListenerDefaultActionElForwardElStickinessEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardElStickinessEl {
        LbListenerDefaultActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElForwardElStickinessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElForwardElStickinessElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElForwardElStickinessElRef {
        LbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElForwardElStickinessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElForwardElTargetGroupEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl LbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElForwardElTargetGroupEl {
    type O = BlockAssignable<LbListenerDefaultActionElForwardElTargetGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}
impl BuildLbListenerDefaultActionElForwardElTargetGroupEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardElTargetGroupEl {
        LbListenerDefaultActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElForwardElTargetGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LbListenerDefaultActionElForwardElTargetGroupElRef {
        LbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElForwardElTargetGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct LbListenerDefaultActionElForwardElDynamic {
    stickiness: Option<DynamicBlock<LbListenerDefaultActionElForwardElStickinessEl>>,
    target_group: Option<DynamicBlock<LbListenerDefaultActionElForwardElTargetGroupEl>>,
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<LbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<Vec<LbListenerDefaultActionElForwardElTargetGroupEl>>,
    dynamic: LbListenerDefaultActionElForwardElDynamic,
}
impl LbListenerDefaultActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElForwardElStickinessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stickiness = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElForwardElTargetGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElForwardEl {
    type O = BlockAssignable<LbListenerDefaultActionElForwardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElForwardEl {}
impl BuildLbListenerDefaultActionElForwardEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardEl {
        LbListenerDefaultActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElForwardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElForwardElRef {
        LbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<LbListenerDefaultActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    format: PrimField<String>,
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl LbListenerDefaultActionElJwtValidationElAdditionalClaimEl {}
impl ToListMappable for LbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    type O = BlockAssignable<LbListenerDefaultActionElJwtValidationElAdditionalClaimEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildLbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
    pub fn build(self) -> LbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
        LbListenerDefaultActionElJwtValidationElAdditionalClaimEl {
            format: self.format,
            name: self.name,
            values: self.values,
        }
    }
}
pub struct LbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
        LbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElJwtValidationElAdditionalClaimElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct LbListenerDefaultActionElJwtValidationElDynamic {
    additional_claim:
        Option<DynamicBlock<LbListenerDefaultActionElJwtValidationElAdditionalClaimEl>>,
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElJwtValidationEl {
    issuer: PrimField<String>,
    jwks_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_claim: Option<Vec<LbListenerDefaultActionElJwtValidationElAdditionalClaimEl>>,
    dynamic: LbListenerDefaultActionElJwtValidationElDynamic,
}
impl LbListenerDefaultActionElJwtValidationEl {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElJwtValidationElAdditionalClaimEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_claim = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_claim = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElJwtValidationEl {
    type O = BlockAssignable<LbListenerDefaultActionElJwtValidationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElJwtValidationEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub jwks_endpoint: PrimField<String>,
}
impl BuildLbListenerDefaultActionElJwtValidationEl {
    pub fn build(self) -> LbListenerDefaultActionElJwtValidationEl {
        LbListenerDefaultActionElJwtValidationEl {
            issuer: self.issuer,
            jwks_endpoint: self.jwks_endpoint,
            additional_claim: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElJwtValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElJwtValidationElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElJwtValidationElRef {
        LbListenerDefaultActionElJwtValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElJwtValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks_endpoint` after provisioning.\n"]
    pub fn jwks_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.jwks_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    status_code: PrimField<String>,
}
impl LbListenerDefaultActionElRedirectEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerDefaultActionElRedirectEl {
    type O = BlockAssignable<LbListenerDefaultActionElRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionElRedirectEl {
    #[doc = ""]
    pub status_code: PrimField<String>,
}
impl BuildLbListenerDefaultActionElRedirectEl {
    pub fn build(self) -> LbListenerDefaultActionElRedirectEl {
        LbListenerDefaultActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}
pub struct LbListenerDefaultActionElRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElRedirectElRef {
        LbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize, Default)]
struct LbListenerDefaultActionElDynamic {
    authenticate_cognito: Option<DynamicBlock<LbListenerDefaultActionElAuthenticateCognitoEl>>,
    authenticate_oidc: Option<DynamicBlock<LbListenerDefaultActionElAuthenticateOidcEl>>,
    fixed_response: Option<DynamicBlock<LbListenerDefaultActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<LbListenerDefaultActionElForwardEl>>,
    jwt_validation: Option<DynamicBlock<LbListenerDefaultActionElJwtValidationEl>>,
    redirect: Option<DynamicBlock<LbListenerDefaultActionElRedirectEl>>,
}
#[derive(Serialize)]
pub struct LbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<Vec<LbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<Vec<LbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<LbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<LbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_validation: Option<Vec<LbListenerDefaultActionElJwtValidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<LbListenerDefaultActionElRedirectEl>>,
    dynamic: LbListenerDefaultActionElDynamic,
}
impl LbListenerDefaultActionEl {
    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }
    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElAuthenticateCognitoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_cognito = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_cognito = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElAuthenticateOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_oidc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_oidc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElForwardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElJwtValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jwt_validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_validation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl Into<BlockAssignable<LbListenerDefaultActionElRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LbListenerDefaultActionEl {
    type O = BlockAssignable<LbListenerDefaultActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerDefaultActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildLbListenerDefaultActionEl {
    pub fn build(self) -> LbListenerDefaultActionEl {
        LbListenerDefaultActionEl {
            order: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: self.type_,
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            jwt_validation: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LbListenerDefaultActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> LbListenerDefaultActionElRef {
        LbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_group_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(
        &self,
    ) -> ListRef<LbListenerDefaultActionElAuthenticateCognitoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticate_cognito", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<LbListenerDefaultActionElAuthenticateOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticate_oidc", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<LbListenerDefaultActionElFixedResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<LbListenerDefaultActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> ListRef<LbListenerDefaultActionElJwtValidationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jwt_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<LbListenerDefaultActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
#[derive(Serialize)]
pub struct LbListenerMutualAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_trust_store_ca_names: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_client_certificate_expiry: Option<PrimField<bool>>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_store_arn: Option<PrimField<String>>,
}
impl LbListenerMutualAuthenticationEl {
    #[doc = "Set the field `advertise_trust_store_ca_names`.\n"]
    pub fn set_advertise_trust_store_ca_names(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_trust_store_ca_names = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_client_certificate_expiry`.\n"]
    pub fn set_ignore_client_certificate_expiry(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_client_certificate_expiry = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_store_arn`.\n"]
    pub fn set_trust_store_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trust_store_arn = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerMutualAuthenticationEl {
    type O = BlockAssignable<LbListenerMutualAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerMutualAuthenticationEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}
impl BuildLbListenerMutualAuthenticationEl {
    pub fn build(self) -> LbListenerMutualAuthenticationEl {
        LbListenerMutualAuthenticationEl {
            advertise_trust_store_ca_names: core::default::Default::default(),
            ignore_client_certificate_expiry: core::default::Default::default(),
            mode: self.mode,
            trust_store_arn: core::default::Default::default(),
        }
    }
}
pub struct LbListenerMutualAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerMutualAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> LbListenerMutualAuthenticationElRef {
        LbListenerMutualAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerMutualAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_trust_store_ca_names` after provisioning.\n"]
    pub fn advertise_trust_store_ca_names(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_trust_store_ca_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_client_certificate_expiry` after provisioning.\n"]
    pub fn ignore_client_certificate_expiry(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_client_certificate_expiry", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `trust_store_arn` after provisioning.\n"]
    pub fn trust_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_store_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LbListenerTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for LbListenerTimeoutsEl {
    type O = BlockAssignable<LbListenerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLbListenerTimeoutsEl {}
impl BuildLbListenerTimeoutsEl {
    pub fn build(self) -> LbListenerTimeoutsEl {
        LbListenerTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LbListenerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LbListenerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LbListenerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct LbListenerDynamic {
    default_action: Option<DynamicBlock<LbListenerDefaultActionEl>>,
    mutual_authentication: Option<DynamicBlock<LbListenerMutualAuthenticationEl>>,
}
