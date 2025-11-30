use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Route53recoverycontrolconfigSafetyRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asserted_controls: Option<ListField<PrimField<String>>>,
    control_panel_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gating_controls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_controls: Option<ListField<PrimField<String>>>,
    wait_period_ms: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_config: Option<Vec<Route53recoverycontrolconfigSafetyRuleRuleConfigEl>>,
    dynamic: Route53recoverycontrolconfigSafetyRuleDynamic,
}

struct Route53recoverycontrolconfigSafetyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53recoverycontrolconfigSafetyRuleData>,
}

#[derive(Clone)]
pub struct Route53recoverycontrolconfigSafetyRule(Rc<Route53recoverycontrolconfigSafetyRule_>);

impl Route53recoverycontrolconfigSafetyRule {
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

    #[doc = "Set the field `asserted_controls`.\n"]
    pub fn set_asserted_controls(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asserted_controls = Some(v.into());
        self
    }

    #[doc = "Set the field `gating_controls`.\n"]
    pub fn set_gating_controls(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gating_controls = Some(v.into());
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

    #[doc = "Set the field `target_controls`.\n"]
    pub fn set_target_controls(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_controls = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_config`.\n"]
    pub fn set_rule_config(
        self,
        v: impl Into<BlockAssignable<Route53recoverycontrolconfigSafetyRuleRuleConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `asserted_controls` after provisioning.\n"]
    pub fn asserted_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asserted_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `control_panel_arn` after provisioning.\n"]
    pub fn control_panel_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_panel_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gating_controls` after provisioning.\n"]
    pub fn gating_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gating_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_controls` after provisioning.\n"]
    pub fn target_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_period_ms` after provisioning.\n"]
    pub fn wait_period_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_period_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_config` after provisioning.\n"]
    pub fn rule_config(&self) -> ListRef<Route53recoverycontrolconfigSafetyRuleRuleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_config", self.extract_ref()),
        )
    }
}

impl Referable for Route53recoverycontrolconfigSafetyRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Route53recoverycontrolconfigSafetyRule {}

impl ToListMappable for Route53recoverycontrolconfigSafetyRule {
    type O = ListRef<Route53recoverycontrolconfigSafetyRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53recoverycontrolconfigSafetyRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53recoverycontrolconfig_safety_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53recoverycontrolconfigSafetyRule {
    pub tf_id: String,
    #[doc = ""]
    pub control_panel_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub wait_period_ms: PrimField<f64>,
}

impl BuildRoute53recoverycontrolconfigSafetyRule {
    pub fn build(self, stack: &mut Stack) -> Route53recoverycontrolconfigSafetyRule {
        let out = Route53recoverycontrolconfigSafetyRule(Rc::new(
            Route53recoverycontrolconfigSafetyRule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(Route53recoverycontrolconfigSafetyRuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    asserted_controls: core::default::Default::default(),
                    control_panel_arn: self.control_panel_arn,
                    gating_controls: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    target_controls: core::default::Default::default(),
                    wait_period_ms: self.wait_period_ms,
                    rule_config: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53recoverycontrolconfigSafetyRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoverycontrolconfigSafetyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Route53recoverycontrolconfigSafetyRuleRef {
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

    #[doc = "Get a reference to the value of field `asserted_controls` after provisioning.\n"]
    pub fn asserted_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asserted_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `control_panel_arn` after provisioning.\n"]
    pub fn control_panel_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_panel_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gating_controls` after provisioning.\n"]
    pub fn gating_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gating_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_controls` after provisioning.\n"]
    pub fn target_controls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_controls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_period_ms` after provisioning.\n"]
    pub fn wait_period_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_period_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_config` after provisioning.\n"]
    pub fn rule_config(&self) -> ListRef<Route53recoverycontrolconfigSafetyRuleRuleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Route53recoverycontrolconfigSafetyRuleRuleConfigEl {
    inverted: PrimField<bool>,
    threshold: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl Route53recoverycontrolconfigSafetyRuleRuleConfigEl {}

impl ToListMappable for Route53recoverycontrolconfigSafetyRuleRuleConfigEl {
    type O = BlockAssignable<Route53recoverycontrolconfigSafetyRuleRuleConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoverycontrolconfigSafetyRuleRuleConfigEl {
    #[doc = ""]
    pub inverted: PrimField<bool>,
    #[doc = ""]
    pub threshold: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildRoute53recoverycontrolconfigSafetyRuleRuleConfigEl {
    pub fn build(self) -> Route53recoverycontrolconfigSafetyRuleRuleConfigEl {
        Route53recoverycontrolconfigSafetyRuleRuleConfigEl {
            inverted: self.inverted,
            threshold: self.threshold,
            type_: self.type_,
        }
    }
}

pub struct Route53recoverycontrolconfigSafetyRuleRuleConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoverycontrolconfigSafetyRuleRuleConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53recoverycontrolconfigSafetyRuleRuleConfigElRef {
        Route53recoverycontrolconfigSafetyRuleRuleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoverycontrolconfigSafetyRuleRuleConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inverted` after provisioning.\n"]
    pub fn inverted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inverted", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoverycontrolconfigSafetyRuleDynamic {
    rule_config: Option<DynamicBlock<Route53recoverycontrolconfigSafetyRuleRuleConfigEl>>,
}
