use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataRoute53ResolverFirewallRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    firewall_rule_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataRoute53ResolverFirewallRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRoute53ResolverFirewallRulesData>,
}

#[derive(Clone)]
pub struct DataRoute53ResolverFirewallRules(Rc<DataRoute53ResolverFirewallRules_>);

impl DataRoute53ResolverFirewallRules {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_rule_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rules` after provisioning.\n"]
    pub fn firewall_rules(&self) -> ListRef<DataRoute53ResolverFirewallRulesFirewallRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

impl Referable for DataRoute53ResolverFirewallRules {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataRoute53ResolverFirewallRules {}

impl ToListMappable for DataRoute53ResolverFirewallRules {
    type O = ListRef<DataRoute53ResolverFirewallRulesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRoute53ResolverFirewallRules_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route53_resolver_firewall_rules".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53ResolverFirewallRules {
    pub tf_id: String,
    #[doc = ""]
    pub firewall_rule_group_id: PrimField<String>,
}

impl BuildDataRoute53ResolverFirewallRules {
    pub fn build(self, stack: &mut Stack) -> DataRoute53ResolverFirewallRules {
        let out = DataRoute53ResolverFirewallRules(Rc::new(DataRoute53ResolverFirewallRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRoute53ResolverFirewallRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                action: core::default::Default::default(),
                firewall_rule_group_id: self.firewall_rule_group_id,
                id: core::default::Default::default(),
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53ResolverFirewallRulesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53ResolverFirewallRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataRoute53ResolverFirewallRulesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_rule_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rules` after provisioning.\n"]
    pub fn firewall_rules(&self) -> ListRef<DataRoute53ResolverFirewallRulesFirewallRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataRoute53ResolverFirewallRulesFirewallRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_override_dns_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_override_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_override_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_response: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator_request_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_domain_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_rule_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modification_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
}

impl DataRoute53ResolverFirewallRulesFirewallRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_dns_type`.\n"]
    pub fn set_block_override_dns_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.block_override_dns_type = Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_domain`.\n"]
    pub fn set_block_override_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.block_override_domain = Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_ttl`.\n"]
    pub fn set_block_override_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_override_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `block_response`.\n"]
    pub fn set_block_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.block_response = Some(v.into());
        self
    }

    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_time = Some(v.into());
        self
    }

    #[doc = "Set the field `creator_request_id`.\n"]
    pub fn set_creator_request_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creator_request_id = Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_domain_list_id`.\n"]
    pub fn set_firewall_domain_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.firewall_domain_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_rule_group_id`.\n"]
    pub fn set_firewall_rule_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.firewall_rule_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `modification_time`.\n"]
    pub fn set_modification_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.modification_time = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
}

impl ToListMappable for DataRoute53ResolverFirewallRulesFirewallRulesEl {
    type O = BlockAssignable<DataRoute53ResolverFirewallRulesFirewallRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53ResolverFirewallRulesFirewallRulesEl {}

impl BuildDataRoute53ResolverFirewallRulesFirewallRulesEl {
    pub fn build(self) -> DataRoute53ResolverFirewallRulesFirewallRulesEl {
        DataRoute53ResolverFirewallRulesFirewallRulesEl {
            action: core::default::Default::default(),
            block_override_dns_type: core::default::Default::default(),
            block_override_domain: core::default::Default::default(),
            block_override_ttl: core::default::Default::default(),
            block_response: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            creator_request_id: core::default::Default::default(),
            firewall_domain_list_id: core::default::Default::default(),
            firewall_rule_group_id: core::default::Default::default(),
            modification_time: core::default::Default::default(),
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53ResolverFirewallRulesFirewallRulesElRef {
        DataRoute53ResolverFirewallRulesFirewallRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_override_dns_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_override_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_override_ttl", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_response", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator_request_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_domain_list_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_rule_group_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modification_time", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
