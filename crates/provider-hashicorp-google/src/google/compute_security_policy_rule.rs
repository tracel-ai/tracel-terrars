use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeSecurityPolicyRuleData {
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
    preview: Option<PrimField<bool>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    security_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeSecurityPolicyRuleHeaderActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeSecurityPolicyRuleMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config: Option<Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<Vec<ComputeSecurityPolicyRuleRateLimitOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_options: Option<Vec<ComputeSecurityPolicyRuleRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeSecurityPolicyRuleTimeoutsEl>,
    dynamic: ComputeSecurityPolicyRuleDynamic,
}
struct ComputeSecurityPolicyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeSecurityPolicyRuleData>,
}
#[derive(Clone)]
pub struct ComputeSecurityPolicyRule(Rc<ComputeSecurityPolicyRule_>);
impl ComputeSecurityPolicyRule {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `preview`.\nIf set to true, the specified action is not enforced."]
    pub fn set_preview(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preview = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRulePreconfiguredWafConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preconfigured_waf_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preconfigured_waf_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rate_limit_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rate_limit_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleRedirectOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeSecurityPolicyRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe name of the security policy this rule belongs to."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeSecurityPolicyRuleHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> ListRef<ComputeSecurityPolicyRuleRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyRuleTimeoutsElRef {
        ComputeSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeSecurityPolicyRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeSecurityPolicyRule {}
impl ToListMappable for ComputeSecurityPolicyRule {
    type O = ListRef<ComputeSecurityPolicyRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeSecurityPolicyRule_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_security_policy_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeSecurityPolicyRule {
    pub tf_id: String,
    #[doc = "The Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub action: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub priority: PrimField<f64>,
    #[doc = "The name of the security policy this rule belongs to."]
    pub security_policy: PrimField<String>,
}
impl BuildComputeSecurityPolicyRule {
    pub fn build(self, stack: &mut Stack) -> ComputeSecurityPolicyRule {
        let out = ComputeSecurityPolicyRule(Rc::new(ComputeSecurityPolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeSecurityPolicyRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                preview: core::default::Default::default(),
                priority: self.priority,
                project: core::default::Default::default(),
                security_policy: self.security_policy,
                header_action: core::default::Default::default(),
                match_: core::default::Default::default(),
                preconfigured_waf_config: core::default::Default::default(),
                rate_limit_options: core::default::Default::default(),
                redirect_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeSecurityPolicyRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeSecurityPolicyRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe name of the security policy this rule belongs to."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeSecurityPolicyRuleHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> ListRef<ComputeSecurityPolicyRuleRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyRuleTimeoutsElRef {
        ComputeSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    #[doc = "Set the field `header_name`.\nThe name of the header to set."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\nThe value to set the named header to."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {}
impl BuildComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
        ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
        ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to set."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value to set the named header to."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleHeaderActionElDynamic {
    request_headers_to_adds:
        Option<DynamicBlock<ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>>,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_adds:
        Option<Vec<ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>>,
    dynamic: ComputeSecurityPolicyRuleHeaderActionElDynamic,
}
impl ComputeSecurityPolicyRuleHeaderActionEl {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_adds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_adds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleHeaderActionEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleHeaderActionEl {}
impl BuildComputeSecurityPolicyRuleHeaderActionEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleHeaderActionEl {
        ComputeSecurityPolicyRuleHeaderActionEl {
            request_headers_to_adds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleHeaderActionElRef {
        ComputeSecurityPolicyRuleHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_adds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
}
impl ComputeSecurityPolicyRuleMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleMatchElConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleMatchElConfigEl {}
impl BuildComputeSecurityPolicyRuleMatchElConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleMatchElConfigEl {
        ComputeSecurityPolicyRuleMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleMatchElConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleMatchElConfigElRef {
        ComputeSecurityPolicyRuleMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleMatchElExprEl {
    expression: PrimField<String>,
}
impl ComputeSecurityPolicyRuleMatchElExprEl {}
impl ToListMappable for ComputeSecurityPolicyRuleMatchElExprEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleMatchElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: PrimField<String>,
}
impl BuildComputeSecurityPolicyRuleMatchElExprEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleMatchElExprEl {
        ComputeSecurityPolicyRuleMatchElExprEl {
            expression: self.expression,
        }
    }
}
pub struct ComputeSecurityPolicyRuleMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleMatchElExprElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleMatchElExprElRef {
        ComputeSecurityPolicyRuleMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_token_site_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_token_site_keys: Option<ListField<PrimField<String>>>,
}
impl ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
    #[doc = "Set the field `action_token_site_keys`.\nA list of site keys to be used during the validation of reCAPTCHA action-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn set_action_token_site_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.action_token_site_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `session_token_site_keys`.\nA list of site keys to be used during the validation of reCAPTCHA session-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn set_session_token_site_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.session_token_site_keys = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {}
impl BuildComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
        ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl {
            action_token_site_keys: core::default::Default::default(),
            session_token_site_keys: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef {
        ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_token_site_keys` after provisioning.\nA list of site keys to be used during the validation of reCAPTCHA action-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn action_token_site_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_token_site_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_token_site_keys` after provisioning.\nA list of site keys to be used during the validation of reCAPTCHA session-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn session_token_site_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_token_site_keys", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleMatchElExprOptionsElDynamic {
    recaptcha_options:
        Option<DynamicBlock<ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl>>,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleMatchElExprOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_options: Option<Vec<ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl>>,
    dynamic: ComputeSecurityPolicyRuleMatchElExprOptionsElDynamic,
}
impl ComputeSecurityPolicyRuleMatchElExprOptionsEl {
    #[doc = "Set the field `recaptcha_options`.\n"]
    pub fn set_recaptcha_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recaptcha_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recaptcha_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleMatchElExprOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleMatchElExprOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleMatchElExprOptionsEl {}
impl BuildComputeSecurityPolicyRuleMatchElExprOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleMatchElExprOptionsEl {
        ComputeSecurityPolicyRuleMatchElExprOptionsEl {
            recaptcha_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleMatchElExprOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleMatchElExprOptionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleMatchElExprOptionsElRef {
        ComputeSecurityPolicyRuleMatchElExprOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleMatchElExprOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recaptcha_options` after provisioning.\n"]
    pub fn recaptcha_options(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleMatchElExprOptionsElRecaptchaOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recaptcha_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleMatchElDynamic {
    config: Option<DynamicBlock<ComputeSecurityPolicyRuleMatchElConfigEl>>,
    expr: Option<DynamicBlock<ComputeSecurityPolicyRuleMatchElExprEl>>,
    expr_options: Option<DynamicBlock<ComputeSecurityPolicyRuleMatchElExprOptionsEl>>,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComputeSecurityPolicyRuleMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<ComputeSecurityPolicyRuleMatchElExprEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr_options: Option<Vec<ComputeSecurityPolicyRuleMatchElExprOptionsEl>>,
    dynamic: ComputeSecurityPolicyRuleMatchElDynamic,
}
impl ComputeSecurityPolicyRuleMatchEl {
    #[doc = "Set the field `versioned_expr`.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleMatchElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleMatchElExprEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `expr_options`.\n"]
    pub fn set_expr_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleMatchElExprOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expr_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expr_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleMatchEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleMatchEl {}
impl BuildComputeSecurityPolicyRuleMatchEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleMatchEl {
        ComputeSecurityPolicyRuleMatchEl {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            expr_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleMatchElRef {
        ComputeSecurityPolicyRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.versioned_expr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComputeSecurityPolicyRuleMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<ComputeSecurityPolicyRuleMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
    #[doc = "Get a reference to the value of field `expr_options` after provisioning.\n"]
    pub fn expr_options(&self) -> ListRef<ComputeSecurityPolicyRuleMatchElExprOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr_options", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable<
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    pub fn build(
        self,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    type O =
        BlockAssignable<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(self) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic {
    request_cookie: Option<
        DynamicBlock<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl>,
    >,
    request_header: Option<
        DynamicBlock<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl>,
    >,
    request_query_param: Option<
        DynamicBlock<
            ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
        >,
    >,
    request_uri: Option<
        DynamicBlock<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_rule_ids: Option<ListField<PrimField<String>>>,
    target_rule_set: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_cookie:
        Option<Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header:
        Option<Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_query_param: Option<
        Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_uri:
        Option<Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl>>,
    dynamic: ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[doc = "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_rule_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_cookie = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_cookie = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_query_param = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_query_param = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_uri = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_uri = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    type O = BlockAssignable<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: PrimField<String>,
}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    pub fn build(self) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
            target_rule_ids: core::default::Default::default(),
            target_rule_set: self.target_rule_set,
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn target_rule_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_rule_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\nTarget WAF rule set to apply the preconfigured WAF exclusion."]
    pub fn target_rule_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_rule_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_cookie", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRulePreconfiguredWafConfigElDynamic {
    exclusion: Option<DynamicBlock<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion: Option<Vec<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
    dynamic: ComputeSecurityPolicyRulePreconfiguredWafConfigElDynamic,
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRulePreconfiguredWafConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyRulePreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRulePreconfiguredWafConfigEl {}
impl BuildComputeSecurityPolicyRulePreconfiguredWafConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRulePreconfiguredWafConfigEl {
        ComputeSecurityPolicyRulePreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRulePreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRulePreconfiguredWafConfigElRef {
        ComputeSecurityPolicyRulePreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {}
impl BuildComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
        ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
        ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_type: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[doc = "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_type`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {}
impl BuildComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
        ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
        ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_type` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc = "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nType of the redirect action."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {}
impl BuildComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
        ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef {
        ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {}
impl BuildComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
        ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
        ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleRateLimitOptionsElDynamic {
    ban_threshold: Option<DynamicBlock<ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
    enforce_on_key_configs:
        Option<DynamicBlock<ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    exceed_redirect_options:
        Option<DynamicBlock<ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl>>,
    rate_limit_threshold:
        Option<DynamicBlock<ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>>,
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRateLimitOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_duration_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conform_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_threshold: Option<Vec<ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_configs:
        Option<Vec<ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_redirect_options:
        Option<Vec<ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold:
        Option<Vec<ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>>,
    dynamic: ComputeSecurityPolicyRuleRateLimitOptionsElDynamic,
}
impl ComputeSecurityPolicyRuleRateLimitOptionsEl {
    #[doc = "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ban_duration_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `conform_action`.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn set_conform_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conform_action = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exceed_action`.\nAction to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn set_exceed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exceed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ban_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ban_threshold = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enforce_on_key_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce_on_key_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exceed_redirect_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exceed_redirect_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rate_limit_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_threshold = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRateLimitOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRateLimitOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRateLimitOptionsEl {}
impl BuildComputeSecurityPolicyRuleRateLimitOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRateLimitOptionsEl {
        ComputeSecurityPolicyRuleRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            exceed_redirect_options: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRateLimitOptionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleRateLimitOptionsElRef {
        ComputeSecurityPolicyRuleRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRateLimitOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ban_duration_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn conform_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conform_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exceed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ban_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElExceedRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exceed_redirect_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRuleRedirectOptionsEl {
    #[doc = "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nType of the redirect action."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeSecurityPolicyRuleRedirectOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleRedirectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleRedirectOptionsEl {}
impl BuildComputeSecurityPolicyRuleRedirectOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleRedirectOptionsEl {
        ComputeSecurityPolicyRuleRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleRedirectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleRedirectOptionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleRedirectOptionsElRef {
        ComputeSecurityPolicyRuleRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeSecurityPolicyRuleTimeoutsEl {
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
impl ToListMappable for ComputeSecurityPolicyRuleTimeoutsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSecurityPolicyRuleTimeoutsEl {}
impl BuildComputeSecurityPolicyRuleTimeoutsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleTimeoutsEl {
        ComputeSecurityPolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeSecurityPolicyRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSecurityPolicyRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleTimeoutsElRef {
        ComputeSecurityPolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSecurityPolicyRuleTimeoutsElRef {
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
struct ComputeSecurityPolicyRuleDynamic {
    header_action: Option<DynamicBlock<ComputeSecurityPolicyRuleHeaderActionEl>>,
    match_: Option<DynamicBlock<ComputeSecurityPolicyRuleMatchEl>>,
    preconfigured_waf_config:
        Option<DynamicBlock<ComputeSecurityPolicyRulePreconfiguredWafConfigEl>>,
    rate_limit_options: Option<DynamicBlock<ComputeSecurityPolicyRuleRateLimitOptionsEl>>,
    redirect_options: Option<DynamicBlock<ComputeSecurityPolicyRuleRedirectOptionsEl>>,
}
