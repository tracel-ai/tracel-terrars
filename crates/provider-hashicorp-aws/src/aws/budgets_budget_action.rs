use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BudgetsBudgetActionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    action_type: PrimField<String>,
    approval_model: PrimField<String>,
    budget_name: PrimField<String>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    notification_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_threshold: Option<Vec<BudgetsBudgetActionActionThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<Vec<BudgetsBudgetActionDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber: Option<Vec<BudgetsBudgetActionSubscriberEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BudgetsBudgetActionTimeoutsEl>,
    dynamic: BudgetsBudgetActionDynamic,
}

struct BudgetsBudgetAction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BudgetsBudgetActionData>,
}

#[derive(Clone)]
pub struct BudgetsBudgetAction(Rc<BudgetsBudgetAction_>);

impl BudgetsBudgetAction {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
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

    #[doc = "Set the field `action_threshold`.\n"]
    pub fn set_action_threshold(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionActionThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_threshold = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `subscriber`.\n"]
    pub fn set_subscriber(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionSubscriberEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BudgetsBudgetActionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_type` after provisioning.\n"]
    pub fn action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approval_model` after provisioning.\n"]
    pub fn approval_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_name` after provisioning.\n"]
    pub fn budget_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.budget_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> ListRef<BudgetsBudgetActionActionThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_threshold", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<BudgetsBudgetActionDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BudgetsBudgetActionTimeoutsElRef {
        BudgetsBudgetActionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BudgetsBudgetAction {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BudgetsBudgetAction {}

impl ToListMappable for BudgetsBudgetAction {
    type O = ListRef<BudgetsBudgetActionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BudgetsBudgetAction_ {
    fn extract_resource_type(&self) -> String {
        "aws_budgets_budget_action".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBudgetsBudgetAction {
    pub tf_id: String,
    #[doc = ""]
    pub action_type: PrimField<String>,
    #[doc = ""]
    pub approval_model: PrimField<String>,
    #[doc = ""]
    pub budget_name: PrimField<String>,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub notification_type: PrimField<String>,
}

impl BuildBudgetsBudgetAction {
    pub fn build(self, stack: &mut Stack) -> BudgetsBudgetAction {
        let out = BudgetsBudgetAction(Rc::new(BudgetsBudgetAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BudgetsBudgetActionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                action_type: self.action_type,
                approval_model: self.approval_model,
                budget_name: self.budget_name,
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                notification_type: self.notification_type,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action_threshold: core::default::Default::default(),
                definition: core::default::Default::default(),
                subscriber: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BudgetsBudgetActionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BudgetsBudgetActionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_type` after provisioning.\n"]
    pub fn action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approval_model` after provisioning.\n"]
    pub fn approval_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_model", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_name` after provisioning.\n"]
    pub fn budget_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.budget_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> ListRef<BudgetsBudgetActionActionThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_threshold", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<BudgetsBudgetActionDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BudgetsBudgetActionTimeoutsElRef {
        BudgetsBudgetActionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionActionThresholdEl {
    action_threshold_type: PrimField<String>,
    action_threshold_value: PrimField<f64>,
}

impl BudgetsBudgetActionActionThresholdEl {}

impl ToListMappable for BudgetsBudgetActionActionThresholdEl {
    type O = BlockAssignable<BudgetsBudgetActionActionThresholdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionActionThresholdEl {
    #[doc = ""]
    pub action_threshold_type: PrimField<String>,
    #[doc = ""]
    pub action_threshold_value: PrimField<f64>,
}

impl BuildBudgetsBudgetActionActionThresholdEl {
    pub fn build(self) -> BudgetsBudgetActionActionThresholdEl {
        BudgetsBudgetActionActionThresholdEl {
            action_threshold_type: self.action_threshold_type,
            action_threshold_value: self.action_threshold_value,
        }
    }
}

pub struct BudgetsBudgetActionActionThresholdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionActionThresholdElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetActionActionThresholdElRef {
        BudgetsBudgetActionActionThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionActionThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_threshold_type` after provisioning.\n"]
    pub fn action_threshold_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_threshold_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `action_threshold_value` after provisioning.\n"]
    pub fn action_threshold_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_threshold_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<SetField<PrimField<String>>>,
    policy_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<SetField<PrimField<String>>>,
}

impl BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.groups = Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }

    #[doc = "Set the field `users`.\n"]
    pub fn set_users(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.users = Some(v.into());
        self
    }
}

impl ToListMappable for BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    type O = BlockAssignable<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    #[doc = ""]
    pub policy_arn: PrimField<String>,
}

impl BuildBudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
        BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
            groups: core::default::Default::default(),
            policy_arn: self.policy_arn,
            roles: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.users", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    policy_id: PrimField<String>,
    target_ids: SetField<PrimField<String>>,
}

impl BudgetsBudgetActionDefinitionElScpActionDefinitionEl {}

impl ToListMappable for BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    type O = BlockAssignable<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    #[doc = ""]
    pub policy_id: PrimField<String>,
    #[doc = ""]
    pub target_ids: SetField<PrimField<String>>,
}

impl BuildBudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
        BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
            policy_id: self.policy_id,
            target_ids: self.target_ids,
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `target_ids` after provisioning.\n"]
    pub fn target_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    action_sub_type: PrimField<String>,
    instance_ids: SetField<PrimField<String>>,
    region: PrimField<String>,
}

impl BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {}

impl ToListMappable for BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    type O = BlockAssignable<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    #[doc = ""]
    pub action_sub_type: PrimField<String>,
    #[doc = ""]
    pub instance_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub region: PrimField<String>,
}

impl BuildBudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
        BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
            action_sub_type: self.action_sub_type,
            instance_ids: self.instance_ids,
            region: self.region,
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_sub_type` after provisioning.\n"]
    pub fn action_sub_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_sub_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_ids` after provisioning.\n"]
    pub fn instance_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instance_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudgetActionDefinitionElDynamic {
    iam_action_definition:
        Option<DynamicBlock<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
    scp_action_definition:
        Option<DynamicBlock<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
    ssm_action_definition:
        Option<DynamicBlock<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_action_definition: Option<Vec<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scp_action_definition: Option<Vec<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_action_definition: Option<Vec<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
    dynamic: BudgetsBudgetActionDefinitionElDynamic,
}

impl BudgetsBudgetActionDefinitionEl {
    #[doc = "Set the field `iam_action_definition`.\n"]
    pub fn set_iam_action_definition(
        mut self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iam_action_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_action_definition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `scp_action_definition`.\n"]
    pub fn set_scp_action_definition(
        mut self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scp_action_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scp_action_definition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ssm_action_definition`.\n"]
    pub fn set_ssm_action_definition(
        mut self,
        v: impl Into<BlockAssignable<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssm_action_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_action_definition = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BudgetsBudgetActionDefinitionEl {
    type O = BlockAssignable<BudgetsBudgetActionDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionEl {}

impl BuildBudgetsBudgetActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionEl {
        BudgetsBudgetActionDefinitionEl {
            iam_action_definition: core::default::Default::default(),
            scp_action_definition: core::default::Default::default(),
            ssm_action_definition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionDefinitionElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_action_definition` after provisioning.\n"]
    pub fn iam_action_definition(
        &self,
    ) -> ListRef<BudgetsBudgetActionDefinitionElIamActionDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iam_action_definition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scp_action_definition` after provisioning.\n"]
    pub fn scp_action_definition(
        &self,
    ) -> ListRef<BudgetsBudgetActionDefinitionElScpActionDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scp_action_definition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ssm_action_definition` after provisioning.\n"]
    pub fn ssm_action_definition(
        &self,
    ) -> ListRef<BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssm_action_definition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionSubscriberEl {
    address: PrimField<String>,
    subscription_type: PrimField<String>,
}

impl BudgetsBudgetActionSubscriberEl {}

impl ToListMappable for BudgetsBudgetActionSubscriberEl {
    type O = BlockAssignable<BudgetsBudgetActionSubscriberEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionSubscriberEl {
    #[doc = ""]
    pub address: PrimField<String>,
    #[doc = ""]
    pub subscription_type: PrimField<String>,
}

impl BuildBudgetsBudgetActionSubscriberEl {
    pub fn build(self) -> BudgetsBudgetActionSubscriberEl {
        BudgetsBudgetActionSubscriberEl {
            address: self.address,
            subscription_type: self.subscription_type,
        }
    }
}

pub struct BudgetsBudgetActionSubscriberElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionSubscriberElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetActionSubscriberElRef {
        BudgetsBudgetActionSubscriberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionSubscriberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\n"]
    pub fn subscription_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BudgetsBudgetActionTimeoutsEl {
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

impl ToListMappable for BudgetsBudgetActionTimeoutsEl {
    type O = BlockAssignable<BudgetsBudgetActionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionTimeoutsEl {}

impl BuildBudgetsBudgetActionTimeoutsEl {
    pub fn build(self) -> BudgetsBudgetActionTimeoutsEl {
        BudgetsBudgetActionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BudgetsBudgetActionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BudgetsBudgetActionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetActionTimeoutsElRef {
        BudgetsBudgetActionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionTimeoutsElRef {
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
struct BudgetsBudgetActionDynamic {
    action_threshold: Option<DynamicBlock<BudgetsBudgetActionActionThresholdEl>>,
    definition: Option<DynamicBlock<BudgetsBudgetActionDefinitionEl>>,
    subscriber: Option<DynamicBlock<BudgetsBudgetActionSubscriberEl>>,
}
