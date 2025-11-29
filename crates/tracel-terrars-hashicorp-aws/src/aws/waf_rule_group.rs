use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafRuleGroupData {
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
    metric_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activated_rule: Option<Vec<WafRuleGroupActivatedRuleEl>>,
    dynamic: WafRuleGroupDynamic,
}

struct WafRuleGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafRuleGroupData>,
}

#[derive(Clone)]
pub struct WafRuleGroup(Rc<WafRuleGroup_>);

impl WafRuleGroup {
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

    #[doc = "Set the field `activated_rule`.\n"]
    pub fn set_activated_rule(self, v: impl Into<BlockAssignable<WafRuleGroupActivatedRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().activated_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.activated_rule = Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Referable for WafRuleGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WafRuleGroup { }

impl ToListMappable for WafRuleGroup {
    type O = ListRef<WafRuleGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WafRuleGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_waf_rule_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafRuleGroup {
    pub tf_id: String,
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildWafRuleGroup {
    pub fn build(self, stack: &mut Stack) -> WafRuleGroup {
        let out = WafRuleGroup(Rc::new(WafRuleGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafRuleGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                metric_name: self.metric_name,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                activated_rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafRuleGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafRuleGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl WafRuleGroupRef {
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

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafRuleGroupActivatedRuleElActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafRuleGroupActivatedRuleElActionEl { }

impl ToListMappable for WafRuleGroupActivatedRuleElActionEl {
    type O = BlockAssignable<WafRuleGroupActivatedRuleElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafRuleGroupActivatedRuleElActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafRuleGroupActivatedRuleElActionEl {
    pub fn build(self) -> WafRuleGroupActivatedRuleElActionEl {
        WafRuleGroupActivatedRuleElActionEl { type_: self.type_ }
    }
}

pub struct WafRuleGroupActivatedRuleElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafRuleGroupActivatedRuleElActionElRef {
    fn new(shared: StackShared, base: String) -> WafRuleGroupActivatedRuleElActionElRef {
        WafRuleGroupActivatedRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafRuleGroupActivatedRuleElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafRuleGroupActivatedRuleElDynamic {
    action: Option<DynamicBlock<WafRuleGroupActivatedRuleElActionEl>>,
}

#[derive(Serialize)]
pub struct WafRuleGroupActivatedRuleEl {
    priority: PrimField<f64>,
    rule_id: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<WafRuleGroupActivatedRuleElActionEl>>,
    dynamic: WafRuleGroupActivatedRuleElDynamic,
}

impl WafRuleGroupActivatedRuleEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<WafRuleGroupActivatedRuleElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafRuleGroupActivatedRuleEl {
    type O = BlockAssignable<WafRuleGroupActivatedRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafRuleGroupActivatedRuleEl {
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub rule_id: PrimField<String>,
}

impl BuildWafRuleGroupActivatedRuleEl {
    pub fn build(self) -> WafRuleGroupActivatedRuleEl {
        WafRuleGroupActivatedRuleEl {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafRuleGroupActivatedRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafRuleGroupActivatedRuleElRef {
    fn new(shared: StackShared, base: String) -> WafRuleGroupActivatedRuleElRef {
        WafRuleGroupActivatedRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafRuleGroupActivatedRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<WafRuleGroupActivatedRuleElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafRuleGroupDynamic {
    activated_rule: Option<DynamicBlock<WafRuleGroupActivatedRuleEl>>,
}
