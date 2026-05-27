use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionNetworkFirewallPolicyWithRulesData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<ComputeRegionNetworkFirewallPolicyWithRulesRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl>,
    dynamic: ComputeRegionNetworkFirewallPolicyWithRulesDynamic,
}
struct ComputeRegionNetworkFirewallPolicyWithRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionNetworkFirewallPolicyWithRulesData>,
}
#[derive(Clone)]
pub struct ComputeRegionNetworkFirewallPolicyWithRules(
    Rc<ComputeRegionNetworkFirewallPolicyWithRules_>,
);
impl ComputeRegionNetworkFirewallPolicyWithRules {
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
    #[doc = "Set the field `policy_type`.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\", \"RDMA_ROCE_POLICY\", \"RDMA_FALCON_POLICY\", \"ULL_POLICY\"]"]
    pub fn set_policy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of this resource."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleEl>>,
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
    pub fn set_timeouts(
        self,
        v: impl Into<ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_firewall_policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn network_firewall_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_firewall_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\", \"RDMA_ROCE_POLICY\", \"RDMA_FALCON_POLICY\", \"ULL_POLICY\"]"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of firewall policy pre-defined rules."]
    pub fn predefined_rules(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of this resource."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionNetworkFirewallPolicyWithRules {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionNetworkFirewallPolicyWithRules {}
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRules {
    type O = ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionNetworkFirewallPolicyWithRules_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_network_firewall_policy_with_rules".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRules {
    pub tf_id: String,
    #[doc = "User-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionNetworkFirewallPolicyWithRules {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionNetworkFirewallPolicyWithRules {
        let out = ComputeRegionNetworkFirewallPolicyWithRules(Rc::new(
            ComputeRegionNetworkFirewallPolicyWithRules_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ComputeRegionNetworkFirewallPolicyWithRulesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    policy_type: core::default::Default::default(),
                    project: core::default::Default::default(),
                    region: core::default::Default::default(),
                    rule: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_firewall_policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn network_firewall_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_firewall_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\", \"RDMA_ROCE_POLICY\", \"RDMA_FALCON_POLICY\", \"ULL_POLICY\"]"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of firewall policy pre-defined rules."]
    pub fn predefined_rules(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of this resource."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
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
impl ToListMappable
    for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl
{
    type O = BlockAssignable<
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
    pub fn build(
        self,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl {
            ip_protocol: core::default::Default::default(),
            ports: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
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
impl ToListMappable
    for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl
{
    type O = BlockAssignable<
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
    pub fn build(
        self,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
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
    layer4_config: Option<
        ListField<
            ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_secure_tag: Option<
        ListField<
            ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_threat_intelligences: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
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
        v: impl Into<
            ListField<
                ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigEl,
            >,
        >,
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
        v: impl Into<
            ListField<
                ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagEl,
            >,
        >,
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
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef {
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
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElLayer4ConfigElRef>
    {
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
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElSrcSecureTagElRef>
    {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
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
impl ToListMappable
    for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl
{
    type O = BlockAssignable<
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
    pub fn build(
        self,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
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
    match_: Option<ListField<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_profile_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_secure_tag: Option<
        ListField<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspect: Option<PrimField<bool>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
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
        v: impl Into<ListField<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchEl>>,
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
    #[doc = "Set the field `target_secure_tag`.\n"]
    pub fn set_target_secure_tag(
        mut self,
        v: impl Into<
            ListField<
                ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagEl,
            >,
        >,
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
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            match_: core::default::Default::default(),
            priority: core::default::Default::default(),
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElRef {
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
    pub fn match_(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElMatchElRef> {
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
    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesPredefinedRulesElTargetSecureTagElRef>
    {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    #[doc = "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\nExample inputs include: [\"22\"], [\"80\",\"443\"], and\n[\"12345-12349\"]."]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    type O =
        BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    #[doc = "The IP protocol to which this rule applies. The protocol\ntype is required when creating a firewall rule.\nThis value can either be one of the following well\nknown protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),\nor the IP protocol number."]
    pub ip_protocol: PrimField<String>,
}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl {
            ip_protocol: self.ip_protocol,
            ports: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    #[doc = "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    type O =
        BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef {
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
struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElDynamic {
    layer4_config: Option<
        DynamicBlock<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>,
    >,
    src_secure_tag: Option<
        DynamicBlock<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
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
    layer4_config:
        Option<Vec<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_secure_tag:
        Option<Vec<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>>,
    dynamic: ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElDynamic,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
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
        v: impl Into<
            BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigEl>,
        >,
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
        v: impl Into<
            BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagEl>,
        >,
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
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl {
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
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef {
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
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElLayer4ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layer4_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_secure_tag` after provisioning.\n"]
    pub fn src_secure_tag(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElSrcSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_secure_tag", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    #[doc = "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef {
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
struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElDynamic {
    match_: Option<DynamicBlock<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl>>,
    target_secure_tag:
        Option<DynamicBlock<ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
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
    target_service_accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspect: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_secure_tag:
        Option<Vec<ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl>>,
    dynamic: ComputeRegionNetworkFirewallPolicyWithRulesRuleElDynamic,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
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
        v: impl Into<BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchEl>>,
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
        v: impl Into<
            BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagEl>,
        >,
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
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
    #[doc = "The Action to perform when the client connection triggers the rule. Can currently be either\n\"allow\", \"deny\", \"apply_security_profile_group\" or \"goto_next\"."]
    pub action: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest priority."]
    pub priority: PrimField<f64>,
}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleEl {
            action: self.action,
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            priority: self.priority,
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
            match_: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesRuleElRef {
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
    pub fn match_(&self) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(
        &self,
    ) -> ListRef<ComputeRegionNetworkFirewallPolicyWithRulesRuleElTargetSecureTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_secure_tag", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
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
impl ToListMappable for ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
    type O = BlockAssignable<ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {}
impl BuildComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
    pub fn build(self) -> ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
        ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
        ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkFirewallPolicyWithRulesTimeoutsElRef {
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
struct ComputeRegionNetworkFirewallPolicyWithRulesDynamic {
    rule: Option<DynamicBlock<ComputeRegionNetworkFirewallPolicyWithRulesRuleEl>>,
}
