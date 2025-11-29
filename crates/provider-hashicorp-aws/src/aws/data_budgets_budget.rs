use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBudgetsBudgetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataBudgetsBudget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBudgetsBudgetData>,
}

#[derive(Clone)]
pub struct DataBudgetsBudget(Rc<DataBudgetsBudget_>);

impl DataBudgetsBudget {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> ListRef<DataBudgetsBudgetAutoAdjustDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_exceeded` after provisioning.\n"]
    pub fn budget_exceeded(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.budget_exceeded", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_limit` after provisioning.\n"]
    pub fn budget_limit(&self) -> ListRef<DataBudgetsBudgetBudgetLimitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.budget_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `calculated_spend` after provisioning.\n"]
    pub fn calculated_spend(&self) -> ListRef<DataBudgetsBudgetCalculatedSpendElRef> {
        ListRef::new(self.shared().clone(), format!("{}.calculated_spend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filter` after provisioning.\n"]
    pub fn cost_filter(&self) -> SetRef<DataBudgetsBudgetCostFilterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.cost_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> ListRef<DataBudgetsBudgetCostTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification` after provisioning.\n"]
    pub fn notification(&self) -> SetRef<DataBudgetsBudgetNotificationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `planned_limit` after provisioning.\n"]
    pub fn planned_limit(&self) -> SetRef<DataBudgetsBudgetPlannedLimitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.planned_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}

impl Referable for DataBudgetsBudget {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBudgetsBudget { }

impl ToListMappable for DataBudgetsBudget {
    type O = ListRef<DataBudgetsBudgetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBudgetsBudget_ {
    fn extract_datasource_type(&self) -> String {
        "aws_budgets_budget".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBudgetsBudget {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataBudgetsBudget {
    pub fn build(self, stack: &mut Stack) -> DataBudgetsBudget {
        let out = DataBudgetsBudget(Rc::new(DataBudgetsBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBudgetsBudgetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                name_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBudgetsBudgetRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBudgetsBudgetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> ListRef<DataBudgetsBudgetAutoAdjustDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_exceeded` after provisioning.\n"]
    pub fn budget_exceeded(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.budget_exceeded", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_limit` after provisioning.\n"]
    pub fn budget_limit(&self) -> ListRef<DataBudgetsBudgetBudgetLimitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.budget_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `calculated_spend` after provisioning.\n"]
    pub fn calculated_spend(&self) -> ListRef<DataBudgetsBudgetCalculatedSpendElRef> {
        ListRef::new(self.shared().clone(), format!("{}.calculated_spend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filter` after provisioning.\n"]
    pub fn cost_filter(&self) -> SetRef<DataBudgetsBudgetCostFilterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.cost_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> ListRef<DataBudgetsBudgetCostTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification` after provisioning.\n"]
    pub fn notification(&self) -> SetRef<DataBudgetsBudgetNotificationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `planned_limit` after provisioning.\n"]
    pub fn planned_limit(&self) -> SetRef<DataBudgetsBudgetPlannedLimitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.planned_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_adjustment_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookback_available_periods: Option<PrimField<f64>>,
}

impl DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    #[doc = "Set the field `budget_adjustment_period`.\n"]
    pub fn set_budget_adjustment_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.budget_adjustment_period = Some(v.into());
        self
    }

    #[doc = "Set the field `lookback_available_periods`.\n"]
    pub fn set_lookback_available_periods(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lookback_available_periods = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    type O = BlockAssignable<DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {}

impl BuildDataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    pub fn build(self) -> DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
        DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
            budget_adjustment_period: core::default::Default::default(),
            lookback_available_periods: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
        DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `budget_adjustment_period` after provisioning.\n"]
    pub fn budget_adjustment_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.budget_adjustment_period", self.base))
    }

    #[doc = "Get a reference to the value of field `lookback_available_periods` after provisioning.\n"]
    pub fn lookback_available_periods(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lookback_available_periods", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetAutoAdjustDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_adjust_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    historical_options: Option<ListField<DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_auto_adjust_time: Option<PrimField<String>>,
}

impl DataBudgetsBudgetAutoAdjustDataEl {
    #[doc = "Set the field `auto_adjust_type`.\n"]
    pub fn set_auto_adjust_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_adjust_type = Some(v.into());
        self
    }

    #[doc = "Set the field `historical_options`.\n"]
    pub fn set_historical_options(
        mut self,
        v: impl Into<ListField<DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
    ) -> Self {
        self.historical_options = Some(v.into());
        self
    }

    #[doc = "Set the field `last_auto_adjust_time`.\n"]
    pub fn set_last_auto_adjust_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_auto_adjust_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetAutoAdjustDataEl {
    type O = BlockAssignable<DataBudgetsBudgetAutoAdjustDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetAutoAdjustDataEl {}

impl BuildDataBudgetsBudgetAutoAdjustDataEl {
    pub fn build(self) -> DataBudgetsBudgetAutoAdjustDataEl {
        DataBudgetsBudgetAutoAdjustDataEl {
            auto_adjust_type: core::default::Default::default(),
            historical_options: core::default::Default::default(),
            last_auto_adjust_time: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetAutoAdjustDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetAutoAdjustDataElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetAutoAdjustDataElRef {
        DataBudgetsBudgetAutoAdjustDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetAutoAdjustDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_adjust_type` after provisioning.\n"]
    pub fn auto_adjust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_adjust_type", self.base))
    }

    #[doc = "Get a reference to the value of field `historical_options` after provisioning.\n"]
    pub fn historical_options(&self) -> ListRef<DataBudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.historical_options", self.base))
    }

    #[doc = "Get a reference to the value of field `last_auto_adjust_time` after provisioning.\n"]
    pub fn last_auto_adjust_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_auto_adjust_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetBudgetLimitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl DataBudgetsBudgetBudgetLimitEl {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amount = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetBudgetLimitEl {
    type O = BlockAssignable<DataBudgetsBudgetBudgetLimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetBudgetLimitEl {}

impl BuildDataBudgetsBudgetBudgetLimitEl {
    pub fn build(self) -> DataBudgetsBudgetBudgetLimitEl {
        DataBudgetsBudgetBudgetLimitEl {
            amount: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetBudgetLimitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetBudgetLimitElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetBudgetLimitElRef {
        DataBudgetsBudgetBudgetLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetBudgetLimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetCalculatedSpendElActualSpendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl DataBudgetsBudgetCalculatedSpendElActualSpendEl {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amount = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetCalculatedSpendElActualSpendEl {
    type O = BlockAssignable<DataBudgetsBudgetCalculatedSpendElActualSpendEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetCalculatedSpendElActualSpendEl {}

impl BuildDataBudgetsBudgetCalculatedSpendElActualSpendEl {
    pub fn build(self) -> DataBudgetsBudgetCalculatedSpendElActualSpendEl {
        DataBudgetsBudgetCalculatedSpendElActualSpendEl {
            amount: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetCalculatedSpendElActualSpendElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetCalculatedSpendElActualSpendElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetCalculatedSpendElActualSpendElRef {
        DataBudgetsBudgetCalculatedSpendElActualSpendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetCalculatedSpendElActualSpendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetCalculatedSpendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_spend: Option<ListField<DataBudgetsBudgetCalculatedSpendElActualSpendEl>>,
}

impl DataBudgetsBudgetCalculatedSpendEl {
    #[doc = "Set the field `actual_spend`.\n"]
    pub fn set_actual_spend(mut self, v: impl Into<ListField<DataBudgetsBudgetCalculatedSpendElActualSpendEl>>) -> Self {
        self.actual_spend = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetCalculatedSpendEl {
    type O = BlockAssignable<DataBudgetsBudgetCalculatedSpendEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetCalculatedSpendEl {}

impl BuildDataBudgetsBudgetCalculatedSpendEl {
    pub fn build(self) -> DataBudgetsBudgetCalculatedSpendEl {
        DataBudgetsBudgetCalculatedSpendEl { actual_spend: core::default::Default::default() }
    }
}

pub struct DataBudgetsBudgetCalculatedSpendElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetCalculatedSpendElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetCalculatedSpendElRef {
        DataBudgetsBudgetCalculatedSpendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetCalculatedSpendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actual_spend` after provisioning.\n"]
    pub fn actual_spend(&self) -> ListRef<DataBudgetsBudgetCalculatedSpendElActualSpendElRef> {
        ListRef::new(self.shared().clone(), format!("{}.actual_spend", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetCostFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}

impl DataBudgetsBudgetCostFilterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetCostFilterEl {
    type O = BlockAssignable<DataBudgetsBudgetCostFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetCostFilterEl {}

impl BuildDataBudgetsBudgetCostFilterEl {
    pub fn build(self) -> DataBudgetsBudgetCostFilterEl {
        DataBudgetsBudgetCostFilterEl {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetCostFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetCostFilterElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetCostFilterElRef {
        DataBudgetsBudgetCostFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetCostFilterElRef {
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
pub struct DataBudgetsBudgetCostTypesEl {
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

impl DataBudgetsBudgetCostTypesEl {
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

impl ToListMappable for DataBudgetsBudgetCostTypesEl {
    type O = BlockAssignable<DataBudgetsBudgetCostTypesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetCostTypesEl {}

impl BuildDataBudgetsBudgetCostTypesEl {
    pub fn build(self) -> DataBudgetsBudgetCostTypesEl {
        DataBudgetsBudgetCostTypesEl {
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

pub struct DataBudgetsBudgetCostTypesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetCostTypesElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetCostTypesElRef {
        DataBudgetsBudgetCostTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetCostTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_credit` after provisioning.\n"]
    pub fn include_credit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_credit", self.base))
    }

    #[doc = "Get a reference to the value of field `include_discount` after provisioning.\n"]
    pub fn include_discount(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_discount", self.base))
    }

    #[doc = "Get a reference to the value of field `include_other_subscription` after provisioning.\n"]
    pub fn include_other_subscription(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_other_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `include_recurring` after provisioning.\n"]
    pub fn include_recurring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_recurring", self.base))
    }

    #[doc = "Get a reference to the value of field `include_refund` after provisioning.\n"]
    pub fn include_refund(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_refund", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subscription` after provisioning.\n"]
    pub fn include_subscription(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `include_support` after provisioning.\n"]
    pub fn include_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_support", self.base))
    }

    #[doc = "Get a reference to the value of field `include_tax` after provisioning.\n"]
    pub fn include_tax(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_tax", self.base))
    }

    #[doc = "Get a reference to the value of field `include_upfront` after provisioning.\n"]
    pub fn include_upfront(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_upfront", self.base))
    }

    #[doc = "Get a reference to the value of field `use_amortized` after provisioning.\n"]
    pub fn use_amortized(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_amortized", self.base))
    }

    #[doc = "Get a reference to the value of field `use_blended` after provisioning.\n"]
    pub fn use_blended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_blended", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetNotificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comparison_operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_email_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_sns_topic_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_type: Option<PrimField<String>>,
}

impl DataBudgetsBudgetNotificationEl {
    #[doc = "Set the field `comparison_operator`.\n"]
    pub fn set_comparison_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparison_operator = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_type`.\n"]
    pub fn set_notification_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_type = Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_email_addresses`.\n"]
    pub fn set_subscriber_email_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subscriber_email_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_sns_topic_arns`.\n"]
    pub fn set_subscriber_sns_topic_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subscriber_sns_topic_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_type`.\n"]
    pub fn set_threshold_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.threshold_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetNotificationEl {
    type O = BlockAssignable<DataBudgetsBudgetNotificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetNotificationEl {}

impl BuildDataBudgetsBudgetNotificationEl {
    pub fn build(self) -> DataBudgetsBudgetNotificationEl {
        DataBudgetsBudgetNotificationEl {
            comparison_operator: core::default::Default::default(),
            notification_type: core::default::Default::default(),
            subscriber_email_addresses: core::default::Default::default(),
            subscriber_sns_topic_arns: core::default::Default::default(),
            threshold: core::default::Default::default(),
            threshold_type: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetNotificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetNotificationElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetNotificationElRef {
        DataBudgetsBudgetNotificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetNotificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subscriber_email_addresses` after provisioning.\n"]
    pub fn subscriber_email_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subscriber_email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `subscriber_sns_topic_arns` after provisioning.\n"]
    pub fn subscriber_sns_topic_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subscriber_sns_topic_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_type` after provisioning.\n"]
    pub fn threshold_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudgetPlannedLimitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl DataBudgetsBudgetPlannedLimitEl {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amount = Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for DataBudgetsBudgetPlannedLimitEl {
    type O = BlockAssignable<DataBudgetsBudgetPlannedLimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudgetPlannedLimitEl {}

impl BuildDataBudgetsBudgetPlannedLimitEl {
    pub fn build(self) -> DataBudgetsBudgetPlannedLimitEl {
        DataBudgetsBudgetPlannedLimitEl {
            amount: core::default::Default::default(),
            start_time: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

pub struct DataBudgetsBudgetPlannedLimitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBudgetsBudgetPlannedLimitElRef {
    fn new(shared: StackShared, base: String) -> DataBudgetsBudgetPlannedLimitElRef {
        DataBudgetsBudgetPlannedLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudgetPlannedLimitElRef {
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
