use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpclatticeListenerRuleData {
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
    listener_identifier: PrimField<String>,
    name: PrimField<String>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<VpclatticeListenerRuleActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<VpclatticeListenerRuleMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpclatticeListenerRuleTimeoutsEl>,
    dynamic: VpclatticeListenerRuleDynamic,
}

struct VpclatticeListenerRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpclatticeListenerRuleData>,
}

#[derive(Clone)]
pub struct VpclatticeListenerRule(Rc<VpclatticeListenerRule_>);

impl VpclatticeListenerRule {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<VpclatticeListenerRuleActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpclatticeListenerRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<VpclatticeListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<VpclatticeListenerRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerRuleTimeoutsElRef {
        VpclatticeListenerRuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VpclatticeListenerRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpclatticeListenerRule { }

impl ToListMappable for VpclatticeListenerRule {
    type O = ListRef<VpclatticeListenerRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpclatticeListenerRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpclattice_listener_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpclatticeListenerRule {
    pub tf_id: String,
    #[doc = ""]
    pub listener_identifier: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub service_identifier: PrimField<String>,
}

impl BuildVpclatticeListenerRule {
    pub fn build(self, stack: &mut Stack) -> VpclatticeListenerRule {
        let out = VpclatticeListenerRule(Rc::new(VpclatticeListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpclatticeListenerRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                listener_identifier: self.listener_identifier,
                name: self.name,
                priority: self.priority,
                region: core::default::Default::default(),
                service_identifier: self.service_identifier,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                match_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpclatticeListenerRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpclatticeListenerRuleRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<VpclatticeListenerRuleActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<VpclatticeListenerRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerRuleTimeoutsElRef {
        VpclatticeListenerRuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleActionElFixedResponseEl {
    status_code: PrimField<f64>,
}

impl VpclatticeListenerRuleActionElFixedResponseEl { }

impl ToListMappable for VpclatticeListenerRuleActionElFixedResponseEl {
    type O = BlockAssignable<VpclatticeListenerRuleActionElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleActionElFixedResponseEl {
    #[doc = ""]
    pub status_code: PrimField<f64>,
}

impl BuildVpclatticeListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> VpclatticeListenerRuleActionElFixedResponseEl {
        VpclatticeListenerRuleActionElFixedResponseEl { status_code: self.status_code }
    }
}

pub struct VpclatticeListenerRuleActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleActionElFixedResponseElRef {
        VpclatticeListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleActionElForwardElTargetGroupsEl {
    target_group_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl VpclatticeListenerRuleActionElForwardElTargetGroupsEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleActionElForwardElTargetGroupsEl {
    type O = BlockAssignable<VpclatticeListenerRuleActionElForwardElTargetGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleActionElForwardElTargetGroupsEl {
    #[doc = ""]
    pub target_group_identifier: PrimField<String>,
}

impl BuildVpclatticeListenerRuleActionElForwardElTargetGroupsEl {
    pub fn build(self) -> VpclatticeListenerRuleActionElForwardElTargetGroupsEl {
        VpclatticeListenerRuleActionElForwardElTargetGroupsEl {
            target_group_identifier: self.target_group_identifier,
            weight: core::default::Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleActionElForwardElTargetGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleActionElForwardElTargetGroupsElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleActionElForwardElTargetGroupsElRef {
        VpclatticeListenerRuleActionElForwardElTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleActionElForwardElTargetGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_group_identifier` after provisioning.\n"]
    pub fn target_group_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_group_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleActionElForwardElDynamic {
    target_groups: Option<DynamicBlock<VpclatticeListenerRuleActionElForwardElTargetGroupsEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_groups: Option<Vec<VpclatticeListenerRuleActionElForwardElTargetGroupsEl>>,
    dynamic: VpclatticeListenerRuleActionElForwardElDynamic,
}

impl VpclatticeListenerRuleActionElForwardEl {
    #[doc = "Set the field `target_groups`.\n"]
    pub fn set_target_groups(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleActionElForwardElTargetGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_groups = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_groups = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleActionElForwardEl {
    type O = BlockAssignable<VpclatticeListenerRuleActionElForwardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleActionElForwardEl {}

impl BuildVpclatticeListenerRuleActionElForwardEl {
    pub fn build(self) -> VpclatticeListenerRuleActionElForwardEl {
        VpclatticeListenerRuleActionElForwardEl {
            target_groups: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleActionElForwardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleActionElForwardElRef {
        VpclatticeListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_groups` after provisioning.\n"]
    pub fn target_groups(&self) -> ListRef<VpclatticeListenerRuleActionElForwardElTargetGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_groups", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleActionElDynamic {
    fixed_response: Option<DynamicBlock<VpclatticeListenerRuleActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<VpclatticeListenerRuleActionElForwardEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<VpclatticeListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<VpclatticeListenerRuleActionElForwardEl>>,
    dynamic: VpclatticeListenerRuleActionElDynamic,
}

impl VpclatticeListenerRuleActionEl {
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(mut self, v: impl Into<BlockAssignable<VpclatticeListenerRuleActionElForwardEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleActionEl {
    type O = BlockAssignable<VpclatticeListenerRuleActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleActionEl {}

impl BuildVpclatticeListenerRuleActionEl {
    pub fn build(self) -> VpclatticeListenerRuleActionEl {
        VpclatticeListenerRuleActionEl {
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleActionElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleActionElRef {
        VpclatticeListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<VpclatticeListenerRuleActionElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<VpclatticeListenerRuleActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
    #[doc = "Set the field `contains`.\n"]
    pub fn set_contains(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains = Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {}

impl BuildVpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
        VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef {
        VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contains` after provisioning.\n"]
    pub fn contains(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElDynamic {
    match_: Option<DynamicBlock<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl>>,
    dynamic: VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElDynamic,
}

impl VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.case_sensitive = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildVpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
        VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl {
            case_sensitive: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef {
        VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {}

impl BuildVpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
        VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef {
        VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleMatchElHttpMatchElPathMatchElDynamic {
    match_: Option<DynamicBlock<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl>>,
    dynamic: VpclatticeListenerRuleMatchElHttpMatchElPathMatchElDynamic,
}

impl VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.case_sensitive = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {}

impl BuildVpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
        VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl {
            case_sensitive: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef {
        VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleMatchElHttpMatchElDynamic {
    header_matches: Option<DynamicBlock<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl>>,
    path_match: Option<DynamicBlock<VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchElHttpMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_matches: Option<Vec<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_match: Option<Vec<VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl>>,
    dynamic: VpclatticeListenerRuleMatchElHttpMatchElDynamic,
}

impl VpclatticeListenerRuleMatchElHttpMatchEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc = "Set the field `header_matches`.\n"]
    pub fn set_header_matches(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_matches = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_matches = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_match`.\n"]
    pub fn set_path_match(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchElPathMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchElHttpMatchEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchElHttpMatchEl {}

impl BuildVpclatticeListenerRuleMatchElHttpMatchEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchElHttpMatchEl {
        VpclatticeListenerRuleMatchElHttpMatchEl {
            method: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            path_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElHttpMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElHttpMatchElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElHttpMatchElRef {
        VpclatticeListenerRuleMatchElHttpMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElHttpMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `header_matches` after provisioning.\n"]
    pub fn header_matches(&self) -> ListRef<VpclatticeListenerRuleMatchElHttpMatchElHeaderMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc = "Get a reference to the value of field `path_match` after provisioning.\n"]
    pub fn path_match(&self) -> ListRef<VpclatticeListenerRuleMatchElHttpMatchElPathMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleMatchElDynamic {
    http_match: Option<DynamicBlock<VpclatticeListenerRuleMatchElHttpMatchEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_match: Option<Vec<VpclatticeListenerRuleMatchElHttpMatchEl>>,
    dynamic: VpclatticeListenerRuleMatchElDynamic,
}

impl VpclatticeListenerRuleMatchEl {
    #[doc = "Set the field `http_match`.\n"]
    pub fn set_http_match(mut self, v: impl Into<BlockAssignable<VpclatticeListenerRuleMatchElHttpMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeListenerRuleMatchEl {
    type O = BlockAssignable<VpclatticeListenerRuleMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleMatchEl {}

impl BuildVpclatticeListenerRuleMatchEl {
    pub fn build(self) -> VpclatticeListenerRuleMatchEl {
        VpclatticeListenerRuleMatchEl {
            http_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleMatchElRef {
        VpclatticeListenerRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_match` after provisioning.\n"]
    pub fn http_match(&self) -> ListRef<VpclatticeListenerRuleMatchElHttpMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_match", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VpclatticeListenerRuleTimeoutsEl {
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

impl ToListMappable for VpclatticeListenerRuleTimeoutsEl {
    type O = BlockAssignable<VpclatticeListenerRuleTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRuleTimeoutsEl {}

impl BuildVpclatticeListenerRuleTimeoutsEl {
    pub fn build(self) -> VpclatticeListenerRuleTimeoutsEl {
        VpclatticeListenerRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpclatticeListenerRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeListenerRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerRuleTimeoutsElRef {
        VpclatticeListenerRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRuleTimeoutsElRef {
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
struct VpclatticeListenerRuleDynamic {
    action: Option<DynamicBlock<VpclatticeListenerRuleActionEl>>,
    match_: Option<DynamicBlock<VpclatticeListenerRuleMatchEl>>,
}
