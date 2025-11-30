use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Ec2TrafficMirrorFilterRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    destination_cidr_block: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rule_action: PrimField<String>,
    rule_number: PrimField<f64>,
    source_cidr_block: PrimField<String>,
    traffic_direction: PrimField<String>,
    traffic_mirror_filter_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_range: Option<Vec<Ec2TrafficMirrorFilterRuleDestinationPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_range: Option<Vec<Ec2TrafficMirrorFilterRuleSourcePortRangeEl>>,
    dynamic: Ec2TrafficMirrorFilterRuleDynamic,
}

struct Ec2TrafficMirrorFilterRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TrafficMirrorFilterRuleData>,
}

#[derive(Clone)]
pub struct Ec2TrafficMirrorFilterRule(Rc<Ec2TrafficMirrorFilterRule_>);

impl Ec2TrafficMirrorFilterRule {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        self,
        v: impl Into<BlockAssignable<Ec2TrafficMirrorFilterRuleDestinationPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_port_range = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        self,
        v: impl Into<BlockAssignable<Ec2TrafficMirrorFilterRuleSourcePortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_port_range = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_cidr_block` after provisioning.\n"]
    pub fn source_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_direction` after provisioning.\n"]
    pub fn traffic_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_direction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_mirror_filter_id` after provisioning.\n"]
    pub fn traffic_mirror_filter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_mirror_filter_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> ListRef<Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_port_range", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> ListRef<Ec2TrafficMirrorFilterRuleSourcePortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_port_range", self.extract_ref()),
        )
    }
}

impl Referable for Ec2TrafficMirrorFilterRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Ec2TrafficMirrorFilterRule {}

impl ToListMappable for Ec2TrafficMirrorFilterRule {
    type O = ListRef<Ec2TrafficMirrorFilterRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2TrafficMirrorFilterRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_traffic_mirror_filter_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TrafficMirrorFilterRule {
    pub tf_id: String,
    #[doc = ""]
    pub destination_cidr_block: PrimField<String>,
    #[doc = ""]
    pub rule_action: PrimField<String>,
    #[doc = ""]
    pub rule_number: PrimField<f64>,
    #[doc = ""]
    pub source_cidr_block: PrimField<String>,
    #[doc = ""]
    pub traffic_direction: PrimField<String>,
    #[doc = ""]
    pub traffic_mirror_filter_id: PrimField<String>,
}

impl BuildEc2TrafficMirrorFilterRule {
    pub fn build(self, stack: &mut Stack) -> Ec2TrafficMirrorFilterRule {
        let out = Ec2TrafficMirrorFilterRule(Rc::new(Ec2TrafficMirrorFilterRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TrafficMirrorFilterRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                destination_cidr_block: self.destination_cidr_block,
                id: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_action: self.rule_action,
                rule_number: self.rule_number,
                source_cidr_block: self.source_cidr_block,
                traffic_direction: self.traffic_direction,
                traffic_mirror_filter_id: self.traffic_mirror_filter_id,
                destination_port_range: core::default::Default::default(),
                source_port_range: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TrafficMirrorFilterRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TrafficMirrorFilterRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Ec2TrafficMirrorFilterRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_cidr_block` after provisioning.\n"]
    pub fn source_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_direction` after provisioning.\n"]
    pub fn traffic_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_direction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_mirror_filter_id` after provisioning.\n"]
    pub fn traffic_mirror_filter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_mirror_filter_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> ListRef<Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_port_range", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> ListRef<Ec2TrafficMirrorFilterRuleSourcePortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_port_range", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Ec2TrafficMirrorFilterRuleDestinationPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2TrafficMirrorFilterRuleDestinationPortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2TrafficMirrorFilterRuleDestinationPortRangeEl {
    type O = BlockAssignable<Ec2TrafficMirrorFilterRuleDestinationPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TrafficMirrorFilterRuleDestinationPortRangeEl {}

impl BuildEc2TrafficMirrorFilterRuleDestinationPortRangeEl {
    pub fn build(self) -> Ec2TrafficMirrorFilterRuleDestinationPortRangeEl {
        Ec2TrafficMirrorFilterRuleDestinationPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef {
        Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TrafficMirrorFilterRuleDestinationPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2TrafficMirrorFilterRuleSourcePortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2TrafficMirrorFilterRuleSourcePortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2TrafficMirrorFilterRuleSourcePortRangeEl {
    type O = BlockAssignable<Ec2TrafficMirrorFilterRuleSourcePortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TrafficMirrorFilterRuleSourcePortRangeEl {}

impl BuildEc2TrafficMirrorFilterRuleSourcePortRangeEl {
    pub fn build(self) -> Ec2TrafficMirrorFilterRuleSourcePortRangeEl {
        Ec2TrafficMirrorFilterRuleSourcePortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2TrafficMirrorFilterRuleSourcePortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TrafficMirrorFilterRuleSourcePortRangeElRef {
    fn new(shared: StackShared, base: String) -> Ec2TrafficMirrorFilterRuleSourcePortRangeElRef {
        Ec2TrafficMirrorFilterRuleSourcePortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TrafficMirrorFilterRuleSourcePortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2TrafficMirrorFilterRuleDynamic {
    destination_port_range: Option<DynamicBlock<Ec2TrafficMirrorFilterRuleDestinationPortRangeEl>>,
    source_port_range: Option<DynamicBlock<Ec2TrafficMirrorFilterRuleSourcePortRangeEl>>,
}
