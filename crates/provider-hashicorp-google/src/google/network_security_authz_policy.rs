use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityAuthzPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_provider: Option<Vec<NetworkSecurityAuthzPolicyCustomProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_rules: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<NetworkSecurityAuthzPolicyTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityAuthzPolicyTimeoutsEl>,
    dynamic: NetworkSecurityAuthzPolicyDynamic,
}
struct NetworkSecurityAuthzPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityAuthzPolicyData>,
}
#[derive(Clone)]
pub struct NetworkSecurityAuthzPolicy(Rc<NetworkSecurityAuthzPolicy_>);
impl NetworkSecurityAuthzPolicy {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_profile`.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn set_policy_profile(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_provider`.\n"]
    pub fn set_custom_provider(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_provider = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_rules`.\n"]
    pub fn set_http_rules(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkSecurityAuthzPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nWhen the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzPolicy resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_profile` after provisioning.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn policy_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_provider` after provisioning.\n"]
    pub fn custom_provider(&self) -> ListRef<NetworkSecurityAuthzPolicyCustomProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_rules` after provisioning.\n"]
    pub fn http_rules(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<NetworkSecurityAuthzPolicyTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityAuthzPolicyTimeoutsElRef {
        NetworkSecurityAuthzPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityAuthzPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityAuthzPolicy {}
impl ToListMappable for NetworkSecurityAuthzPolicy {
    type O = ListRef<NetworkSecurityAuthzPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityAuthzPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_authz_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityAuthzPolicy {
    pub tf_id: String,
    #[doc = "When the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub action: PrimField<String>,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "Identifier. Name of the AuthzPolicy resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkSecurityAuthzPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityAuthzPolicy {
        let out = NetworkSecurityAuthzPolicy(Rc::new(NetworkSecurityAuthzPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkSecurityAuthzPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                policy_profile: core::default::Default::default(),
                project: core::default::Default::default(),
                custom_provider: core::default::Default::default(),
                http_rules: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityAuthzPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityAuthzPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nWhen the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzPolicy resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_profile` after provisioning.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn policy_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_provider` after provisioning.\n"]
    pub fn custom_provider(&self) -> ListRef<NetworkSecurityAuthzPolicyCustomProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_rules` after provisioning.\n"]
    pub fn http_rules(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<NetworkSecurityAuthzPolicyTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityAuthzPolicyTimeoutsElRef {
        NetworkSecurityAuthzPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
    resources: ListField<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {}
impl ToListMappable for NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
    #[doc = "A list of references to authorization extensions that will be invoked for requests matching this policy. Limited to 1 custom provider."]
    pub resources: ListField<PrimField<String>>,
}
impl BuildNetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
        NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl {
            resources: self.resources,
        }
    }
}
pub struct NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef {
        NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of references to authorization extensions that will be invoked for requests matching this policy. Limited to 1 custom provider."]
    pub fn resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
    enabled: PrimField<bool>,
}
impl NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {}
impl ToListMappable for NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
    #[doc = "Enable Cloud IAP at the AuthzPolicy level."]
    pub enabled: PrimField<bool>,
}
impl BuildNetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
        NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl {
            enabled: self.enabled,
        }
    }
}
pub struct NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef {
        NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable Cloud IAP at the AuthzPolicy level."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyCustomProviderElDynamic {
    authz_extension:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl>>,
    cloud_iap: Option<DynamicBlock<NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyCustomProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authz_extension: Option<Vec<NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_iap: Option<Vec<NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl>>,
    dynamic: NetworkSecurityAuthzPolicyCustomProviderElDynamic,
}
impl NetworkSecurityAuthzPolicyCustomProviderEl {
    #[doc = "Set the field `authz_extension`.\n"]
    pub fn set_authz_extension(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authz_extension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authz_extension = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_iap`.\n"]
    pub fn set_cloud_iap(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderElCloudIapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_iap = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_iap = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyCustomProviderEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyCustomProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyCustomProviderEl {}
impl BuildNetworkSecurityAuthzPolicyCustomProviderEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyCustomProviderEl {
        NetworkSecurityAuthzPolicyCustomProviderEl {
            authz_extension: core::default::Default::default(),
            cloud_iap: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyCustomProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyCustomProviderElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyCustomProviderElRef {
        NetworkSecurityAuthzPolicyCustomProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyCustomProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authz_extension` after provisioning.\n"]
    pub fn authz_extension(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyCustomProviderElAuthzExtensionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authz_extension", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_iap` after provisioning.\n"]
    pub fn cloud_iap(&self) -> ListRef<NetworkSecurityAuthzPolicyCustomProviderElCloudIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_iap", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
    length: PrimField<f64>,
    prefix: PrimField<String>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
    #[doc = "The length of the address range."]
    pub length: PrimField<f64>,
    #[doc = "The address prefix."]
    pub prefix: PrimField<String>,
}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl {
            length: self.length,
            prefix: self.prefix,
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `length` after provisioning.\nThe length of the address range."]
    pub fn length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe address prefix."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElDynamic {
    principal: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_selector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal:
        Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `principal_selector`.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn set_principal_selector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principal = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            principal_selector: core::default::Default::default(),
            suffix: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `principal_selector` after provisioning.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn principal_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElPrincipalElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl
{}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef
    {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<ListField<PrimField<String>>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
    #[doc = "Set the field `ids`.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn set_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ids = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl {
            ids: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ids` after provisioning.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElDynamic {
    iam_service_account: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl,
        >,
    >,
    tag_value_id_set: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_service_account: Option<
        Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_value_id_set: Option<
        Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl>,
    >,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
    #[doc = "Set the field `iam_service_account`.\n"]
    pub fn set_iam_service_account(
        mut self,
        v : impl Into < BlockAssignable < NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iam_service_account = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_service_account = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_value_id_set`.\n"]
    pub fn set_tag_value_id_set(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_value_id_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_value_id_set = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl {
            iam_service_account: core::default::Default::default(),
            tag_value_id_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam_service_account` after provisioning.\n"]
    pub fn iam_service_account(
        &self,
    ) -> ListRef<
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElIamServiceAccountElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iam_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_value_id_set` after provisioning.\n"]
    pub fn tag_value_id_set(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElTagValueIdSetElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_value_id_set", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElDynamic {
    ip_blocks:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl>>,
    principals:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl>>,
    resources:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_blocks: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_blocks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElIpBlocksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElPrincipalsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
    length: PrimField<f64>,
    prefix: PrimField<String>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
    #[doc = "The length of the address range."]
    pub length: PrimField<f64>,
    #[doc = "The address prefix."]
    pub prefix: PrimField<String>,
}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl {
            length: self.length,
            prefix: self.prefix,
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `length` after provisioning.\nThe length of the address range."]
    pub fn length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe address prefix."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElDynamic {
    principal: Option<
        DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_selector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal:
        Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `principal_selector`.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn set_principal_selector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principal = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            principal_selector: core::default::Default::default(),
            suffix: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `principal_selector` after provisioning.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn principal_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElPrincipalElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl
{}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<ListField<PrimField<String>>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {
    #[doc = "Set the field `ids`.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn set_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ids = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl {
            ids: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ids` after provisioning.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElDynamic {
    iam_service_account: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl,
        >,
    >,
    tag_value_id_set: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_service_account: Option<
        Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_value_id_set:
        Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
    #[doc = "Set the field `iam_service_account`.\n"]
    pub fn set_iam_service_account(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iam_service_account = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_service_account = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_value_id_set`.\n"]
    pub fn set_tag_value_id_set(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_value_id_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_value_id_set = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl {
            iam_service_account: core::default::Default::default(),
            tag_value_id_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam_service_account` after provisioning.\n"]
    pub fn iam_service_account(
        &self,
    ) -> ListRef<
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElIamServiceAccountElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iam_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_value_id_set` after provisioning.\n"]
    pub fn tag_value_id_set(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElTagValueIdSetElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_value_id_set", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElDynamic {
    ip_blocks: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl>>,
    principals:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl>>,
    resources:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_blocks: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_blocks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElIpBlocksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElPrincipalsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElFromElDynamic {
    not_sources: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl>>,
    sources: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    not_sources: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElFromElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromEl {
    #[doc = "Set the field `not_sources`.\n"]
    pub fn set_not_sources(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElFromEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElFromEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElFromEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElFromEl {
        NetworkSecurityAuthzPolicyHttpRulesElFromEl {
            not_sources: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElFromElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyHttpRulesElFromElRef {
        NetworkSecurityAuthzPolicyHttpRulesElFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `not_sources` after provisioning.\n"]
    pub fn not_sources(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElNotSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not_sources", self.base))
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl
{}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef
    {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElDynamic {
    value: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<
        Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl>,
    >,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {
    #[doc = "Set the field `name`.\nSpecifies the name of the header in the request."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the header in the request."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> ListRef<
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElValueElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElDynamic {
    headers: Option<
        DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers:
        Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl {
            headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElDynamic {
    header_set:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl>>,
    hosts: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl>>,
    paths: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_set: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
    #[doc = "Set the field `methods`.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn set_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }
    #[doc = "Set the field `header_set`.\n"]
    pub fn set_header_set(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_set = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hosts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.paths = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.paths = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl {
            methods: core::default::Default::default(),
            header_set: core::default::Default::default(),
            hosts: core::default::Default::default(),
            paths: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `header_set` after provisioning.\n"]
    pub fn header_set(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHeaderSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_set", self.base))
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElPathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElDynamic {
    value: Option<
        DynamicBlock<
            NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<
        Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl>,
    >,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
    #[doc = "Set the field `name`.\nSpecifies the name of the header in the request."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
    type O =
        BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the header in the request."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElValueElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElDynamic {
    headers: Option<
        DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl {
            headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {
    #[doc = "Set the field `contains`.\nA substring match on the MCP method parameter name."]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nAn exact match on the MCP method parameter name."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nSpecifies that the string match should be case insensitive."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nA prefix match on the MCP method parameter name."]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nA suffix match on the MCP method parameter name."]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl
{
    type O = BlockAssignable<
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {
    pub fn build(
        self,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nA substring match on the MCP method parameter name."]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nAn exact match on the MCP method parameter name."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that the string match should be case insensitive."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nA prefix match on the MCP method parameter name."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nA suffix match on the MCP method parameter name."]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElDynamic {
    params: Option<
        DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params:
        Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.params = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
    #[doc = "The MCP method to match against. Allowed values are as follows:\n1) “tools”, “prompts”, “resources” - these will match against all sub methods under the respective methods.\n2) “prompts/list”, “tools/list”, “resources/list”, “resources/templates/list”\n3) “prompts/get”, “tools/call”, “resources/subscribe”, “resources/unsubscribe”, “resources/read”\nParams cannot be specified for categories 1) and 2)."]
    pub name: PrimField<String>,
}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl {
            name: self.name,
            params: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe MCP method to match against. Allowed values are as follows:\n1) “tools”, “prompts”, “resources” - these will match against all sub methods under the respective methods.\n2) “prompts/list”, “tools/list”, “resources/list”, “resources/templates/list”\n3) “prompts/get”, “tools/call”, “resources/subscribe”, “resources/unsubscribe”, “resources/read”\nParams cannot be specified for categories 1) and 2)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElParamsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElDynamic {
    methods:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_protocol_methods_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
    #[doc = "Set the field `base_protocol_methods_option`.\nIf specified, matches on the MCP protocol’s non-access specific methods namely: * initialize/ * completion/ * logging/ * notifications/ * ping Default value: \"SKIP_BASE_PROTOCOL_METHODS\" Possible values: [\"SKIP_BASE_PROTOCOL_METHODS\", \"MATCH_BASE_PROTOCOL_METHODS\"]"]
    pub fn set_base_protocol_methods_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_protocol_methods_option = Some(v.into());
        self
    }
    #[doc = "Set the field `methods`.\n"]
    pub fn set_methods(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.methods = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.methods = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl {
            base_protocol_methods_option: core::default::Default::default(),
            methods: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_protocol_methods_option` after provisioning.\nIf specified, matches on the MCP protocol’s non-access specific methods namely: * initialize/ * completion/ * logging/ * notifications/ * ping Default value: \"SKIP_BASE_PROTOCOL_METHODS\" Possible values: [\"SKIP_BASE_PROTOCOL_METHODS\", \"MATCH_BASE_PROTOCOL_METHODS\"]"]
    pub fn base_protocol_methods_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_protocol_methods_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\n"]
    pub fn methods(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElMethodsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
    #[doc = "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElDynamic {
    header_set:
        Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl>>,
    hosts: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl>>,
    mcp: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl>>,
    paths: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_set: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
    #[doc = "Set the field `methods`.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn set_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }
    #[doc = "Set the field `header_set`.\n"]
    pub fn set_header_set(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_set = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hosts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mcp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.paths = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.paths = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl {
            methods: core::default::Default::default(),
            header_set: core::default::Default::default(),
            hosts: core::default::Default::default(),
            mcp: core::default::Default::default(),
            paths: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `header_set` after provisioning.\n"]
    pub fn header_set(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHeaderSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_set", self.base))
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElMcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElPathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElToElDynamic {
    not_operations: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl>>,
    operations: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesElToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    not_operations: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElToElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesElToEl {
    #[doc = "Set the field `not_operations`.\n"]
    pub fn set_not_operations(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not_operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not_operations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesElToEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesElToEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesElToEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesElToEl {
        NetworkSecurityAuthzPolicyHttpRulesElToEl {
            not_operations: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElToElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyHttpRulesElToElRef {
        NetworkSecurityAuthzPolicyHttpRulesElToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `not_operations` after provisioning.\n"]
    pub fn not_operations(
        &self,
    ) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElNotOperationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.not_operations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElOperationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyHttpRulesElDynamic {
    from: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElFromEl>>,
    to: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesElToEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyHttpRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    when: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<Vec<NetworkSecurityAuthzPolicyHttpRulesElToEl>>,
    dynamic: NetworkSecurityAuthzPolicyHttpRulesElDynamic,
}
impl NetworkSecurityAuthzPolicyHttpRulesEl {
    #[doc = "Set the field `when`.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn set_when(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.when = Some(v.into());
        self
    }
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElFromEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `to`.\n"]
    pub fn set_to(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesElToEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyHttpRulesEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyHttpRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyHttpRulesEl {}
impl BuildNetworkSecurityAuthzPolicyHttpRulesEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyHttpRulesEl {
        NetworkSecurityAuthzPolicyHttpRulesEl {
            when: core::default::Default::default(),
            from: core::default::Default::default(),
            to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyHttpRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyHttpRulesElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyHttpRulesElRef {
        NetworkSecurityAuthzPolicyHttpRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyHttpRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `when` after provisioning.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn when(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.when", self.base))
    }
    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.from", self.base))
    }
    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> ListRef<NetworkSecurityAuthzPolicyHttpRulesElToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.to", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<PrimField<String>>>,
}
impl NetworkSecurityAuthzPolicyTargetEl {
    #[doc = "Set the field `load_balancing_scheme`.\nRequired when targeting forwarding rules and secure web proxy. Must not be specified when targeting Agent\nGateway. All resources referenced by this policy and extensions must share the same load balancing scheme.\nFor more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn set_load_balancing_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of references to the Forwarding Rules or Secure Web Proxy Gateways or Agent Gateways on which this\npolicy will be applied."]
    pub fn set_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityAuthzPolicyTargetEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyTargetEl {}
impl BuildNetworkSecurityAuthzPolicyTargetEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyTargetEl {
        NetworkSecurityAuthzPolicyTargetEl {
            load_balancing_scheme: core::default::Default::default(),
            resources: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyTargetElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyTargetElRef {
        NetworkSecurityAuthzPolicyTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nRequired when targeting forwarding rules and secure web proxy. Must not be specified when targeting Agent\nGateway. All resources referenced by this policy and extensions must share the same load balancing scheme.\nFor more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of references to the Forwarding Rules or Secure Web Proxy Gateways or Agent Gateways on which this\npolicy will be applied."]
    pub fn resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityAuthzPolicyTimeoutsEl {
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
impl ToListMappable for NetworkSecurityAuthzPolicyTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityAuthzPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityAuthzPolicyTimeoutsEl {}
impl BuildNetworkSecurityAuthzPolicyTimeoutsEl {
    pub fn build(self) -> NetworkSecurityAuthzPolicyTimeoutsEl {
        NetworkSecurityAuthzPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityAuthzPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityAuthzPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityAuthzPolicyTimeoutsElRef {
        NetworkSecurityAuthzPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityAuthzPolicyTimeoutsElRef {
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
struct NetworkSecurityAuthzPolicyDynamic {
    custom_provider: Option<DynamicBlock<NetworkSecurityAuthzPolicyCustomProviderEl>>,
    http_rules: Option<DynamicBlock<NetworkSecurityAuthzPolicyHttpRulesEl>>,
    target: Option<DynamicBlock<NetworkSecurityAuthzPolicyTargetEl>>,
}
