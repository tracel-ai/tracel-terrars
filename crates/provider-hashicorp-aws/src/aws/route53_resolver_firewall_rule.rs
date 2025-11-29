use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53ResolverFirewallRuleData {
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
    block_override_dns_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_override_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_override_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_response: Option<PrimField<String>>,
    firewall_domain_list_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_domain_redirection_action: Option<PrimField<String>>,
    firewall_rule_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct Route53ResolverFirewallRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53ResolverFirewallRuleData>,
}

#[derive(Clone)]
pub struct Route53ResolverFirewallRule(Rc<Route53ResolverFirewallRule_>);

impl Route53ResolverFirewallRule {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `block_override_dns_type`.\n"]
    pub fn set_block_override_dns_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().block_override_dns_type = Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_domain`.\n"]
    pub fn set_block_override_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().block_override_domain = Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_ttl`.\n"]
    pub fn set_block_override_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().block_override_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `block_response`.\n"]
    pub fn set_block_response(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().block_response = Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_domain_redirection_action`.\n"]
    pub fn set_firewall_domain_redirection_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firewall_domain_redirection_action = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `q_type`.\n"]
    pub fn set_q_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().q_type = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_dns_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_redirection_action` after provisioning.\n"]
    pub fn firewall_domain_redirection_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_redirection_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `q_type` after provisioning.\n"]
    pub fn q_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.q_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for Route53ResolverFirewallRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Route53ResolverFirewallRule { }

impl ToListMappable for Route53ResolverFirewallRule {
    type O = ListRef<Route53ResolverFirewallRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53ResolverFirewallRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53_resolver_firewall_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53ResolverFirewallRule {
    pub tf_id: String,
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub firewall_domain_list_id: PrimField<String>,
    #[doc = ""]
    pub firewall_rule_group_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub priority: PrimField<f64>,
}

impl BuildRoute53ResolverFirewallRule {
    pub fn build(self, stack: &mut Stack) -> Route53ResolverFirewallRule {
        let out = Route53ResolverFirewallRule(Rc::new(Route53ResolverFirewallRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53ResolverFirewallRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                block_override_dns_type: core::default::Default::default(),
                block_override_domain: core::default::Default::default(),
                block_override_ttl: core::default::Default::default(),
                block_response: core::default::Default::default(),
                firewall_domain_list_id: self.firewall_domain_list_id,
                firewall_domain_redirection_action: core::default::Default::default(),
                firewall_rule_group_id: self.firewall_rule_group_id,
                id: core::default::Default::default(),
                name: self.name,
                priority: self.priority,
                q_type: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53ResolverFirewallRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53ResolverFirewallRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Route53ResolverFirewallRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_dns_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_override_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_redirection_action` after provisioning.\n"]
    pub fn firewall_domain_redirection_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_redirection_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `q_type` after provisioning.\n"]
    pub fn q_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.q_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
