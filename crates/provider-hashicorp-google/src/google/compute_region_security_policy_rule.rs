use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionSecurityPolicyRuleData {
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
    region: PrimField<String>,
    security_policy: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeRegionSecurityPolicyRuleMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_match: Option<Vec<ComputeRegionSecurityPolicyRuleNetworkMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config: Option<Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<Vec<ComputeRegionSecurityPolicyRuleRateLimitOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionSecurityPolicyRuleTimeoutsEl>,
    dynamic: ComputeRegionSecurityPolicyRuleDynamic,
}
struct ComputeRegionSecurityPolicyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionSecurityPolicyRuleData>,
}
#[derive(Clone)]
pub struct ComputeRegionSecurityPolicyRule(Rc<ComputeRegionSecurityPolicyRule_>);
impl ComputeRegionSecurityPolicyRule {
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
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleMatchEl>>,
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
    #[doc = "Set the field `network_match`.\n"]
    pub fn set_network_match(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleNetworkMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_match = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl>>,
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
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionSecurityPolicyRuleTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy rule should reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe name of the security policy this rule belongs to."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeRegionSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(&self) -> ListRef<ComputeRegionSecurityPolicyRuleNetworkMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicyRuleTimeoutsElRef {
        ComputeRegionSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionSecurityPolicyRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionSecurityPolicyRule {}
impl ToListMappable for ComputeRegionSecurityPolicyRule {
    type O = ListRef<ComputeRegionSecurityPolicyRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionSecurityPolicyRule_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_security_policy_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionSecurityPolicyRule {
    pub tf_id: String,
    #[doc = "The Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub action: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub priority: PrimField<f64>,
    #[doc = "The Region in which the created Region Security Policy rule should reside."]
    pub region: PrimField<String>,
    #[doc = "The name of the security policy this rule belongs to."]
    pub security_policy: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRule {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionSecurityPolicyRule {
        let out = ComputeRegionSecurityPolicyRule(Rc::new(ComputeRegionSecurityPolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionSecurityPolicyRuleData {
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
                region: self.region,
                security_policy: self.security_policy,
                match_: core::default::Default::default(),
                network_match: core::default::Default::default(),
                preconfigured_waf_config: core::default::Default::default(),
                rate_limit_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionSecurityPolicyRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionSecurityPolicyRuleRef {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy rule should reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\nThe name of the security policy this rule belongs to."]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeRegionSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(&self) -> ListRef<ComputeRegionSecurityPolicyRuleNetworkMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicyRuleTimeoutsElRef {
        ComputeRegionSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionSecurityPolicyRuleMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRuleMatchElConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleMatchElConfigEl {}
impl BuildComputeRegionSecurityPolicyRuleMatchElConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleMatchElConfigEl {
        ComputeRegionSecurityPolicyRuleMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleMatchElConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRuleMatchElConfigElRef {
        ComputeRegionSecurityPolicyRuleMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleMatchElConfigElRef {
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
pub struct ComputeRegionSecurityPolicyRuleMatchElExprEl {
    expression: PrimField<String>,
}
impl ComputeRegionSecurityPolicyRuleMatchElExprEl {}
impl ToListMappable for ComputeRegionSecurityPolicyRuleMatchElExprEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleMatchElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRuleMatchElExprEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleMatchElExprEl {
        ComputeRegionSecurityPolicyRuleMatchElExprEl {
            expression: self.expression,
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleMatchElExprElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRuleMatchElExprElRef {
        ComputeRegionSecurityPolicyRuleMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRuleMatchElDynamic {
    config: Option<DynamicBlock<ComputeRegionSecurityPolicyRuleMatchElConfigEl>>,
    expr: Option<DynamicBlock<ComputeRegionSecurityPolicyRuleMatchElExprEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComputeRegionSecurityPolicyRuleMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<ComputeRegionSecurityPolicyRuleMatchElExprEl>>,
    dynamic: ComputeRegionSecurityPolicyRuleMatchElDynamic,
}
impl ComputeRegionSecurityPolicyRuleMatchEl {
    #[doc = "Set the field `versioned_expr`.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleMatchElConfigEl>>,
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
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleMatchElExprEl>>,
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
}
impl ToListMappable for ComputeRegionSecurityPolicyRuleMatchEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleMatchEl {}
impl BuildComputeRegionSecurityPolicyRuleMatchEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleMatchEl {
        ComputeRegionSecurityPolicyRuleMatchEl {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRuleMatchElRef {
        ComputeRegionSecurityPolicyRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleMatchElRef {
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
    pub fn config(&self) -> ListRef<ComputeRegionSecurityPolicyRuleMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<ComputeRegionSecurityPolicyRuleMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
    #[doc = "Set the field `name`.\nName of the user-defined field, as given in the definition."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {}
impl BuildComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
        ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef {
        ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the user-defined field, as given in the definition."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRuleNetworkMatchElDynamic {
    user_defined_fields:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleNetworkMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ports: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocols: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_asns: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ports: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_fields:
        Option<Vec<ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl>>,
    dynamic: ComputeRegionSecurityPolicyRuleNetworkMatchElDynamic,
}
impl ComputeRegionSecurityPolicyRuleNetworkMatchEl {
    #[doc = "Set the field `dest_ip_ranges`.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_ports`.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_dest_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocols`.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn set_ip_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_protocols = Some(v.into());
        self
    }
    #[doc = "Set the field `src_asns`.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn set_src_asns(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.src_asns = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ip_ranges`.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ports`.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_src_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `src_region_codes`.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_defined_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRuleNetworkMatchEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleNetworkMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleNetworkMatchEl {}
impl BuildComputeRegionSecurityPolicyRuleNetworkMatchEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleNetworkMatchEl {
        ComputeRegionSecurityPolicyRuleNetworkMatchEl {
            dest_ip_ranges: core::default::Default::default(),
            dest_ports: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
            src_asns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_ports: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleNetworkMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleNetworkMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRuleNetworkMatchElRef {
        ComputeRegionSecurityPolicyRuleNetworkMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleNetworkMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_ports` after provisioning.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn dest_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocols` after provisioning.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn ip_protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }
    #[doc = "Get a reference to the value of field `src_asns` after provisioning.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn src_asns(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.src_asns", self.base))
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_ports` after provisioning.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn src_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleNetworkMatchElUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef
    {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic {
    request_cookie: Option<
        DynamicBlock<
            ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
        >,
    >,
    request_header: Option<
        DynamicBlock<
            ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
        >,
    >,
    request_query_param: Option<
        DynamicBlock<
            ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
        >,
    >,
    request_uri: Option<
        DynamicBlock<
            ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_rule_ids: Option<ListField<PrimField<String>>>,
    target_rule_set: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_cookie: Option<
        Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header: Option<
        Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_query_param: Option<
        Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_uri:
        Option<Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl>>,
    dynamic: ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
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
                ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
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
                ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
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
        v : impl Into < BlockAssignable < ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl >>,
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
                ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl,
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
impl ToListMappable for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
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
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
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
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_cookie", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> ListRef<
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElDynamic {
    exclusion:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion: Option<Vec<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
    dynamic: ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElDynamic,
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>,
        >,
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
impl ToListMappable for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {}
impl BuildComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef {
        ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
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
impl ToListMappable for ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {}
impl BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef {
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
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_type: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
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
impl ToListMappable for ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    type O =
        BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {}
impl BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef {
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
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
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
impl ToListMappable for ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {}
impl BuildComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef {
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
struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElDynamic {
    ban_threshold:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
    enforce_on_key_configs: Option<
        DynamicBlock<ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>,
    >,
    rate_limit_threshold:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
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
    ban_threshold: Option<Vec<ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_configs:
        Option<Vec<ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold:
        Option<Vec<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>>,
    dynamic: ComputeRegionSecurityPolicyRuleRateLimitOptionsElDynamic,
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
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
    #[doc = "Set the field `exceed_action`.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn set_exceed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exceed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdEl>>,
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
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsEl>,
        >,
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
    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdEl>,
        >,
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
impl ToListMappable for ComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleRateLimitOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleRateLimitOptionsEl {}
impl BuildComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef {
        ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleRateLimitOptionsElRef {
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
    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exceed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ban_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleRateLimitOptionsElEnforceOnKeyConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRuleRateLimitOptionsElRateLimitThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRuleTimeoutsEl {
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
impl ToListMappable for ComputeRegionSecurityPolicyRuleTimeoutsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRuleTimeoutsEl {}
impl BuildComputeRegionSecurityPolicyRuleTimeoutsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRuleTimeoutsEl {
        ComputeRegionSecurityPolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRuleTimeoutsElRef {
        ComputeRegionSecurityPolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRuleTimeoutsElRef {
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
struct ComputeRegionSecurityPolicyRuleDynamic {
    match_: Option<DynamicBlock<ComputeRegionSecurityPolicyRuleMatchEl>>,
    network_match: Option<DynamicBlock<ComputeRegionSecurityPolicyRuleNetworkMatchEl>>,
    preconfigured_waf_config:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulePreconfiguredWafConfigEl>>,
    rate_limit_options: Option<DynamicBlock<ComputeRegionSecurityPolicyRuleRateLimitOptionsEl>>,
}
