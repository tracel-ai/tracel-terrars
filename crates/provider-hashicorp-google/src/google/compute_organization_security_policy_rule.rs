use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeOrganizationSecurityPolicyRuleData {
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
    policy_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeOrganizationSecurityPolicyRuleHeaderActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeOrganizationSecurityPolicyRuleMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config:
        Option<Vec<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_options: Option<Vec<ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeOrganizationSecurityPolicyRuleTimeoutsEl>,
    dynamic: ComputeOrganizationSecurityPolicyRuleDynamic,
}
struct ComputeOrganizationSecurityPolicyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeOrganizationSecurityPolicyRuleData>,
}
#[derive(Clone)]
pub struct ComputeOrganizationSecurityPolicyRule(Rc<ComputeOrganizationSecurityPolicyRule_>);
impl ComputeOrganizationSecurityPolicyRule {
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
    #[doc = "Set the field `description`.\nA description of the rule."]
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
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        self,
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRuleHeaderActionEl>>,
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
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchEl>>,
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
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl>>,
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
    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        self,
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl>>,
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
    pub fn set_timeouts(
        self,
        v: impl Into<ComputeOrganizationSecurityPolicyRuleTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
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
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the OrganizationSecurityPolicy this rule applies to."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
        ComputeOrganizationSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeOrganizationSecurityPolicyRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeOrganizationSecurityPolicyRule {}
impl ToListMappable for ComputeOrganizationSecurityPolicyRule {
    type O = ListRef<ComputeOrganizationSecurityPolicyRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeOrganizationSecurityPolicyRule_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_organization_security_policy_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRule {
    pub tf_id: String,
    #[doc = "The Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
    pub action: PrimField<String>,
    #[doc = "The ID of the OrganizationSecurityPolicy this rule applies to."]
    pub policy_id: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub priority: PrimField<f64>,
}
impl BuildComputeOrganizationSecurityPolicyRule {
    pub fn build(self, stack: &mut Stack) -> ComputeOrganizationSecurityPolicyRule {
        let out = ComputeOrganizationSecurityPolicyRule(Rc::new(
            ComputeOrganizationSecurityPolicyRule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ComputeOrganizationSecurityPolicyRuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    action: self.action,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    policy_id: self.policy_id,
                    preview: core::default::Default::default(),
                    priority: self.priority,
                    header_action: core::default::Default::default(),
                    match_: core::default::Default::default(),
                    preconfigured_waf_config: core::default::Default::default(),
                    redirect_options: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeOrganizationSecurityPolicyRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
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
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the OrganizationSecurityPolicy this rule applies to."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
        ComputeOrganizationSecurityPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    type O =
        BlockAssignable<ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {}
impl BuildComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
        ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
        ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef {
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
struct ComputeOrganizationSecurityPolicyRuleHeaderActionElDynamic {
    request_headers_to_adds: Option<
        DynamicBlock<ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_adds:
        Option<Vec<ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl>>,
    dynamic: ComputeOrganizationSecurityPolicyRuleHeaderActionElDynamic,
}
impl ComputeOrganizationSecurityPolicyRuleHeaderActionEl {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsEl,
            >,
        >,
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleHeaderActionEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleHeaderActionEl {}
impl BuildComputeOrganizationSecurityPolicyRuleHeaderActionEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleHeaderActionEl {
        ComputeOrganizationSecurityPolicyRuleHeaderActionEl {
            request_headers_to_adds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleHeaderActionElRef {
        ComputeOrganizationSecurityPolicyRuleHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRuleHeaderActionElRequestHeadersToAddsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_adds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
}
impl ComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleMatchElConfigEl {}
impl BuildComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
        ComputeOrganizationSecurityPolicyRuleMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef {
        ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleMatchElExprEl {
    expression: PrimField<String>,
}
impl ComputeOrganizationSecurityPolicyRuleMatchElExprEl {}
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleMatchElExprEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleMatchElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRuleMatchElExprEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleMatchElExprEl {
        ComputeOrganizationSecurityPolicyRuleMatchElExprEl {
            expression: self.expression,
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleMatchElExprElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleMatchElExprElRef {
        ComputeOrganizationSecurityPolicyRuleMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyRuleMatchElDynamic {
    config: Option<DynamicBlock<ComputeOrganizationSecurityPolicyRuleMatchElConfigEl>>,
    expr: Option<DynamicBlock<ComputeOrganizationSecurityPolicyRuleMatchElExprEl>>,
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComputeOrganizationSecurityPolicyRuleMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<ComputeOrganizationSecurityPolicyRuleMatchElExprEl>>,
    dynamic: ComputeOrganizationSecurityPolicyRuleMatchElDynamic,
}
impl ComputeOrganizationSecurityPolicyRuleMatchEl {
    #[doc = "Set the field `description`.\nA description of the rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `versioned_expr`.\nPreconfigured versioned expression. For organization security policy rules,\nthe only supported type is \"SRC_IPS_V1\".\n**NOTE** : 'FIREWALL' type is deprecated. Please use 'google_compute_firewall_policy_rule' resource instead."]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchElConfigEl>>,
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
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchElExprEl>>,
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleMatchEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleMatchEl {}
impl BuildComputeOrganizationSecurityPolicyRuleMatchEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleMatchEl {
        ComputeOrganizationSecurityPolicyRuleMatchEl {
            description: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeOrganizationSecurityPolicyRuleMatchElRef {
        ComputeOrganizationSecurityPolicyRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\nPreconfigured versioned expression. For organization security policy rules,\nthe only supported type is \"SRC_IPS_V1\".\n**NOTE** : 'FIREWALL' type is deprecated. Please use 'google_compute_firewall_policy_rule' resource instead."]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.versioned_expr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<ComputeOrganizationSecurityPolicyRuleMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef {
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
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef {
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
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl
    BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl { operator : self . operator , value : core :: default :: Default :: default () , }
    }
}
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef { fn new (shared : StackShared , base : String) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef { ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef { shared : shared , base : base . to_string () , } } }
impl
    ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
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
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl
{
    type O = BlockAssignable<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef
    {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef {
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
struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic { request_cookie : Option < DynamicBlock < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl >> , request_header : Option < DynamicBlock < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl >> , request_query_param : Option < DynamicBlock < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl >> , request_uri : Option < DynamicBlock < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl >> , }
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl { # [serde (skip_serializing_if = "Option::is_none")] target_rule_ids : Option < ListField < PrimField < String > > > , target_rule_set : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] request_cookie : Option < Vec < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_header : Option < Vec < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_query_param : Option < Vec < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_uri : Option < Vec < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl > > , dynamic : ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElDynamic , }
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[doc = "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_rule_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v : impl Into < BlockAssignable < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieEl >>,
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
        v : impl Into < BlockAssignable < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderEl >>,
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
        v : impl Into < BlockAssignable < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamEl >>,
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
        v : impl Into < BlockAssignable < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriEl >>,
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    type O =
        BlockAssignable<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl {
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
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef {
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
    ) -> ListRef<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestCookieElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_cookie", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> ListRef<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestHeaderElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]    pub fn request_query_param (& self) -> ListRef < ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestQueryParamElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRequestUriElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElDynamic {
    exclusion: Option<
        DynamicBlock<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion:
        Option<Vec<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl>>,
    dynamic: ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElDynamic,
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionEl,
            >,
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {}
impl BuildComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef {
        ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
    #[doc = "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302."]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
    #[doc = "Type of the redirect action. For organization security policies, only EXTERNAL_302 is supported."]
    pub type_: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
        ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef {
        ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action. For organization security policies, only EXTERNAL_302 is supported."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyRuleTimeoutsEl {
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
impl ToListMappable for ComputeOrganizationSecurityPolicyRuleTimeoutsEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyRuleTimeoutsEl {}
impl BuildComputeOrganizationSecurityPolicyRuleTimeoutsEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRuleTimeoutsEl {
        ComputeOrganizationSecurityPolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
        ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyRuleTimeoutsElRef {
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
struct ComputeOrganizationSecurityPolicyRuleDynamic {
    header_action: Option<DynamicBlock<ComputeOrganizationSecurityPolicyRuleHeaderActionEl>>,
    match_: Option<DynamicBlock<ComputeOrganizationSecurityPolicyRuleMatchEl>>,
    preconfigured_waf_config:
        Option<DynamicBlock<ComputeOrganizationSecurityPolicyRulePreconfiguredWafConfigEl>>,
    redirect_options: Option<DynamicBlock<ComputeOrganizationSecurityPolicyRuleRedirectOptionsEl>>,
}
