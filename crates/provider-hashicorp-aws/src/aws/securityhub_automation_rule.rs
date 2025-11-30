use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SecurityhubAutomationRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_terminal: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rule_name: PrimField<String>,
    rule_order: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<SecurityhubAutomationRuleActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criteria: Option<Vec<SecurityhubAutomationRuleCriteriaEl>>,
    dynamic: SecurityhubAutomationRuleDynamic,
}

struct SecurityhubAutomationRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityhubAutomationRuleData>,
}

#[derive(Clone)]
pub struct SecurityhubAutomationRule(Rc<SecurityhubAutomationRule_>);

impl SecurityhubAutomationRule {
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

    #[doc = "Set the field `is_terminal`.\n"]
    pub fn set_is_terminal(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_terminal = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_status`.\n"]
    pub fn set_rule_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rule_status = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `criteria`.\n"]
    pub fn set_criteria(
        self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.criteria = Some(d);
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

    #[doc = "Get a reference to the value of field `is_terminal` after provisioning.\n"]
    pub fn is_terminal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_terminal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_order", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_status` after provisioning.\n"]
    pub fn rule_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> ListRef<SecurityhubAutomationRuleCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.criteria", self.extract_ref()),
        )
    }
}

impl Referable for SecurityhubAutomationRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SecurityhubAutomationRule {}

impl ToListMappable for SecurityhubAutomationRule {
    type O = ListRef<SecurityhubAutomationRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecurityhubAutomationRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_securityhub_automation_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubAutomationRule {
    pub tf_id: String,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub rule_name: PrimField<String>,
    #[doc = ""]
    pub rule_order: PrimField<f64>,
}

impl BuildSecurityhubAutomationRule {
    pub fn build(self, stack: &mut Stack) -> SecurityhubAutomationRule {
        let out = SecurityhubAutomationRule(Rc::new(SecurityhubAutomationRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityhubAutomationRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                is_terminal: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                rule_order: self.rule_order,
                rule_status: core::default::Default::default(),
                tags: core::default::Default::default(),
                actions: core::default::Default::default(),
                criteria: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubAutomationRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SecurityhubAutomationRuleRef {
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

    #[doc = "Get a reference to the value of field `is_terminal` after provisioning.\n"]
    pub fn is_terminal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_terminal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_order", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_status` after provisioning.\n"]
    pub fn rule_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> ListRef<SecurityhubAutomationRuleCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.criteria", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
    text: PrimField<String>,
    updated_by: PrimField<String>,
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {}

impl ToListMappable for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
    type O = BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub updated_by: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl {
            text: self.text,
            updated_by: self.updated_by,
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_by` after provisioning.\n"]
    pub fn updated_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_by", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
    id: PrimField<String>,
    product_arn: PrimField<String>,
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {}

impl ToListMappable for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
    type O =
        BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub product_arn: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl {
            id: self.id,
            product_arn: self.product_arn,
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsElRef {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `product_arn` after provisioning.\n"]
    pub fn product_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<PrimField<f64>>,
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }

    #[doc = "Set the field `product`.\n"]
    pub fn set_product(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.product = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
    type O = BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {}

impl BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl {
            label: core::default::Default::default(),
            product: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `product` after provisioning.\n"]
    pub fn product(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
    type O = BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {}

impl BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl {
            status: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElDynamic {
    note: Option<DynamicBlock<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl>>,
    related_findings: Option<
        DynamicBlock<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl>,
    >,
    severity:
        Option<DynamicBlock<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl>>,
    workflow:
        Option<DynamicBlock<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_fields: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_findings:
        Option<Vec<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<Vec<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<Vec<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl>>,
    dynamic: SecurityhubAutomationRuleActionsElFindingFieldsUpdateElDynamic,
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.confidence = Some(v.into());
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.criticality = Some(v.into());
        self
    }

    #[doc = "Set the field `types`.\n"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.user_defined_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verification_state = Some(v.into());
        self
    }

    #[doc = "Set the field `note`.\n"]
    pub fn set_note(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `related_findings`.\n"]
    pub fn set_related_findings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRelatedFindingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_findings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.severity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.severity = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workflow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
    type O = BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {}

impl BuildSecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl {
            confidence: core::default::Default::default(),
            criticality: core::default::Default::default(),
            types: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            note: core::default::Default::default(),
            related_findings: core::default::Default::default(),
            severity: core::default::Default::default(),
            workflow: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef {
        SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidence", self.base))
    }

    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.criticality", self.base))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `verification_state` after provisioning.\n"]
    pub fn verification_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_state", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `note` after provisioning.\n"]
    pub fn note(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElNoteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.note", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElSeverityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElWorkflowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workflow", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleActionsElDynamic {
    finding_fields_update:
        Option<DynamicBlock<SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleActionsEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_fields_update: Option<Vec<SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl>>,
    dynamic: SecurityhubAutomationRuleActionsElDynamic,
}

impl SecurityhubAutomationRuleActionsEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `finding_fields_update`.\n"]
    pub fn set_finding_fields_update(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleActionsElFindingFieldsUpdateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_fields_update = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_fields_update = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleActionsEl {
    type O = BlockAssignable<SecurityhubAutomationRuleActionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleActionsEl {}

impl BuildSecurityhubAutomationRuleActionsEl {
    pub fn build(self) -> SecurityhubAutomationRuleActionsEl {
        SecurityhubAutomationRuleActionsEl {
            type_: core::default::Default::default(),
            finding_fields_update: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleActionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleActionsElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleActionsElRef {
        SecurityhubAutomationRuleActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `finding_fields_update` after provisioning.\n"]
    pub fn finding_fields_update(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleActionsElFindingFieldsUpdateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.finding_fields_update", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElAwsAccountIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElAwsAccountIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
        SecurityhubAutomationRuleCriteriaElAwsAccountIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElAwsAccountIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElAwsAccountIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElAwsAccountIdElRef {
        SecurityhubAutomationRuleCriteriaElAwsAccountIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElAwsAccountIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElAwsAccountNameEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElAwsAccountNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
        SecurityhubAutomationRuleCriteriaElAwsAccountNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElAwsAccountNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElAwsAccountNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElAwsAccountNameElRef {
        SecurityhubAutomationRuleCriteriaElAwsAccountNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElAwsAccountNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElCompanyNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElCompanyNameEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElCompanyNameEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElCompanyNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElCompanyNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElCompanyNameEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElCompanyNameEl {
        SecurityhubAutomationRuleCriteriaElCompanyNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElCompanyNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElCompanyNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElCompanyNameElRef {
        SecurityhubAutomationRuleCriteriaElCompanyNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElCompanyNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
        SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdElRef {
        SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
        SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdElRef {
        SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElComplianceStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElComplianceStatusEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElComplianceStatusEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElComplianceStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElComplianceStatusEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElComplianceStatusEl {
        SecurityhubAutomationRuleCriteriaElComplianceStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElComplianceStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElComplianceStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElComplianceStatusElRef {
        SecurityhubAutomationRuleCriteriaElComplianceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElComplianceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElConfidenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<f64>>,
}

impl SecurityhubAutomationRuleCriteriaElConfidenceEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.eq = Some(v.into());
        self
    }

    #[doc = "Set the field `gt`.\n"]
    pub fn set_gt(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gt = Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gte = Some(v.into());
        self
    }

    #[doc = "Set the field `lt`.\n"]
    pub fn set_lt(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lt = Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElConfidenceEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElConfidenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElConfidenceEl {}

impl BuildSecurityhubAutomationRuleCriteriaElConfidenceEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElConfidenceEl {
        SecurityhubAutomationRuleCriteriaElConfidenceEl {
            eq: core::default::Default::default(),
            gt: core::default::Default::default(),
            gte: core::default::Default::default(),
            lt: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElConfidenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElConfidenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElConfidenceElRef {
        SecurityhubAutomationRuleCriteriaElConfidenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElConfidenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gt` after provisioning.\n"]
    pub fn gt(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gt", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lt` after provisioning.\n"]
    pub fn lt(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lt", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
        SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef {
        SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElCreatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElCreatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElCreatedAtElDynamic,
}

impl SecurityhubAutomationRuleCriteriaElCreatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElCreatedAtEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElCreatedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElCreatedAtEl {}

impl BuildSecurityhubAutomationRuleCriteriaElCreatedAtEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElCreatedAtEl {
        SecurityhubAutomationRuleCriteriaElCreatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElCreatedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElCreatedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElCreatedAtElRef {
        SecurityhubAutomationRuleCriteriaElCreatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElCreatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleCriteriaElCreatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElCriticalityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<f64>>,
}

impl SecurityhubAutomationRuleCriteriaElCriticalityEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.eq = Some(v.into());
        self
    }

    #[doc = "Set the field `gt`.\n"]
    pub fn set_gt(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gt = Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gte = Some(v.into());
        self
    }

    #[doc = "Set the field `lt`.\n"]
    pub fn set_lt(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lt = Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElCriticalityEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElCriticalityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElCriticalityEl {}

impl BuildSecurityhubAutomationRuleCriteriaElCriticalityEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElCriticalityEl {
        SecurityhubAutomationRuleCriteriaElCriticalityEl {
            eq: core::default::Default::default(),
            gt: core::default::Default::default(),
            gte: core::default::Default::default(),
            lt: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElCriticalityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElCriticalityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElCriticalityElRef {
        SecurityhubAutomationRuleCriteriaElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gt` after provisioning.\n"]
    pub fn gt(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gt", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lt` after provisioning.\n"]
    pub fn lt(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lt", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElDescriptionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElDescriptionEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElDescriptionEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElDescriptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElDescriptionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElDescriptionEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElDescriptionEl {
        SecurityhubAutomationRuleCriteriaElDescriptionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElDescriptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElDescriptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElDescriptionElRef {
        SecurityhubAutomationRuleCriteriaElDescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElDescriptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
        SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef {
        SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElFirstObservedAtElDynamic {
    date_range:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElFirstObservedAtElDynamic,
}

impl SecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElFirstObservedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElFirstObservedAtEl {}

impl BuildSecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
        SecurityhubAutomationRuleCriteriaElFirstObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElFirstObservedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElFirstObservedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElFirstObservedAtElRef {
        SecurityhubAutomationRuleCriteriaElFirstObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElFirstObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleCriteriaElFirstObservedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElGeneratorIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElGeneratorIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElGeneratorIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElGeneratorIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElGeneratorIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElGeneratorIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElGeneratorIdEl {
        SecurityhubAutomationRuleCriteriaElGeneratorIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElGeneratorIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElGeneratorIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElGeneratorIdElRef {
        SecurityhubAutomationRuleCriteriaElGeneratorIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElGeneratorIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElIdEl {
        SecurityhubAutomationRuleCriteriaElIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElIdElRef {
        SecurityhubAutomationRuleCriteriaElIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
        SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef {
        SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElLastObservedAtElDynamic {
    date_range:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElLastObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElLastObservedAtElDynamic,
}

impl SecurityhubAutomationRuleCriteriaElLastObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElLastObservedAtEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElLastObservedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElLastObservedAtEl {}

impl BuildSecurityhubAutomationRuleCriteriaElLastObservedAtEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElLastObservedAtEl {
        SecurityhubAutomationRuleCriteriaElLastObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElLastObservedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElLastObservedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElLastObservedAtElRef {
        SecurityhubAutomationRuleCriteriaElLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElLastObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleCriteriaElLastObservedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElNoteTextEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElNoteTextEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElNoteTextEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteTextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElNoteTextEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElNoteTextEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElNoteTextEl {
        SecurityhubAutomationRuleCriteriaElNoteTextEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElNoteTextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElNoteTextElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElNoteTextElRef {
        SecurityhubAutomationRuleCriteriaElNoteTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElNoteTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDynamic,
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {}

impl BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElRef {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElNoteUpdatedByElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElNoteUpdatedByElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElNoteUpdatedByElRef {
        SecurityhubAutomationRuleCriteriaElNoteUpdatedByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElNoteUpdatedByElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElProductArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElProductArnEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElProductArnEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElProductArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElProductArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElProductArnEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElProductArnEl {
        SecurityhubAutomationRuleCriteriaElProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElProductArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElProductArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElProductArnElRef {
        SecurityhubAutomationRuleCriteriaElProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElProductArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElProductNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElProductNameEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElProductNameEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElProductNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElProductNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElProductNameEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElProductNameEl {
        SecurityhubAutomationRuleCriteriaElProductNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElProductNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElProductNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElProductNameElRef {
        SecurityhubAutomationRuleCriteriaElProductNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElProductNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElRecordStateEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElRecordStateEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElRecordStateEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElRecordStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElRecordStateEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElRecordStateEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElRecordStateEl {
        SecurityhubAutomationRuleCriteriaElRecordStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElRecordStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElRecordStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElRecordStateElRef {
        SecurityhubAutomationRuleCriteriaElRecordStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElRecordStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
        SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElRelatedFindingsIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElRelatedFindingsIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElRelatedFindingsIdElRef {
        SecurityhubAutomationRuleCriteriaElRelatedFindingsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElRelatedFindingsIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
        SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnElRef {
        SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
        SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceApplicationArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceApplicationArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceApplicationArnElRef {
        SecurityhubAutomationRuleCriteriaElResourceApplicationArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceApplicationArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
        SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceApplicationNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceApplicationNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceApplicationNameElRef {
        SecurityhubAutomationRuleCriteriaElResourceApplicationNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceApplicationNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
        SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceDetailsOtherElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceDetailsOtherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceDetailsOtherElRef {
        SecurityhubAutomationRuleCriteriaElResourceDetailsOtherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceDetailsOtherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceIdEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceIdEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceIdEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceIdEl {
        SecurityhubAutomationRuleCriteriaElResourceIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceIdElRef {
        SecurityhubAutomationRuleCriteriaElResourceIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourcePartitionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourcePartitionEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourcePartitionEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourcePartitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourcePartitionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourcePartitionEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourcePartitionEl {
        SecurityhubAutomationRuleCriteriaElResourcePartitionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourcePartitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourcePartitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourcePartitionElRef {
        SecurityhubAutomationRuleCriteriaElResourcePartitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourcePartitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceRegionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceRegionEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceRegionEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceRegionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceRegionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceRegionEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceRegionEl {
        SecurityhubAutomationRuleCriteriaElResourceRegionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceRegionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceRegionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceRegionElRef {
        SecurityhubAutomationRuleCriteriaElResourceRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceRegionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceTagsEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceTagsEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceTagsEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceTagsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceTagsEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceTagsEl {
        SecurityhubAutomationRuleCriteriaElResourceTagsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceTagsElRef {
        SecurityhubAutomationRuleCriteriaElResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElResourceTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElResourceTypeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElResourceTypeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElResourceTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElResourceTypeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElResourceTypeEl {
        SecurityhubAutomationRuleCriteriaElResourceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElResourceTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElResourceTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElResourceTypeElRef {
        SecurityhubAutomationRuleCriteriaElResourceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElResourceTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElSeverityLabelEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElSeverityLabelEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElSeverityLabelEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElSeverityLabelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElSeverityLabelEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElSeverityLabelEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElSeverityLabelEl {
        SecurityhubAutomationRuleCriteriaElSeverityLabelEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElSeverityLabelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElSeverityLabelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElSeverityLabelElRef {
        SecurityhubAutomationRuleCriteriaElSeverityLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElSeverityLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElSourceUrlEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElSourceUrlEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElSourceUrlEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElSourceUrlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElSourceUrlEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElSourceUrlEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElSourceUrlEl {
        SecurityhubAutomationRuleCriteriaElSourceUrlEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElSourceUrlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElSourceUrlElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElSourceUrlElRef {
        SecurityhubAutomationRuleCriteriaElSourceUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElSourceUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElTitleEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElTitleEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElTitleEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElTitleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElTitleEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElTitleEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElTitleEl {
        SecurityhubAutomationRuleCriteriaElTitleEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElTitleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElTitleElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElTitleElRef {
        SecurityhubAutomationRuleCriteriaElTitleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElTitleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElTypeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElTypeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElTypeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElTypeEl {
        SecurityhubAutomationRuleCriteriaElTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElTypeElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElTypeElRef {
        SecurityhubAutomationRuleCriteriaElTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
        SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef {
        SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElUpdatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElUpdatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElUpdatedAtElDynamic,
}

impl SecurityhubAutomationRuleCriteriaElUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElUpdatedAtEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElUpdatedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElUpdatedAtEl {}

impl BuildSecurityhubAutomationRuleCriteriaElUpdatedAtEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElUpdatedAtEl {
        SecurityhubAutomationRuleCriteriaElUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElUpdatedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElUpdatedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElUpdatedAtElRef {
        SecurityhubAutomationRuleCriteriaElUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElUpdatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubAutomationRuleCriteriaElUpdatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
        SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElUserDefinedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElUserDefinedFieldsElRef {
        SecurityhubAutomationRuleCriteriaElUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElVerificationStateEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElVerificationStateEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElVerificationStateEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElVerificationStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElVerificationStateEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElVerificationStateEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElVerificationStateEl {
        SecurityhubAutomationRuleCriteriaElVerificationStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElVerificationStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElVerificationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElVerificationStateElRef {
        SecurityhubAutomationRuleCriteriaElVerificationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElVerificationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl SecurityhubAutomationRuleCriteriaElWorkflowStatusEl {}

impl ToListMappable for SecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaElWorkflowStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
        SecurityhubAutomationRuleCriteriaElWorkflowStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElWorkflowStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElWorkflowStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubAutomationRuleCriteriaElWorkflowStatusElRef {
        SecurityhubAutomationRuleCriteriaElWorkflowStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElWorkflowStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleCriteriaElDynamic {
    aws_account_id: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElAwsAccountIdEl>>,
    aws_account_name: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElAwsAccountNameEl>>,
    company_name: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElCompanyNameEl>>,
    compliance_associated_standards_id:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl>>,
    compliance_security_control_id:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl>>,
    compliance_status: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElComplianceStatusEl>>,
    confidence: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElConfidenceEl>>,
    created_at: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElCreatedAtEl>>,
    criticality: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElCriticalityEl>>,
    description: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElDescriptionEl>>,
    first_observed_at: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElFirstObservedAtEl>>,
    generator_id: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElGeneratorIdEl>>,
    id: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElIdEl>>,
    last_observed_at: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElLastObservedAtEl>>,
    note_text: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElNoteTextEl>>,
    note_updated_at: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl>>,
    note_updated_by: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl>>,
    product_arn: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElProductArnEl>>,
    product_name: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElProductNameEl>>,
    record_state: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElRecordStateEl>>,
    related_findings_id:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl>>,
    related_findings_product_arn:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl>>,
    resource_application_arn:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl>>,
    resource_application_name:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl>>,
    resource_details_other:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl>>,
    resource_id: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceIdEl>>,
    resource_partition:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourcePartitionEl>>,
    resource_region: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceRegionEl>>,
    resource_tags: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceTagsEl>>,
    resource_type: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElResourceTypeEl>>,
    severity_label: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElSeverityLabelEl>>,
    source_url: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElSourceUrlEl>>,
    title: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElTitleEl>>,
    type_: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElTypeEl>>,
    updated_at: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElUpdatedAtEl>>,
    user_defined_fields:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl>>,
    verification_state:
        Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElVerificationStateEl>>,
    workflow_status: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaElWorkflowStatusEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<Vec<SecurityhubAutomationRuleCriteriaElAwsAccountIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_name: Option<Vec<SecurityhubAutomationRuleCriteriaElAwsAccountNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<Vec<SecurityhubAutomationRuleCriteriaElCompanyNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_associated_standards_id:
        Option<Vec<SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_security_control_id:
        Option<Vec<SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_status: Option<Vec<SecurityhubAutomationRuleCriteriaElComplianceStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<Vec<SecurityhubAutomationRuleCriteriaElConfidenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<Vec<SecurityhubAutomationRuleCriteriaElCreatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<Vec<SecurityhubAutomationRuleCriteriaElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Vec<SecurityhubAutomationRuleCriteriaElDescriptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_observed_at: Option<Vec<SecurityhubAutomationRuleCriteriaElFirstObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator_id: Option<Vec<SecurityhubAutomationRuleCriteriaElGeneratorIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Vec<SecurityhubAutomationRuleCriteriaElIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_observed_at: Option<Vec<SecurityhubAutomationRuleCriteriaElLastObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_text: Option<Vec<SecurityhubAutomationRuleCriteriaElNoteTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_updated_at: Option<Vec<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_updated_by: Option<Vec<SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_arn: Option<Vec<SecurityhubAutomationRuleCriteriaElProductArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<Vec<SecurityhubAutomationRuleCriteriaElProductNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_state: Option<Vec<SecurityhubAutomationRuleCriteriaElRecordStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_findings_id: Option<Vec<SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_findings_product_arn:
        Option<Vec<SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_application_arn:
        Option<Vec<SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_application_name:
        Option<Vec<SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_details_other: Option<Vec<SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<Vec<SecurityhubAutomationRuleCriteriaElResourceIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_partition: Option<Vec<SecurityhubAutomationRuleCriteriaElResourcePartitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_region: Option<Vec<SecurityhubAutomationRuleCriteriaElResourceRegionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<Vec<SecurityhubAutomationRuleCriteriaElResourceTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<Vec<SecurityhubAutomationRuleCriteriaElResourceTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity_label: Option<Vec<SecurityhubAutomationRuleCriteriaElSeverityLabelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_url: Option<Vec<SecurityhubAutomationRuleCriteriaElSourceUrlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<SecurityhubAutomationRuleCriteriaElTitleEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<Vec<SecurityhubAutomationRuleCriteriaElTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<Vec<SecurityhubAutomationRuleCriteriaElUpdatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_fields: Option<Vec<SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_state: Option<Vec<SecurityhubAutomationRuleCriteriaElVerificationStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_status: Option<Vec<SecurityhubAutomationRuleCriteriaElWorkflowStatusEl>>,
    dynamic: SecurityhubAutomationRuleCriteriaElDynamic,
}

impl SecurityhubAutomationRuleCriteriaEl {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElAwsAccountIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_account_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `aws_account_name`.\n"]
    pub fn set_aws_account_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElAwsAccountNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_account_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElCompanyNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.company_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.company_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `compliance_associated_standards_id`.\n"]
    pub fn set_compliance_associated_standards_id(
        mut self,
        v: impl Into<
            BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceAssociatedStandardsIdEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compliance_associated_standards_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_associated_standards_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `compliance_security_control_id`.\n"]
    pub fn set_compliance_security_control_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceSecurityControlIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compliance_security_control_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_security_control_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElComplianceStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compliance_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_status = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElConfidenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidence = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElCreatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.created_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.created_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElCriticalityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.criticality = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElDescriptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.description = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.description = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElFirstObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.first_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `generator_id`.\n"]
    pub fn set_generator_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElGeneratorIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generator_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generator_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `note_text`.\n"]
    pub fn set_note_text(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_text = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `note_updated_at`.\n"]
    pub fn set_note_updated_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_updated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `note_updated_by`.\n"]
    pub fn set_note_updated_by(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElNoteUpdatedByEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_updated_by = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_by = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `product_arn`.\n"]
    pub fn set_product_arn(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElProductArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.product_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.product_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElProductNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.product_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.product_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `record_state`.\n"]
    pub fn set_record_state(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElRecordStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_state = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `related_findings_id`.\n"]
    pub fn set_related_findings_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElRelatedFindingsIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_findings_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `related_findings_product_arn`.\n"]
    pub fn set_related_findings_product_arn(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElRelatedFindingsProductArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_findings_product_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_product_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_application_arn`.\n"]
    pub fn set_resource_application_arn(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceApplicationArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_application_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_application_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_application_name`.\n"]
    pub fn set_resource_application_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceApplicationNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_application_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_application_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_details_other`.\n"]
    pub fn set_resource_details_other(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceDetailsOtherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_details_other = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_details_other = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_partition`.\n"]
    pub fn set_resource_partition(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourcePartitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_partition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_partition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_region`.\n"]
    pub fn set_resource_region(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceRegionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_region = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_region = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElResourceTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `severity_label`.\n"]
    pub fn set_severity_label(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElSeverityLabelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.severity_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_label = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_url`.\n"]
    pub fn set_source_url(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElSourceUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_url = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_url = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElTitleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.title = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.title = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.type_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.updated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElUserDefinedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_defined_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_fields = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElVerificationStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.verification_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.verification_state = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `workflow_status`.\n"]
    pub fn set_workflow_status(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubAutomationRuleCriteriaElWorkflowStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workflow_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow_status = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecurityhubAutomationRuleCriteriaEl {
    type O = BlockAssignable<SecurityhubAutomationRuleCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleCriteriaEl {}

impl BuildSecurityhubAutomationRuleCriteriaEl {
    pub fn build(self) -> SecurityhubAutomationRuleCriteriaEl {
        SecurityhubAutomationRuleCriteriaEl {
            aws_account_id: core::default::Default::default(),
            aws_account_name: core::default::Default::default(),
            company_name: core::default::Default::default(),
            compliance_associated_standards_id: core::default::Default::default(),
            compliance_security_control_id: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            confidence: core::default::Default::default(),
            created_at: core::default::Default::default(),
            criticality: core::default::Default::default(),
            description: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            generator_id: core::default::Default::default(),
            id: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            note_text: core::default::Default::default(),
            note_updated_at: core::default::Default::default(),
            note_updated_by: core::default::Default::default(),
            product_arn: core::default::Default::default(),
            product_name: core::default::Default::default(),
            record_state: core::default::Default::default(),
            related_findings_id: core::default::Default::default(),
            related_findings_product_arn: core::default::Default::default(),
            resource_application_arn: core::default::Default::default(),
            resource_application_name: core::default::Default::default(),
            resource_details_other: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_partition: core::default::Default::default(),
            resource_region: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity_label: core::default::Default::default(),
            source_url: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            workflow_status: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecurityhubAutomationRuleCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubAutomationRuleCriteriaElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubAutomationRuleCriteriaElRef {
        SecurityhubAutomationRuleCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleDynamic {
    actions: Option<DynamicBlock<SecurityhubAutomationRuleActionsEl>>,
    criteria: Option<DynamicBlock<SecurityhubAutomationRuleCriteriaEl>>,
}
