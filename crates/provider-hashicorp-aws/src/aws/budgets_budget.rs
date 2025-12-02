use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BudgetsBudgetData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_view_arn: Option<PrimField<String>>,
    budget_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_amount: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_period_end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_period_start: Option<PrimField<String>>,
    time_unit: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_adjust_data: Option<Vec<BudgetsBudgetAutoAdjustDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_filter: Option<Vec<BudgetsBudgetCostFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_types: Option<Vec<BudgetsBudgetCostTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Vec<BudgetsBudgetNotificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    planned_limit: Option<Vec<BudgetsBudgetPlannedLimitEl>>,
    dynamic: BudgetsBudgetDynamic,
}
struct BudgetsBudget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BudgetsBudgetData>,
}
#[derive(Clone)]
pub struct BudgetsBudget(Rc<BudgetsBudget_>);
impl BudgetsBudget {
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
    #[doc = "Set the field `billing_view_arn`.\n"]
    pub fn set_billing_view_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_view_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `limit_amount`.\n"]
    pub fn set_limit_amount(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().limit_amount = Some(v.into());
        self
    }
    #[doc = "Set the field `limit_unit`.\n"]
    pub fn set_limit_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().limit_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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
    #[doc = "Set the field `time_period_end`.\n"]
    pub fn set_time_period_end(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().time_period_end = Some(v.into());
        self
    }
    #[doc = "Set the field `time_period_start`.\n"]
    pub fn set_time_period_start(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().time_period_start = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_adjust_data`.\n"]
    pub fn set_auto_adjust_data(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetAutoAdjustDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_adjust_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_adjust_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cost_filter`.\n"]
    pub fn set_cost_filter(self, v: impl Into<BlockAssignable<BudgetsBudgetCostFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cost_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cost_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cost_types`.\n"]
    pub fn set_cost_types(self, v: impl Into<BlockAssignable<BudgetsBudgetCostTypesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cost_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cost_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification`.\n"]
    pub fn set_notification(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetNotificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `planned_limit`.\n"]
    pub fn set_planned_limit(
        self,
        v: impl Into<BlockAssignable<BudgetsBudgetPlannedLimitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().planned_limit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.planned_limit = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_view_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.budget_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `limit_amount` after provisioning.\n"]
    pub fn limit_amount(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.limit_amount", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `limit_unit` after provisioning.\n"]
    pub fn limit_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.limit_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_period_end", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_period_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> ListRef<BudgetsBudgetAutoAdjustDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_adjust_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> ListRef<BudgetsBudgetCostTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_types", self.extract_ref()),
        )
    }
}
impl Referable for BudgetsBudget {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BudgetsBudget {}
impl ToListMappable for BudgetsBudget {
    type O = ListRef<BudgetsBudgetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BudgetsBudget_ {
    fn extract_resource_type(&self) -> String {
        "aws_budgets_budget".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBudgetsBudget {
    pub tf_id: String,
    #[doc = ""]
    pub budget_type: PrimField<String>,
    #[doc = ""]
    pub time_unit: PrimField<String>,
}
impl BuildBudgetsBudget {
    pub fn build(self, stack: &mut Stack) -> BudgetsBudget {
        let out = BudgetsBudget(Rc::new(BudgetsBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BudgetsBudgetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                billing_view_arn: core::default::Default::default(),
                budget_type: self.budget_type,
                id: core::default::Default::default(),
                limit_amount: core::default::Default::default(),
                limit_unit: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                time_period_end: core::default::Default::default(),
                time_period_start: core::default::Default::default(),
                time_unit: self.time_unit,
                auto_adjust_data: core::default::Default::default(),
                cost_filter: core::default::Default::default(),
                cost_types: core::default::Default::default(),
                notification: core::default::Default::default(),
                planned_limit: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BudgetsBudgetRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BudgetsBudgetRef {
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_view_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.budget_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `limit_amount` after provisioning.\n"]
    pub fn limit_amount(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.limit_amount", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `limit_unit` after provisioning.\n"]
    pub fn limit_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.limit_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_period_end", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_period_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> ListRef<BudgetsBudgetAutoAdjustDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_adjust_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> ListRef<BudgetsBudgetCostTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_types", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    budget_adjustment_period: PrimField<f64>,
}
impl BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {}
impl ToListMappable for BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    type O = BlockAssignable<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    #[doc = ""]
    pub budget_adjustment_period: PrimField<f64>,
}
impl BuildBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    pub fn build(self) -> BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
        BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
            budget_adjustment_period: self.budget_adjustment_period,
        }
    }
}
pub struct BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
        BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `budget_adjustment_period` after provisioning.\n"]
    pub fn budget_adjustment_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.budget_adjustment_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lookback_available_periods` after provisioning.\n"]
    pub fn lookback_available_periods(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lookback_available_periods", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BudgetsBudgetAutoAdjustDataElDynamic {
    historical_options: Option<DynamicBlock<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
}
#[derive(Serialize)]
pub struct BudgetsBudgetAutoAdjustDataEl {
    auto_adjust_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    historical_options: Option<Vec<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
    dynamic: BudgetsBudgetAutoAdjustDataElDynamic,
}
impl BudgetsBudgetAutoAdjustDataEl {
    #[doc = "Set the field `historical_options`.\n"]
    pub fn set_historical_options(
        mut self,
        v: impl Into<BlockAssignable<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.historical_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.historical_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BudgetsBudgetAutoAdjustDataEl {
    type O = BlockAssignable<BudgetsBudgetAutoAdjustDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetAutoAdjustDataEl {
    #[doc = ""]
    pub auto_adjust_type: PrimField<String>,
}
impl BuildBudgetsBudgetAutoAdjustDataEl {
    pub fn build(self) -> BudgetsBudgetAutoAdjustDataEl {
        BudgetsBudgetAutoAdjustDataEl {
            auto_adjust_type: self.auto_adjust_type,
            historical_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BudgetsBudgetAutoAdjustDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetAutoAdjustDataElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetAutoAdjustDataElRef {
        BudgetsBudgetAutoAdjustDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetAutoAdjustDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_adjust_type` after provisioning.\n"]
    pub fn auto_adjust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_adjust_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_auto_adjust_time` after provisioning.\n"]
    pub fn last_auto_adjust_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_auto_adjust_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `historical_options` after provisioning.\n"]
    pub fn historical_options(
        &self,
    ) -> ListRef<BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.historical_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BudgetsBudgetCostFilterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl BudgetsBudgetCostFilterEl {}
impl ToListMappable for BudgetsBudgetCostFilterEl {
    type O = BlockAssignable<BudgetsBudgetCostFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetCostFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}
impl BuildBudgetsBudgetCostFilterEl {
    pub fn build(self) -> BudgetsBudgetCostFilterEl {
        BudgetsBudgetCostFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}
pub struct BudgetsBudgetCostFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetCostFilterElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetCostFilterElRef {
        BudgetsBudgetCostFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetCostFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct BudgetsBudgetCostTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_credit: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_discount: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_other_subscription: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_recurring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_refund: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subscription: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_tax: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_upfront: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_amortized: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_blended: Option<PrimField<bool>>,
}
impl BudgetsBudgetCostTypesEl {
    #[doc = "Set the field `include_credit`.\n"]
    pub fn set_include_credit(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_credit = Some(v.into());
        self
    }
    #[doc = "Set the field `include_discount`.\n"]
    pub fn set_include_discount(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_discount = Some(v.into());
        self
    }
    #[doc = "Set the field `include_other_subscription`.\n"]
    pub fn set_include_other_subscription(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_other_subscription = Some(v.into());
        self
    }
    #[doc = "Set the field `include_recurring`.\n"]
    pub fn set_include_recurring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_recurring = Some(v.into());
        self
    }
    #[doc = "Set the field `include_refund`.\n"]
    pub fn set_include_refund(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_refund = Some(v.into());
        self
    }
    #[doc = "Set the field `include_subscription`.\n"]
    pub fn set_include_subscription(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_subscription = Some(v.into());
        self
    }
    #[doc = "Set the field `include_support`.\n"]
    pub fn set_include_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_support = Some(v.into());
        self
    }
    #[doc = "Set the field `include_tax`.\n"]
    pub fn set_include_tax(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_tax = Some(v.into());
        self
    }
    #[doc = "Set the field `include_upfront`.\n"]
    pub fn set_include_upfront(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_upfront = Some(v.into());
        self
    }
    #[doc = "Set the field `use_amortized`.\n"]
    pub fn set_use_amortized(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_amortized = Some(v.into());
        self
    }
    #[doc = "Set the field `use_blended`.\n"]
    pub fn set_use_blended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_blended = Some(v.into());
        self
    }
}
impl ToListMappable for BudgetsBudgetCostTypesEl {
    type O = BlockAssignable<BudgetsBudgetCostTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetCostTypesEl {}
impl BuildBudgetsBudgetCostTypesEl {
    pub fn build(self) -> BudgetsBudgetCostTypesEl {
        BudgetsBudgetCostTypesEl {
            include_credit: core::default::Default::default(),
            include_discount: core::default::Default::default(),
            include_other_subscription: core::default::Default::default(),
            include_recurring: core::default::Default::default(),
            include_refund: core::default::Default::default(),
            include_subscription: core::default::Default::default(),
            include_support: core::default::Default::default(),
            include_tax: core::default::Default::default(),
            include_upfront: core::default::Default::default(),
            use_amortized: core::default::Default::default(),
            use_blended: core::default::Default::default(),
        }
    }
}
pub struct BudgetsBudgetCostTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetCostTypesElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetCostTypesElRef {
        BudgetsBudgetCostTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetCostTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_credit` after provisioning.\n"]
    pub fn include_credit(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_credit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_discount` after provisioning.\n"]
    pub fn include_discount(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_discount", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_other_subscription` after provisioning.\n"]
    pub fn include_other_subscription(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_other_subscription", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_recurring` after provisioning.\n"]
    pub fn include_recurring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_recurring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_refund` after provisioning.\n"]
    pub fn include_refund(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_refund", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_subscription` after provisioning.\n"]
    pub fn include_subscription(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_subscription", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_support` after provisioning.\n"]
    pub fn include_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_support", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_tax` after provisioning.\n"]
    pub fn include_tax(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_tax", self.base))
    }
    #[doc = "Get a reference to the value of field `include_upfront` after provisioning.\n"]
    pub fn include_upfront(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_upfront", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_amortized` after provisioning.\n"]
    pub fn use_amortized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_amortized", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_blended` after provisioning.\n"]
    pub fn use_blended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_blended", self.base))
    }
}
#[derive(Serialize)]
pub struct BudgetsBudgetNotificationEl {
    comparison_operator: PrimField<String>,
    notification_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_email_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_sns_topic_arns: Option<SetField<PrimField<String>>>,
    threshold: PrimField<f64>,
    threshold_type: PrimField<String>,
}
impl BudgetsBudgetNotificationEl {
    #[doc = "Set the field `subscriber_email_addresses`.\n"]
    pub fn set_subscriber_email_addresses(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.subscriber_email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `subscriber_sns_topic_arns`.\n"]
    pub fn set_subscriber_sns_topic_arns(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.subscriber_sns_topic_arns = Some(v.into());
        self
    }
}
impl ToListMappable for BudgetsBudgetNotificationEl {
    type O = BlockAssignable<BudgetsBudgetNotificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetNotificationEl {
    #[doc = ""]
    pub comparison_operator: PrimField<String>,
    #[doc = ""]
    pub notification_type: PrimField<String>,
    #[doc = ""]
    pub threshold: PrimField<f64>,
    #[doc = ""]
    pub threshold_type: PrimField<String>,
}
impl BuildBudgetsBudgetNotificationEl {
    pub fn build(self) -> BudgetsBudgetNotificationEl {
        BudgetsBudgetNotificationEl {
            comparison_operator: self.comparison_operator,
            notification_type: self.notification_type,
            subscriber_email_addresses: core::default::Default::default(),
            subscriber_sns_topic_arns: core::default::Default::default(),
            threshold: self.threshold,
            threshold_type: self.threshold_type,
        }
    }
}
pub struct BudgetsBudgetNotificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetNotificationElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetNotificationElRef {
        BudgetsBudgetNotificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetNotificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comparison_operator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_email_addresses` after provisioning.\n"]
    pub fn subscriber_email_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subscriber_email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_sns_topic_arns` after provisioning.\n"]
    pub fn subscriber_sns_topic_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subscriber_sns_topic_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold_type` after provisioning.\n"]
    pub fn threshold_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BudgetsBudgetPlannedLimitEl {
    amount: PrimField<String>,
    start_time: PrimField<String>,
    unit: PrimField<String>,
}
impl BudgetsBudgetPlannedLimitEl {}
impl ToListMappable for BudgetsBudgetPlannedLimitEl {
    type O = BlockAssignable<BudgetsBudgetPlannedLimitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBudgetsBudgetPlannedLimitEl {
    #[doc = ""]
    pub amount: PrimField<String>,
    #[doc = ""]
    pub start_time: PrimField<String>,
    #[doc = ""]
    pub unit: PrimField<String>,
}
impl BuildBudgetsBudgetPlannedLimitEl {
    pub fn build(self) -> BudgetsBudgetPlannedLimitEl {
        BudgetsBudgetPlannedLimitEl {
            amount: self.amount,
            start_time: self.start_time,
            unit: self.unit,
        }
    }
}
pub struct BudgetsBudgetPlannedLimitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BudgetsBudgetPlannedLimitElRef {
    fn new(shared: StackShared, base: String) -> BudgetsBudgetPlannedLimitElRef {
        BudgetsBudgetPlannedLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BudgetsBudgetPlannedLimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}
#[derive(Serialize, Default)]
struct BudgetsBudgetDynamic {
    auto_adjust_data: Option<DynamicBlock<BudgetsBudgetAutoAdjustDataEl>>,
    cost_filter: Option<DynamicBlock<BudgetsBudgetCostFilterEl>>,
    cost_types: Option<DynamicBlock<BudgetsBudgetCostTypesEl>>,
    notification: Option<DynamicBlock<BudgetsBudgetNotificationEl>>,
    planned_limit: Option<DynamicBlock<BudgetsBudgetPlannedLimitEl>>,
}
