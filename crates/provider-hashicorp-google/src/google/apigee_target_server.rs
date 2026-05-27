use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeTargetServerData {
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
    env_id: PrimField<String>,
    host: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s_sl_info: Option<Vec<ApigeeTargetServerSSlInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeTargetServerTimeoutsEl>,
    dynamic: ApigeeTargetServerDynamic,
}
struct ApigeeTargetServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeTargetServerData>,
}
#[derive(Clone)]
pub struct ApigeeTargetServer(Rc<ApigeeTargetServer_>);
impl ApigeeTargetServer {
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
    #[doc = "Set the field `description`.\nA human-readable description of this TargetServer."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_enabled`.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn set_is_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `s_sl_info`.\n"]
    pub fn set_s_sl_info(self, v: impl Into<BlockAssignable<ApigeeTargetServerSSlInfoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s_sl_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s_sl_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeTargetServerTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of this TargetServer."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s_sl_info` after provisioning.\n"]
    pub fn s_sl_info(&self) -> ListRef<ApigeeTargetServerSSlInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s_sl_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeTargetServer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeTargetServer {}
impl ToListMappable for ApigeeTargetServer {
    type O = ListRef<ApigeeTargetServerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeTargetServer_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_target_server".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeTargetServer {
    pub tf_id: String,
    #[doc = "The Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub env_id: PrimField<String>,
    #[doc = "The host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub host: PrimField<String>,
    #[doc = "The resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub name: PrimField<String>,
    #[doc = "The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub port: PrimField<f64>,
}
impl BuildApigeeTargetServer {
    pub fn build(self, stack: &mut Stack) -> ApigeeTargetServer {
        let out = ApigeeTargetServer(Rc::new(ApigeeTargetServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeTargetServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                env_id: self.env_id,
                host: self.host,
                id: core::default::Default::default(),
                is_enabled: core::default::Default::default(),
                name: self.name,
                port: self.port,
                protocol: core::default::Default::default(),
                s_sl_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeTargetServerRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeTargetServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeTargetServerRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of this TargetServer."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\nEnabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource id of this reference. Values must match the regular expression [\\w\\s-.]+."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nImmutable. The protocol used by this TargetServer. Possible values: [\"HTTP\", \"HTTP2\", \"GRPC_TARGET\", \"GRPC\", \"EXTERNAL_CALLOUT\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s_sl_info` after provisioning.\n"]
    pub fn s_sl_info(&self) -> ListRef<ApigeeTargetServerSSlInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s_sl_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeTargetServerSSlInfoElCommonNameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard_match: Option<PrimField<bool>>,
}
impl ApigeeTargetServerSSlInfoElCommonNameEl {
    #[doc = "Set the field `value`.\nThe TLS Common Name string of the certificate."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `wildcard_match`.\nIndicates whether the cert should be matched against as a wildcard cert."]
    pub fn set_wildcard_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard_match = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeTargetServerSSlInfoElCommonNameEl {
    type O = BlockAssignable<ApigeeTargetServerSSlInfoElCommonNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeTargetServerSSlInfoElCommonNameEl {}
impl BuildApigeeTargetServerSSlInfoElCommonNameEl {
    pub fn build(self) -> ApigeeTargetServerSSlInfoElCommonNameEl {
        ApigeeTargetServerSSlInfoElCommonNameEl {
            value: core::default::Default::default(),
            wildcard_match: core::default::Default::default(),
        }
    }
}
pub struct ApigeeTargetServerSSlInfoElCommonNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeTargetServerSSlInfoElCommonNameElRef {
    fn new(shared: StackShared, base: String) -> ApigeeTargetServerSSlInfoElCommonNameElRef {
        ApigeeTargetServerSSlInfoElCommonNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeTargetServerSSlInfoElCommonNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe TLS Common Name string of the certificate."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `wildcard_match` after provisioning.\nIndicates whether the cert should be matched against as a wildcard cert."]
    pub fn wildcard_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wildcard_match", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApigeeTargetServerSSlInfoElDynamic {
    common_name: Option<DynamicBlock<ApigeeTargetServerSSlInfoElCommonNameEl>>,
}
#[derive(Serialize)]
pub struct ApigeeTargetServerSSlInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ciphers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_auth_enabled: Option<PrimField<bool>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_validation_errors: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocols: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<Vec<ApigeeTargetServerSSlInfoElCommonNameEl>>,
    dynamic: ApigeeTargetServerSSlInfoElDynamic,
}
impl ApigeeTargetServerSSlInfoEl {
    #[doc = "Set the field `ciphers`.\nThe SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3."]
    pub fn set_ciphers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ciphers = Some(v.into());
        self
    }
    #[doc = "Set the field `client_auth_enabled`.\nEnables two-way TLS."]
    pub fn set_client_auth_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_auth_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce`.\nIf true, TLS is strictly enforced."]
    pub fn set_enforce(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_validation_errors`.\nIf true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
    pub fn set_ignore_validation_errors(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_validation_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `key_alias`.\nRequired if clientAuthEnabled is true. The resource ID for the alias containing the private key and cert."]
    pub fn set_key_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_alias = Some(v.into());
        self
    }
    #[doc = "Set the field `key_store`.\nRequired if clientAuthEnabled is true. The resource ID of the keystore."]
    pub fn set_key_store(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_store = Some(v.into());
        self
    }
    #[doc = "Set the field `protocols`.\nThe TLS versioins to be used."]
    pub fn set_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.protocols = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_store`.\nThe resource ID of the truststore."]
    pub fn set_trust_store(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trust_store = Some(v.into());
        self
    }
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(
        mut self,
        v: impl Into<BlockAssignable<ApigeeTargetServerSSlInfoElCommonNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.common_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.common_name = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeTargetServerSSlInfoEl {
    type O = BlockAssignable<ApigeeTargetServerSSlInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeTargetServerSSlInfoEl {
    #[doc = "Enables TLS. If false, neither one-way nor two-way TLS will be enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildApigeeTargetServerSSlInfoEl {
    pub fn build(self) -> ApigeeTargetServerSSlInfoEl {
        ApigeeTargetServerSSlInfoEl {
            ciphers: core::default::Default::default(),
            client_auth_enabled: core::default::Default::default(),
            enabled: self.enabled,
            enforce: core::default::Default::default(),
            ignore_validation_errors: core::default::Default::default(),
            key_alias: core::default::Default::default(),
            key_store: core::default::Default::default(),
            protocols: core::default::Default::default(),
            trust_store: core::default::Default::default(),
            common_name: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeTargetServerSSlInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeTargetServerSSlInfoElRef {
    fn new(shared: StackShared, base: String) -> ApigeeTargetServerSSlInfoElRef {
        ApigeeTargetServerSSlInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeTargetServerSSlInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ciphers` after provisioning.\nThe SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3."]
    pub fn ciphers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }
    #[doc = "Get a reference to the value of field `client_auth_enabled` after provisioning.\nEnables two-way TLS."]
    pub fn client_auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_auth_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables TLS. If false, neither one-way nor two-way TLS will be enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `enforce` after provisioning.\nIf true, TLS is strictly enforced."]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_validation_errors` after provisioning.\nIf true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
    pub fn ignore_validation_errors(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_validation_errors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_alias` after provisioning.\nRequired if clientAuthEnabled is true. The resource ID for the alias containing the private key and cert."]
    pub fn key_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_alias", self.base))
    }
    #[doc = "Get a reference to the value of field `key_store` after provisioning.\nRequired if clientAuthEnabled is true. The resource ID of the keystore."]
    pub fn key_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_store", self.base))
    }
    #[doc = "Get a reference to the value of field `protocols` after provisioning.\nThe TLS versioins to be used."]
    pub fn protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }
    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\nThe resource ID of the truststore."]
    pub fn trust_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ListRef<ApigeeTargetServerSSlInfoElCommonNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeTargetServerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeTargetServerTimeoutsEl {
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
impl ToListMappable for ApigeeTargetServerTimeoutsEl {
    type O = BlockAssignable<ApigeeTargetServerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeTargetServerTimeoutsEl {}
impl BuildApigeeTargetServerTimeoutsEl {
    pub fn build(self) -> ApigeeTargetServerTimeoutsEl {
        ApigeeTargetServerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeTargetServerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeTargetServerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeTargetServerTimeoutsElRef {
        ApigeeTargetServerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeTargetServerTimeoutsElRef {
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
struct ApigeeTargetServerDynamic {
    s_sl_info: Option<DynamicBlock<ApigeeTargetServerSSlInfoEl>>,
}
