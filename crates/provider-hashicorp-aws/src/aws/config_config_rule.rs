use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigConfigRuleData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_execution_frequency: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<Vec<ConfigConfigRuleEvaluationModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<ConfigConfigRuleScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<ConfigConfigRuleSourceEl>>,
    dynamic: ConfigConfigRuleDynamic,
}
struct ConfigConfigRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigConfigRuleData>,
}
#[derive(Clone)]
pub struct ConfigConfigRule(Rc<ConfigConfigRule_>);
impl ConfigConfigRule {
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
    #[doc = "Set the field `input_parameters`.\n"]
    pub fn set_input_parameters(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().input_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_execution_frequency`.\n"]
    pub fn set_maximum_execution_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maximum_execution_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Set the field `evaluation_mode`.\n"]
    pub fn set_evaluation_mode(
        self,
        v: impl Into<BlockAssignable<ConfigConfigRuleEvaluationModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluation_mode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl Into<BlockAssignable<ConfigConfigRuleScopeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<ConfigConfigRuleSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_execution_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ConfigConfigRuleScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<ConfigConfigRuleSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}
impl Referable for ConfigConfigRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigConfigRule {}
impl ToListMappable for ConfigConfigRule {
    type O = ListRef<ConfigConfigRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigConfigRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_config_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigConfigRule {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildConfigConfigRule {
    pub fn build(self, stack: &mut Stack) -> ConfigConfigRule {
        let out = ConfigConfigRule(Rc::new(ConfigConfigRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigConfigRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                input_parameters: core::default::Default::default(),
                maximum_execution_frequency: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                evaluation_mode: core::default::Default::default(),
                scope: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigConfigRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigConfigRuleRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_execution_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ConfigConfigRuleScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<ConfigConfigRuleSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConfigRuleEvaluationModeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl ConfigConfigRuleEvaluationModeEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigRuleEvaluationModeEl {
    type O = BlockAssignable<ConfigConfigRuleEvaluationModeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigRuleEvaluationModeEl {}
impl BuildConfigConfigRuleEvaluationModeEl {
    pub fn build(self) -> ConfigConfigRuleEvaluationModeEl {
        ConfigConfigRuleEvaluationModeEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigRuleEvaluationModeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleEvaluationModeElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigRuleEvaluationModeElRef {
        ConfigConfigRuleEvaluationModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigRuleEvaluationModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ConfigConfigRuleScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_resource_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_value: Option<PrimField<String>>,
}
impl ConfigConfigRuleScopeEl {
    #[doc = "Set the field `compliance_resource_id`.\n"]
    pub fn set_compliance_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compliance_resource_id = Some(v.into());
        self
    }
    #[doc = "Set the field `compliance_resource_types`.\n"]
    pub fn set_compliance_resource_types(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.compliance_resource_types = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_key = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_value`.\n"]
    pub fn set_tag_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_value = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigRuleScopeEl {
    type O = BlockAssignable<ConfigConfigRuleScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigRuleScopeEl {}
impl BuildConfigConfigRuleScopeEl {
    pub fn build(self) -> ConfigConfigRuleScopeEl {
        ConfigConfigRuleScopeEl {
            compliance_resource_id: core::default::Default::default(),
            compliance_resource_types: core::default::Default::default(),
            tag_key: core::default::Default::default(),
            tag_value: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigRuleScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleScopeElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigRuleScopeElRef {
        ConfigConfigRuleScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigRuleScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compliance_resource_id` after provisioning.\n"]
    pub fn compliance_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_resource_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_resource_types` after provisioning.\n"]
    pub fn compliance_resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.compliance_resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_value` after provisioning.\n"]
    pub fn tag_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_value", self.base))
    }
}
#[derive(Serialize)]
pub struct ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_debug_log_delivery: Option<PrimField<bool>>,
    policy_runtime: PrimField<String>,
    policy_text: PrimField<String>,
}
impl ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[doc = "Set the field `enable_debug_log_delivery`.\n"]
    pub fn set_enable_debug_log_delivery(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_debug_log_delivery = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    type O = BlockAssignable<ConfigConfigRuleSourceElCustomPolicyDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[doc = ""]
    pub policy_runtime: PrimField<String>,
    #[doc = ""]
    pub policy_text: PrimField<String>,
}
impl BuildConfigConfigRuleSourceElCustomPolicyDetailsEl {
    pub fn build(self) -> ConfigConfigRuleSourceElCustomPolicyDetailsEl {
        ConfigConfigRuleSourceElCustomPolicyDetailsEl {
            enable_debug_log_delivery: core::default::Default::default(),
            policy_runtime: self.policy_runtime,
            policy_text: self.policy_text,
        }
    }
}
pub struct ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
        ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_debug_log_delivery` after provisioning.\n"]
    pub fn enable_debug_log_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_debug_log_delivery", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_runtime` after provisioning.\n"]
    pub fn policy_runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_runtime", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_text` after provisioning.\n"]
    pub fn policy_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_text", self.base))
    }
}
#[derive(Serialize)]
pub struct ConfigConfigRuleSourceElSourceDetailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_execution_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<PrimField<String>>,
}
impl ConfigConfigRuleSourceElSourceDetailEl {
    #[doc = "Set the field `event_source`.\n"]
    pub fn set_event_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_source = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_execution_frequency`.\n"]
    pub fn set_maximum_execution_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_execution_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `message_type`.\n"]
    pub fn set_message_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_type = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigRuleSourceElSourceDetailEl {
    type O = BlockAssignable<ConfigConfigRuleSourceElSourceDetailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigRuleSourceElSourceDetailEl {}
impl BuildConfigConfigRuleSourceElSourceDetailEl {
    pub fn build(self) -> ConfigConfigRuleSourceElSourceDetailEl {
        ConfigConfigRuleSourceElSourceDetailEl {
            event_source: core::default::Default::default(),
            maximum_execution_frequency: core::default::Default::default(),
            message_type: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigRuleSourceElSourceDetailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleSourceElSourceDetailElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigRuleSourceElSourceDetailElRef {
        ConfigConfigRuleSourceElSourceDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigRuleSourceElSourceDetailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_source", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_execution_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_type` after provisioning.\n"]
    pub fn message_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigRuleSourceElDynamic {
    custom_policy_details: Option<DynamicBlock<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    source_detail: Option<DynamicBlock<ConfigConfigRuleSourceElSourceDetailEl>>,
}
#[derive(Serialize)]
pub struct ConfigConfigRuleSourceEl {
    owner: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_policy_details: Option<Vec<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_detail: Option<Vec<ConfigConfigRuleSourceElSourceDetailEl>>,
    dynamic: ConfigConfigRuleSourceElDynamic,
}
impl ConfigConfigRuleSourceEl {
    #[doc = "Set the field `source_identifier`.\n"]
    pub fn set_source_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_policy_details`.\n"]
    pub fn set_custom_policy_details(
        mut self,
        v: impl Into<BlockAssignable<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_policy_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy_details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_detail`.\n"]
    pub fn set_source_detail(
        mut self,
        v: impl Into<BlockAssignable<ConfigConfigRuleSourceElSourceDetailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_detail = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_detail = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConfigConfigRuleSourceEl {
    type O = BlockAssignable<ConfigConfigRuleSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigRuleSourceEl {
    #[doc = ""]
    pub owner: PrimField<String>,
}
impl BuildConfigConfigRuleSourceEl {
    pub fn build(self) -> ConfigConfigRuleSourceEl {
        ConfigConfigRuleSourceEl {
            owner: self.owner,
            source_identifier: core::default::Default::default(),
            custom_policy_details: core::default::Default::default(),
            source_detail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConfigConfigRuleSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigRuleSourceElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigRuleSourceElRef {
        ConfigConfigRuleSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigRuleSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_policy_details` after provisioning.\n"]
    pub fn custom_policy_details(
        &self,
    ) -> ListRef<ConfigConfigRuleSourceElCustomPolicyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_policy_details", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigRuleDynamic {
    evaluation_mode: Option<DynamicBlock<ConfigConfigRuleEvaluationModeEl>>,
    scope: Option<DynamicBlock<ConfigConfigRuleScopeEl>>,
    source: Option<DynamicBlock<ConfigConfigRuleSourceEl>>,
}
