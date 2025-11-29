use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbSslNegotiationPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    lb_port: PrimField<f64>,
    load_balancer: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<Vec<LbSslNegotiationPolicyAttributeEl>>,
    dynamic: LbSslNegotiationPolicyDynamic,
}

struct LbSslNegotiationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LbSslNegotiationPolicyData>,
}

#[derive(Clone)]
pub struct LbSslNegotiationPolicy(Rc<LbSslNegotiationPolicy_>);

impl LbSslNegotiationPolicy {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().triggers = Some(v.into());
        self
    }

    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(self, v: impl Into<BlockAssignable<LbSslNegotiationPolicyAttributeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attribute = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attribute = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lb_port` after provisioning.\n"]
    pub fn lb_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lb_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

impl Referable for LbSslNegotiationPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LbSslNegotiationPolicy { }

impl ToListMappable for LbSslNegotiationPolicy {
    type O = ListRef<LbSslNegotiationPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LbSslNegotiationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_lb_ssl_negotiation_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbSslNegotiationPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub lb_port: PrimField<f64>,
    #[doc = ""]
    pub load_balancer: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildLbSslNegotiationPolicy {
    pub fn build(self, stack: &mut Stack) -> LbSslNegotiationPolicy {
        let out = LbSslNegotiationPolicy(Rc::new(LbSslNegotiationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LbSslNegotiationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                lb_port: self.lb_port,
                load_balancer: self.load_balancer,
                name: self.name,
                region: core::default::Default::default(),
                triggers: core::default::Default::default(),
                attribute: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbSslNegotiationPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbSslNegotiationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LbSslNegotiationPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lb_port` after provisioning.\n"]
    pub fn lb_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lb_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbSslNegotiationPolicyAttributeEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl LbSslNegotiationPolicyAttributeEl { }

impl ToListMappable for LbSslNegotiationPolicyAttributeEl {
    type O = BlockAssignable<LbSslNegotiationPolicyAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbSslNegotiationPolicyAttributeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildLbSslNegotiationPolicyAttributeEl {
    pub fn build(self) -> LbSslNegotiationPolicyAttributeEl {
        LbSslNegotiationPolicyAttributeEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct LbSslNegotiationPolicyAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbSslNegotiationPolicyAttributeElRef {
    fn new(shared: StackShared, base: String) -> LbSslNegotiationPolicyAttributeElRef {
        LbSslNegotiationPolicyAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbSslNegotiationPolicyAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbSslNegotiationPolicyDynamic {
    attribute: Option<DynamicBlock<LbSslNegotiationPolicyAttributeEl>>,
}
