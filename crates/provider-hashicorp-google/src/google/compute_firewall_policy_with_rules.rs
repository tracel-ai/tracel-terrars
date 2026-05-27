use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeFirewallPolicyWithRulesData {
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
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<ComputeFirewallPolicyWithRulesRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeFirewallPolicyWithRulesTimeoutsEl>,
    dynamic: ComputeFirewallPolicyWithRulesDynamic,
}
struct ComputeFirewallPolicyWithRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeFirewallPolicyWithRulesData>,
}
#[derive(Clone)]
pub struct ComputeFirewallPolicyWithRules(Rc<ComputeFirewallPolicyWithRules_>);
impl ComputeFirewallPolicyWithRules {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyWithRulesRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeFirewallPolicyWithRulesTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of the resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of this FirewallPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of pre-define firewall policy rules."]
    pub fn predefined_rules(&self) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_tuple_count` after provisioning.\nTotal count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples."]
    pub fn rule_tuple_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_tuple_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\nA textual name of the security policy."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ComputeFirewallPolicyWithRulesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallPolicyWithRulesTimeoutsElRef {
        ComputeFirewallPolicyWithRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeFirewallPolicyWithRules {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeFirewallPolicyWithRules {}
impl ToListMappable for ComputeFirewallPolicyWithRules {
    type O = ListRef<ComputeFirewallPolicyWithRulesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeFirewallPolicyWithRules_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_firewall_policy_with_rules".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeFirewallPolicyWithRules {
    pub tf_id: String,
    #[doc = "The parent of this FirewallPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub parent: PrimField<String>,
    #[doc = "A textual name of the security policy."]
    pub short_name: PrimField<String>,
}
impl BuildComputeFirewallPolicyWithRules {
    pub fn build(self, stack: &mut Stack) -> ComputeFirewallPolicyWithRules {
        let out = ComputeFirewallPolicyWithRules(Rc::new(ComputeFirewallPolicyWithRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeFirewallPolicyWithRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                short_name: self.short_name,
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeFirewallPolicyWithRulesRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeFirewallPolicyWithRulesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of the resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of this FirewallPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of pre-define firewall policy rules."]
    pub fn predefined_rules(&self) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_tuple_count` after provisioning.\nTotal count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples."]
    pub fn rule_tuple_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_tuple_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\nA textual name of the security policy."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ComputeFirewallPolicyWithRulesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallPolicyWithRulesTimeoutsElRef {
        ComputeFirewallPolicyWithRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    #[doc = "Set the field `ip_protocol`.\n"]
    pub fn set_ip_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {}
impl BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
            ip_protocol: core::default::Default::default(),
            ports: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\n"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {}
impl BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_threat_intelligences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer4_config:
        Option<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_secure_tag:
        Option<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_threat_intelligences: Option<ListField<PrimField<String>>>,
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    #[doc = "Set the field `dest_address_groups`.\n"]
    pub fn set_dest_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_address_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_fqdns`.\n"]
    pub fn set_dest_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_fqdns = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_ip_ranges`.\n"]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_region_codes`.\n"]
    pub fn set_dest_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_threat_intelligences`.\n"]
    pub fn set_dest_threat_intelligences(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.dest_threat_intelligences = Some(v.into());
        self
    }
    #[doc = "Set the field `layer4_config`.\n"]
    pub fn set_layer4_config(
        mut self,
        v: impl Into<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl>>,
    ) -> Self {
        self.layer4_config = Some(v.into());
        self
    }
    #[doc = "Set the field `src_address_groups`.\n"]
    pub fn set_src_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_address_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `src_fqdns`.\n"]
    pub fn set_src_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_fqdns = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `src_region_codes`.\n"]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `src_secure_tag`.\n"]
    pub fn set_src_secure_tag(
        mut self,
        v: impl Into<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl>>,
    ) -> Self {
        self.src_secure_tag = Some(v.into());
        self
    }
    #[doc = "Set the field `src_threat_intelligences`.\n"]
    pub fn set_src_threat_intelligences(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.src_threat_intelligences = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {}
impl BuildComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl {
            dest_address_groups: core::default::Default::default(),
            dest_fqdns: core::default::Default::default(),
            dest_ip_ranges: core::default::Default::default(),
            dest_region_codes: core::default::Default::default(),
            dest_threat_intelligences: core::default::Default::default(),
            layer4_config: core::default::Default::default(),
            src_address_groups: core::default::Default::default(),
            src_fqdns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            src_secure_tag: core::default::Default::default(),
            src_threat_intelligences: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
        ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_address_groups` after provisioning.\n"]
    pub fn dest_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_address_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_fqdns` after provisioning.\n"]
    pub fn dest_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }
    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\n"]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_region_codes` after provisioning.\n"]
    pub fn dest_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\n"]
    pub fn dest_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_threat_intelligences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `layer4_config` after provisioning.\n"]
    pub fn layer4_config(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layer4_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_address_groups` after provisioning.\n"]
    pub fn src_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_address_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_fqdns` after provisioning.\n"]
    pub fn src_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\n"]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_secure_tag` after provisioning.\n"]
    pub fn src_secure_tag(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_secure_tag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_threat_intelligences` after provisioning.\n"]
    pub fn src_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_threat_intelligences", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {}
impl BuildComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
        ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
        ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_profile_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_secure_tag:
        Option<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspect: Option<PrimField<bool>>,
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_logging`.\n"]
    pub fn set_enable_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_name = Some(v.into());
        self
    }
    #[doc = "Set the field `security_profile_group`.\n"]
    pub fn set_security_profile_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_profile_group = Some(v.into());
        self
    }
    #[doc = "Set the field `target_resources`.\n"]
    pub fn set_target_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `target_secure_tag`.\n"]
    pub fn set_target_secure_tag(
        mut self,
        v: impl Into<ListField<ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl>>,
    ) -> Self {
        self.target_secure_tag = Some(v.into());
        self
    }
    #[doc = "Set the field `target_service_accounts`.\n"]
    pub fn set_target_service_accounts(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.target_service_accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_inspect`.\n"]
    pub fn set_tls_inspect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls_inspect = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesPredefinedRulesEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesPredefinedRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesPredefinedRulesEl {}
impl BuildComputeFirewallPolicyWithRulesPredefinedRulesEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesPredefinedRulesEl {
        ComputeFirewallPolicyWithRulesPredefinedRulesEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            match_: core::default::Default::default(),
            priority: core::default::Default::default(),
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_resources: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesPredefinedRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesPredefinedRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesPredefinedRulesElRef {
        ComputeFirewallPolicyWithRulesPredefinedRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesPredefinedRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
    #[doc = "Get a reference to the value of field `security_profile_group` after provisioning.\n"]
    pub fn security_profile_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_profile_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_resources` after provisioning.\n"]
    pub fn target_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_secure_tag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_accounts` after provisioning.\n"]
    pub fn target_service_accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_service_accounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_inspect` after provisioning.\n"]
    pub fn tls_inspect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_inspect", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
}
impl ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    #[doc = "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\nExample inputs include: [\"22\"], [\"80\",\"443\"], and\n[\"12345-12349\"]."]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    #[doc = "The IP protocol to which this rule applies. The protocol\ntype is required when creating a firewall rule.\nThis value can either be one of the following well\nknown protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),\nor the IP protocol number."]
    pub ip_protocol: PrimField<String>,
}
impl BuildComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
        ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
            ip_protocol: self.ip_protocol,
            ports: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
        ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol\ntype is required when creating a firewall rule.\nThis value can either be one of the following well\nknown protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),\nor the IP protocol number."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\nExample inputs include: [\"22\"], [\"80\",\"443\"], and\n[\"12345-12349\"]."]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    #[doc = "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {}
impl BuildComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
        ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
        ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either 'EFFECTIVE' or\n'INEFFECTIVE'. A secure tag is 'INEFFECTIVE' when it is deleted\nor its network is deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeFirewallPolicyWithRulesRuleElMatchElDynamic {
    layer4_config: Option<DynamicBlock<ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>>,
    src_secure_tag: Option<DynamicBlock<ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>>,
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesRuleElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_threat_intelligences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_threat_intelligences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer4_config: Option<Vec<ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_secure_tag: Option<Vec<ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>>,
    dynamic: ComputeFirewallPolicyWithRulesRuleElMatchElDynamic,
}
impl ComputeFirewallPolicyWithRulesRuleElMatchEl {
    #[doc = "Set the field `dest_address_groups`.\nAddress groups which should be matched against the traffic destination.\nMaximum number of destination address groups is 10."]
    pub fn set_dest_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_address_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_fqdns`.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic destination. Maximum number of destination fqdn allowed is 100."]
    pub fn set_dest_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_fqdns = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_ip_ranges`.\nDestination IP address range in CIDR format. Required for\nEGRESS rules."]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_region_codes`.\nRegion codes whose IP addresses will be used to match for destination\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of destination region codes allowed is 5000."]
    pub fn set_dest_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_threat_intelligences`.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic destination."]
    pub fn set_dest_threat_intelligences(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.dest_threat_intelligences = Some(v.into());
        self
    }
    #[doc = "Set the field `src_address_groups`.\nAddress groups which should be matched against the traffic source.\nMaximum number of source address groups is 10."]
    pub fn set_src_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_address_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `src_fqdns`.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic source. Maximum number of source fqdn allowed is 100."]
    pub fn set_src_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_fqdns = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ip_ranges`.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `src_region_codes`.\nRegion codes whose IP addresses will be used to match for source\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of source region codes allowed is 5000."]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `src_threat_intelligences`.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic source."]
    pub fn set_src_threat_intelligences(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.src_threat_intelligences = Some(v.into());
        self
    }
    #[doc = "Set the field `layer4_config`.\n"]
    pub fn set_layer4_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layer4_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layer4_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `src_secure_tag`.\n"]
    pub fn set_src_secure_tag(
        mut self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.src_secure_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.src_secure_tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesRuleElMatchEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesRuleElMatchEl {}
impl BuildComputeFirewallPolicyWithRulesRuleElMatchEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesRuleElMatchEl {
        ComputeFirewallPolicyWithRulesRuleElMatchEl {
            dest_address_groups: core::default::Default::default(),
            dest_fqdns: core::default::Default::default(),
            dest_ip_ranges: core::default::Default::default(),
            dest_region_codes: core::default::Default::default(),
            dest_threat_intelligences: core::default::Default::default(),
            src_address_groups: core::default::Default::default(),
            src_fqdns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            src_threat_intelligences: core::default::Default::default(),
            layer4_config: core::default::Default::default(),
            src_secure_tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesRuleElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRuleElMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyWithRulesRuleElMatchElRef {
        ComputeFirewallPolicyWithRulesRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesRuleElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_address_groups` after provisioning.\nAddress groups which should be matched against the traffic destination.\nMaximum number of destination address groups is 10."]
    pub fn dest_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_address_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_fqdns` after provisioning.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic destination. Maximum number of destination fqdn allowed is 100."]
    pub fn dest_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }
    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nDestination IP address range in CIDR format. Required for\nEGRESS rules."]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_region_codes` after provisioning.\nRegion codes whose IP addresses will be used to match for destination\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of destination region codes allowed is 5000."]
    pub fn dest_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic destination."]
    pub fn dest_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_threat_intelligences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_address_groups` after provisioning.\nAddress groups which should be matched against the traffic source.\nMaximum number of source address groups is 10."]
    pub fn src_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_address_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_fqdns` after provisioning.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic source. Maximum number of source fqdn allowed is 100."]
    pub fn src_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\nRegion codes whose IP addresses will be used to match for source\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of source region codes allowed is 5000."]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_threat_intelligences` after provisioning.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic source."]
    pub fn src_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_threat_intelligences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `layer4_config` after provisioning.\n"]
    pub fn layer4_config(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layer4_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_secure_tag` after provisioning.\n"]
    pub fn src_secure_tag(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_secure_tag", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    #[doc = "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {}
impl BuildComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
        ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
        ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either 'EFFECTIVE' or\n'INEFFECTIVE'. A secure tag is 'INEFFECTIVE' when it is deleted\nor its network is deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeFirewallPolicyWithRulesRuleElDynamic {
    match_: Option<DynamicBlock<ComputeFirewallPolicyWithRulesRuleElMatchEl>>,
    target_secure_tag: Option<DynamicBlock<ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl>>,
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesRuleEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_profile_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspect: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeFirewallPolicyWithRulesRuleElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_secure_tag: Option<Vec<ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl>>,
    dynamic: ComputeFirewallPolicyWithRulesRuleElDynamic,
}
impl ComputeFirewallPolicyWithRulesRuleEl {
    #[doc = "Set the field `description`.\nA description of the rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `direction`.\nThe direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nDenotes whether the firewall policy rule is disabled. When set to true,\nthe firewall policy rule is not enforced and traffic behaves as if it did\nnot exist. If this is unspecified, the firewall policy rule will be\nenabled."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_logging`.\nDenotes whether to enable logging for a particular rule.\nIf logging is enabled, logs will be exported to the\nconfigured export destination in Stackdriver."]
    pub fn set_enable_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_name`.\nAn optional name for the rule. This field is not a unique identifier\nand can be updated."]
    pub fn set_rule_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_name = Some(v.into());
        self
    }
    #[doc = "Set the field `security_profile_group`.\nA fully-qualified URL of a SecurityProfile resource instance.\nExample:\nhttps://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group\nMust be specified if action is 'apply_security_profile_group'."]
    pub fn set_security_profile_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_profile_group = Some(v.into());
        self
    }
    #[doc = "Set the field `target_resources`.\nA list of network resource URLs to which this rule applies.\nThis field allows you to control which network's VMs get\nthis rule. If this field is left blank, all VMs\nwithin the organization will receive the rule."]
    pub fn set_target_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `target_service_accounts`.\nA list of service accounts indicating the sets of\ninstances that are applied with this rule."]
    pub fn set_target_service_accounts(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.target_service_accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_inspect`.\nBoolean flag indicating if the traffic should be TLS decrypted.\nIt can be set only if action = 'apply_security_profile_group' and cannot be set for other actions."]
    pub fn set_tls_inspect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls_inspect = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyWithRulesRuleElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_secure_tag`.\n"]
    pub fn set_target_secure_tag(
        mut self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyWithRulesRuleElTargetSecureTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_secure_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_secure_tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeFirewallPolicyWithRulesRuleEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesRuleEl {
    #[doc = "The Action to perform when the client connection triggers the rule. Can currently be either\n\"allow\", \"deny\", \"apply_security_profile_group\" or \"goto_next\"."]
    pub action: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest priority."]
    pub priority: PrimField<f64>,
}
impl BuildComputeFirewallPolicyWithRulesRuleEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesRuleEl {
        ComputeFirewallPolicyWithRulesRuleEl {
            action: self.action,
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            priority: self.priority,
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_resources: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
            match_: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyWithRulesRuleElRef {
        ComputeFirewallPolicyWithRulesRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Can currently be either\n\"allow\", \"deny\", \"apply_security_profile_group\" or \"goto_next\"."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true,\nthe firewall policy rule is not enforced and traffic behaves as if it did\nnot exist. If this is unspecified, the firewall policy rule will be\nenabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule.\nIf logging is enabled, logs will be exported to the\nconfigured export destination in Stackdriver."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nAn optional name for the rule. This field is not a unique identifier\nand can be updated."]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
    #[doc = "Get a reference to the value of field `security_profile_group` after provisioning.\nA fully-qualified URL of a SecurityProfile resource instance.\nExample:\nhttps://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group\nMust be specified if action is 'apply_security_profile_group'."]
    pub fn security_profile_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_profile_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_resources` after provisioning.\nA list of network resource URLs to which this rule applies.\nThis field allows you to control which network's VMs get\nthis rule. If this field is left blank, all VMs\nwithin the organization will receive the rule."]
    pub fn target_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of\ninstances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_service_accounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_inspect` after provisioning.\nBoolean flag indicating if the traffic should be TLS decrypted.\nIt can be set only if action = 'apply_security_profile_group' and cannot be set for other actions."]
    pub fn tls_inspect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_inspect", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeFirewallPolicyWithRulesRuleElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(
        &self,
    ) -> ListRef<ComputeFirewallPolicyWithRulesRuleElTargetSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_secure_tag", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallPolicyWithRulesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeFirewallPolicyWithRulesTimeoutsEl {
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
impl ToListMappable for ComputeFirewallPolicyWithRulesTimeoutsEl {
    type O = BlockAssignable<ComputeFirewallPolicyWithRulesTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallPolicyWithRulesTimeoutsEl {}
impl BuildComputeFirewallPolicyWithRulesTimeoutsEl {
    pub fn build(self) -> ComputeFirewallPolicyWithRulesTimeoutsEl {
        ComputeFirewallPolicyWithRulesTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallPolicyWithRulesTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallPolicyWithRulesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyWithRulesTimeoutsElRef {
        ComputeFirewallPolicyWithRulesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallPolicyWithRulesTimeoutsElRef {
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
struct ComputeFirewallPolicyWithRulesDynamic {
    rule: Option<DynamicBlock<ComputeFirewallPolicyWithRulesRuleEl>>,
}
