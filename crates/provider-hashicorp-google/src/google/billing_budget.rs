use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BillingBudgetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    billing_account: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_updates_rule: Option<Vec<BillingBudgetAllUpdatesRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Vec<BillingBudgetAmountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_filter: Option<Vec<BillingBudgetBudgetFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_rules: Option<Vec<BillingBudgetThresholdRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BillingBudgetTimeoutsEl>,
    dynamic: BillingBudgetDynamic,
}
struct BillingBudget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BillingBudgetData>,
}
#[derive(Clone)]
pub struct BillingBudget(Rc<BillingBudget_>);
impl BillingBudget {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ownership_scope`.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn set_ownership_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ownership_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `all_updates_rule`.\n"]
    pub fn set_all_updates_rule(
        self,
        v: impl Into<BlockAssignable<BillingBudgetAllUpdatesRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().all_updates_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.all_updates_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(self, v: impl Into<BlockAssignable<BillingBudgetAmountEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amount = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amount = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `budget_filter`.\n"]
    pub fn set_budget_filter(
        self,
        v: impl Into<BlockAssignable<BillingBudgetBudgetFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().budget_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.budget_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threshold_rules`.\n"]
    pub fn set_threshold_rules(
        self,
        v: impl Into<BlockAssignable<BillingBudgetThresholdRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threshold_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threshold_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BillingBudgetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\nID of the billing account to set a budget on."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the budget. The resource name\nimplies the scope of a budget. Values are of the form\nbillingAccounts/{billingAccountId}/budgets/{budgetId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ownership_scope` after provisioning.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn ownership_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_updates_rule` after provisioning.\n"]
    pub fn all_updates_rule(&self) -> ListRef<BillingBudgetAllUpdatesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_updates_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> ListRef<BillingBudgetAmountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amount", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `budget_filter` after provisioning.\n"]
    pub fn budget_filter(&self) -> ListRef<BillingBudgetBudgetFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.budget_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_rules` after provisioning.\n"]
    pub fn threshold_rules(&self) -> ListRef<BillingBudgetThresholdRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingBudgetTimeoutsElRef {
        BillingBudgetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BillingBudget {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BillingBudget {}
impl ToListMappable for BillingBudget {
    type O = ListRef<BillingBudgetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BillingBudget_ {
    fn extract_resource_type(&self) -> String {
        "google_billing_budget".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBillingBudget {
    pub tf_id: String,
    #[doc = "ID of the billing account to set a budget on."]
    pub billing_account: PrimField<String>,
}
impl BuildBillingBudget {
    pub fn build(self, stack: &mut Stack) -> BillingBudget {
        let out = BillingBudget(Rc::new(BillingBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BillingBudgetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_account: self.billing_account,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ownership_scope: core::default::Default::default(),
                all_updates_rule: core::default::Default::default(),
                amount: core::default::Default::default(),
                budget_filter: core::default::Default::default(),
                threshold_rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BillingBudgetRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BillingBudgetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\nID of the billing account to set a budget on."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the budget. The resource name\nimplies the scope of a budget. Values are of the form\nbillingAccounts/{billingAccountId}/budgets/{budgetId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ownership_scope` after provisioning.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn ownership_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `all_updates_rule` after provisioning.\n"]
    pub fn all_updates_rule(&self) -> ListRef<BillingBudgetAllUpdatesRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_updates_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> ListRef<BillingBudgetAmountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amount", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `budget_filter` after provisioning.\n"]
    pub fn budget_filter(&self) -> ListRef<BillingBudgetBudgetFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.budget_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_rules` after provisioning.\n"]
    pub fn threshold_rules(&self) -> ListRef<BillingBudgetThresholdRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingBudgetTimeoutsElRef {
        BillingBudgetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BillingBudgetAllUpdatesRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_default_iam_recipients: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_project_level_recipients: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_notification_channels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version: Option<PrimField<String>>,
}
impl BillingBudgetAllUpdatesRuleEl {
    #[doc = "Set the field `disable_default_iam_recipients`.\nBoolean. When set to true, disables default notifications sent\nwhen a threshold is exceeded. Default recipients are\nthose with Billing Account Administrators and Billing\nAccount Users IAM roles for the target account."]
    pub fn set_disable_default_iam_recipients(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_default_iam_recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_project_level_recipients`.\nWhen set to true, and when the budget has a single project configured,\nnotifications will be sent to project level recipients of that project.\nThis field will be ignored if the budget has multiple or no project configured.\n\nCurrently, project level recipients are the users with Owner role on a cloud project."]
    pub fn set_enable_project_level_recipients(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_project_level_recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_notification_channels`.\nThe full resource name of a monitoring notification\nchannel in the form\nprojects/{project_id}/notificationChannels/{channel_id}.\nA maximum of 5 channels are allowed."]
    pub fn set_monitoring_notification_channels(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.monitoring_notification_channels = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_topic`.\nThe name of the Cloud Pub/Sub topic where budget related\nmessages will be published, in the form\nprojects/{project_id}/topics/{topic_id}. Updates are sent\nat regular intervals to the topic."]
    pub fn set_pubsub_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pubsub_topic = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_version`.\nThe schema version of the notification. Only \"1.0\" is\naccepted. It represents the JSON schema as defined in\nhttps://cloud.google.com/billing/docs/how-to/budgets#notification_format."]
    pub fn set_schema_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_version = Some(v.into());
        self
    }
}
impl ToListMappable for BillingBudgetAllUpdatesRuleEl {
    type O = BlockAssignable<BillingBudgetAllUpdatesRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetAllUpdatesRuleEl {}
impl BuildBillingBudgetAllUpdatesRuleEl {
    pub fn build(self) -> BillingBudgetAllUpdatesRuleEl {
        BillingBudgetAllUpdatesRuleEl {
            disable_default_iam_recipients: core::default::Default::default(),
            enable_project_level_recipients: core::default::Default::default(),
            monitoring_notification_channels: core::default::Default::default(),
            pubsub_topic: core::default::Default::default(),
            schema_version: core::default::Default::default(),
        }
    }
}
pub struct BillingBudgetAllUpdatesRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetAllUpdatesRuleElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetAllUpdatesRuleElRef {
        BillingBudgetAllUpdatesRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetAllUpdatesRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_default_iam_recipients` after provisioning.\nBoolean. When set to true, disables default notifications sent\nwhen a threshold is exceeded. Default recipients are\nthose with Billing Account Administrators and Billing\nAccount Users IAM roles for the target account."]
    pub fn disable_default_iam_recipients(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_default_iam_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_project_level_recipients` after provisioning.\nWhen set to true, and when the budget has a single project configured,\nnotifications will be sent to project level recipients of that project.\nThis field will be ignored if the budget has multiple or no project configured.\n\nCurrently, project level recipients are the users with Owner role on a cloud project."]
    pub fn enable_project_level_recipients(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_project_level_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_notification_channels` after provisioning.\nThe full resource name of a monitoring notification\nchannel in the form\nprojects/{project_id}/notificationChannels/{channel_id}.\nA maximum of 5 channels are allowed."]
    pub fn monitoring_notification_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_notification_channels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe name of the Cloud Pub/Sub topic where budget related\nmessages will be published, in the form\nprojects/{project_id}/topics/{topic_id}. Updates are sent\nat regular intervals to the topic."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_version` after provisioning.\nThe schema version of the notification. Only \"1.0\" is\naccepted. It represents the JSON schema as defined in\nhttps://cloud.google.com/billing/docs/how-to/budgets#notification_format."]
    pub fn schema_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BillingBudgetAmountElSpecifiedAmountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<PrimField<String>>,
}
impl BillingBudgetAmountElSpecifiedAmountEl {
    #[doc = "Set the field `currency_code`.\nThe 3-letter currency code defined in ISO 4217."]
    pub fn set_currency_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.currency_code = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nNumber of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999\ninclusive. If units is positive, nanos must be positive or\nzero. If units is zero, nanos can be positive, zero, or\nnegative. If units is negative, nanos must be negative or\nzero. For example $-1.75 is represented as units=-1 and\nnanos=-750,000,000."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `units`.\nThe whole units of the amount. For example if currencyCode\nis \"USD\", then 1 unit is one US dollar."]
    pub fn set_units(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.units = Some(v.into());
        self
    }
}
impl ToListMappable for BillingBudgetAmountElSpecifiedAmountEl {
    type O = BlockAssignable<BillingBudgetAmountElSpecifiedAmountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetAmountElSpecifiedAmountEl {}
impl BuildBillingBudgetAmountElSpecifiedAmountEl {
    pub fn build(self) -> BillingBudgetAmountElSpecifiedAmountEl {
        BillingBudgetAmountElSpecifiedAmountEl {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}
pub struct BillingBudgetAmountElSpecifiedAmountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetAmountElSpecifiedAmountElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetAmountElSpecifiedAmountElRef {
        BillingBudgetAmountElSpecifiedAmountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetAmountElSpecifiedAmountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\nThe 3-letter currency code defined in ISO 4217."]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.currency_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nNumber of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999\ninclusive. If units is positive, nanos must be positive or\nzero. If units is zero, nanos can be positive, zero, or\nnegative. If units is negative, nanos must be negative or\nzero. For example $-1.75 is represented as units=-1 and\nnanos=-750,000,000."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `units` after provisioning.\nThe whole units of the amount. For example if currencyCode\nis \"USD\", then 1 unit is one US dollar."]
    pub fn units(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}
#[derive(Serialize, Default)]
struct BillingBudgetAmountElDynamic {
    specified_amount: Option<DynamicBlock<BillingBudgetAmountElSpecifiedAmountEl>>,
}
#[derive(Serialize)]
pub struct BillingBudgetAmountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_period_amount: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specified_amount: Option<Vec<BillingBudgetAmountElSpecifiedAmountEl>>,
    dynamic: BillingBudgetAmountElDynamic,
}
impl BillingBudgetAmountEl {
    #[doc = "Set the field `last_period_amount`.\nConfigures a budget amount that is automatically set to 100% of\nlast period's spend.\nBoolean. Set value to true to use. Do not set to false, instead\nuse the 'specified_amount' block."]
    pub fn set_last_period_amount(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.last_period_amount = Some(v.into());
        self
    }
    #[doc = "Set the field `specified_amount`.\n"]
    pub fn set_specified_amount(
        mut self,
        v: impl Into<BlockAssignable<BillingBudgetAmountElSpecifiedAmountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.specified_amount = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.specified_amount = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BillingBudgetAmountEl {
    type O = BlockAssignable<BillingBudgetAmountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetAmountEl {}
impl BuildBillingBudgetAmountEl {
    pub fn build(self) -> BillingBudgetAmountEl {
        BillingBudgetAmountEl {
            last_period_amount: core::default::Default::default(),
            specified_amount: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BillingBudgetAmountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetAmountElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetAmountElRef {
        BillingBudgetAmountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetAmountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_period_amount` after provisioning.\nConfigures a budget amount that is automatically set to 100% of\nlast period's spend.\nBoolean. Set value to true to use. Do not set to false, instead\nuse the 'specified_amount' block."]
    pub fn last_period_amount(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_period_amount", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `specified_amount` after provisioning.\n"]
    pub fn specified_amount(&self) -> ListRef<BillingBudgetAmountElSpecifiedAmountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specified_amount", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl BillingBudgetBudgetFilterElCustomPeriodElEndDateEl {}
impl ToListMappable for BillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
    type O = BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodElEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc = "Month of a year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc = "Year of the date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}
impl BuildBillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
    pub fn build(self) -> BillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
        BillingBudgetBudgetFilterElCustomPeriodElEndDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef {
        BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct BillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl BillingBudgetBudgetFilterElCustomPeriodElStartDateEl {}
impl ToListMappable for BillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
    type O = BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodElStartDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc = "Month of a year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc = "Year of the date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}
impl BuildBillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
    pub fn build(self) -> BillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
        BillingBudgetBudgetFilterElCustomPeriodElStartDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef {
        BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize, Default)]
struct BillingBudgetBudgetFilterElCustomPeriodElDynamic {
    end_date: Option<DynamicBlock<BillingBudgetBudgetFilterElCustomPeriodElEndDateEl>>,
    start_date: Option<DynamicBlock<BillingBudgetBudgetFilterElCustomPeriodElStartDateEl>>,
}
#[derive(Serialize)]
pub struct BillingBudgetBudgetFilterElCustomPeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<Vec<BillingBudgetBudgetFilterElCustomPeriodElEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<Vec<BillingBudgetBudgetFilterElCustomPeriodElStartDateEl>>,
    dynamic: BillingBudgetBudgetFilterElCustomPeriodElDynamic,
}
impl BillingBudgetBudgetFilterElCustomPeriodEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl Into<BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodElEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl Into<BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodElStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BillingBudgetBudgetFilterElCustomPeriodEl {
    type O = BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetBudgetFilterElCustomPeriodEl {}
impl BuildBillingBudgetBudgetFilterElCustomPeriodEl {
    pub fn build(self) -> BillingBudgetBudgetFilterElCustomPeriodEl {
        BillingBudgetBudgetFilterElCustomPeriodEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BillingBudgetBudgetFilterElCustomPeriodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetBudgetFilterElCustomPeriodElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetBudgetFilterElCustomPeriodElRef {
        BillingBudgetBudgetFilterElCustomPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetBudgetFilterElCustomPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> ListRef<BillingBudgetBudgetFilterElCustomPeriodElEndDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> ListRef<BillingBudgetBudgetFilterElCustomPeriodElStartDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
}
#[derive(Serialize, Default)]
struct BillingBudgetBudgetFilterElDynamic {
    custom_period: Option<DynamicBlock<BillingBudgetBudgetFilterElCustomPeriodEl>>,
}
#[derive(Serialize)]
pub struct BillingBudgetBudgetFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_types_treatment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_ancestors: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subaccounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_period: Option<Vec<BillingBudgetBudgetFilterElCustomPeriodEl>>,
    dynamic: BillingBudgetBudgetFilterElDynamic,
}
impl BillingBudgetBudgetFilterEl {
    #[doc = "Set the field `calendar_period`.\nA CalendarPeriod represents the abstract concept of a recurring time period that has a\ncanonical start. Grammatically, \"the start of the current CalendarPeriod\".\nAll calendar times begin at 12 AM US and Canadian Pacific Time (UTC-8).\n\nExactly one of 'calendar_period', 'custom_period' must be provided. Possible values: [\"MONTH\", \"QUARTER\", \"YEAR\", \"CALENDAR_PERIOD_UNSPECIFIED\"]"]
    pub fn set_calendar_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.calendar_period = Some(v.into());
        self
    }
    #[doc = "Set the field `credit_types`.\nOptional. If creditTypesTreatment is INCLUDE_SPECIFIED_CREDITS,\nthis is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See a list of acceptable credit type values.\nIf creditTypesTreatment is not INCLUDE_SPECIFIED_CREDITS, this field must be empty."]
    pub fn set_credit_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.credit_types = Some(v.into());
        self
    }
    #[doc = "Set the field `credit_types_treatment`.\nSpecifies how credits should be treated when determining spend\nfor threshold calculations. Default value: \"INCLUDE_ALL_CREDITS\" Possible values: [\"INCLUDE_ALL_CREDITS\", \"EXCLUDE_ALL_CREDITS\", \"INCLUDE_SPECIFIED_CREDITS\"]"]
    pub fn set_credit_types_treatment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credit_types_treatment = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA single label and value pair specifying that usage from only\nthis set of labeled resources should be included in the budget."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `projects`.\nA set of projects of the form projects/{project_number},\nspecifying that usage from only this set of projects should be\nincluded in the budget. If omitted, the report will include\nall usage for the billing account, regardless of which project\nthe usage occurred on."]
    pub fn set_projects(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.projects = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_ancestors`.\nA set of folder and organization names of the form folders/{folderId} or organizations/{organizationId},\nspecifying that usage from only this set of folders and organizations should be included in the budget.\nIf omitted, the budget includes all usage that the billing account pays for. If the folder or organization\ncontains projects that are paid for by a different Cloud Billing account, the budget doesn't apply to those projects."]
    pub fn set_resource_ancestors(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resource_ancestors = Some(v.into());
        self
    }
    #[doc = "Set the field `services`.\nA set of services of the form services/{service_id},\nspecifying that usage from only this set of services should be\nincluded in the budget. If omitted, the report will include\nusage for all the services. The service names are available\nthrough the Catalog API:\nhttps://cloud.google.com/billing/v1/how-tos/catalog-api."]
    pub fn set_services(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.services = Some(v.into());
        self
    }
    #[doc = "Set the field `subaccounts`.\nA set of subaccounts of the form billingAccounts/{account_id},\nspecifying that usage from only this set of subaccounts should\nbe included in the budget. If a subaccount is set to the name of\nthe parent account, usage from the parent account will be included.\nIf the field is omitted, the report will include usage from the parent\naccount and all subaccounts, if they exist."]
    pub fn set_subaccounts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.subaccounts = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_period`.\n"]
    pub fn set_custom_period(
        mut self,
        v: impl Into<BlockAssignable<BillingBudgetBudgetFilterElCustomPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_period = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BillingBudgetBudgetFilterEl {
    type O = BlockAssignable<BillingBudgetBudgetFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetBudgetFilterEl {}
impl BuildBillingBudgetBudgetFilterEl {
    pub fn build(self) -> BillingBudgetBudgetFilterEl {
        BillingBudgetBudgetFilterEl {
            calendar_period: core::default::Default::default(),
            credit_types: core::default::Default::default(),
            credit_types_treatment: core::default::Default::default(),
            labels: core::default::Default::default(),
            projects: core::default::Default::default(),
            resource_ancestors: core::default::Default::default(),
            services: core::default::Default::default(),
            subaccounts: core::default::Default::default(),
            custom_period: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BillingBudgetBudgetFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetBudgetFilterElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetBudgetFilterElRef {
        BillingBudgetBudgetFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetBudgetFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `calendar_period` after provisioning.\nA CalendarPeriod represents the abstract concept of a recurring time period that has a\ncanonical start. Grammatically, \"the start of the current CalendarPeriod\".\nAll calendar times begin at 12 AM US and Canadian Pacific Time (UTC-8).\n\nExactly one of 'calendar_period', 'custom_period' must be provided. Possible values: [\"MONTH\", \"QUARTER\", \"YEAR\", \"CALENDAR_PERIOD_UNSPECIFIED\"]"]
    pub fn calendar_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.calendar_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `credit_types` after provisioning.\nOptional. If creditTypesTreatment is INCLUDE_SPECIFIED_CREDITS,\nthis is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See a list of acceptable credit type values.\nIf creditTypesTreatment is not INCLUDE_SPECIFIED_CREDITS, this field must be empty."]
    pub fn credit_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.credit_types", self.base))
    }
    #[doc = "Get a reference to the value of field `credit_types_treatment` after provisioning.\nSpecifies how credits should be treated when determining spend\nfor threshold calculations. Default value: \"INCLUDE_ALL_CREDITS\" Possible values: [\"INCLUDE_ALL_CREDITS\", \"EXCLUDE_ALL_CREDITS\", \"INCLUDE_SPECIFIED_CREDITS\"]"]
    pub fn credit_types_treatment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credit_types_treatment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA single label and value pair specifying that usage from only\nthis set of labeled resources should be included in the budget."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `projects` after provisioning.\nA set of projects of the form projects/{project_number},\nspecifying that usage from only this set of projects should be\nincluded in the budget. If omitted, the report will include\nall usage for the billing account, regardless of which project\nthe usage occurred on."]
    pub fn projects(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.projects", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_ancestors` after provisioning.\nA set of folder and organization names of the form folders/{folderId} or organizations/{organizationId},\nspecifying that usage from only this set of folders and organizations should be included in the budget.\nIf omitted, the budget includes all usage that the billing account pays for. If the folder or organization\ncontains projects that are paid for by a different Cloud Billing account, the budget doesn't apply to those projects."]
    pub fn resource_ancestors(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_ancestors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services` after provisioning.\nA set of services of the form services/{service_id},\nspecifying that usage from only this set of services should be\nincluded in the budget. If omitted, the report will include\nusage for all the services. The service names are available\nthrough the Catalog API:\nhttps://cloud.google.com/billing/v1/how-tos/catalog-api."]
    pub fn services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.services", self.base))
    }
    #[doc = "Get a reference to the value of field `subaccounts` after provisioning.\nA set of subaccounts of the form billingAccounts/{account_id},\nspecifying that usage from only this set of subaccounts should\nbe included in the budget. If a subaccount is set to the name of\nthe parent account, usage from the parent account will be included.\nIf the field is omitted, the report will include usage from the parent\naccount and all subaccounts, if they exist."]
    pub fn subaccounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subaccounts", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_period` after provisioning.\n"]
    pub fn custom_period(&self) -> ListRef<BillingBudgetBudgetFilterElCustomPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BillingBudgetThresholdRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    spend_basis: Option<PrimField<String>>,
    threshold_percent: PrimField<f64>,
}
impl BillingBudgetThresholdRulesEl {
    #[doc = "Set the field `spend_basis`.\nThe type of basis used to determine if spend has passed\nthe threshold. Default value: \"CURRENT_SPEND\" Possible values: [\"CURRENT_SPEND\", \"FORECASTED_SPEND\"]"]
    pub fn set_spend_basis(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spend_basis = Some(v.into());
        self
    }
}
impl ToListMappable for BillingBudgetThresholdRulesEl {
    type O = BlockAssignable<BillingBudgetThresholdRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetThresholdRulesEl {
    #[doc = "Send an alert when this threshold is exceeded. This is a\n1.0-based percentage, so 0.5 = 50%. Must be >= 0."]
    pub threshold_percent: PrimField<f64>,
}
impl BuildBillingBudgetThresholdRulesEl {
    pub fn build(self) -> BillingBudgetThresholdRulesEl {
        BillingBudgetThresholdRulesEl {
            spend_basis: core::default::Default::default(),
            threshold_percent: self.threshold_percent,
        }
    }
}
pub struct BillingBudgetThresholdRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetThresholdRulesElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetThresholdRulesElRef {
        BillingBudgetThresholdRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetThresholdRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `spend_basis` after provisioning.\nThe type of basis used to determine if spend has passed\nthe threshold. Default value: \"CURRENT_SPEND\" Possible values: [\"CURRENT_SPEND\", \"FORECASTED_SPEND\"]"]
    pub fn spend_basis(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spend_basis", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold_percent` after provisioning.\nSend an alert when this threshold is exceeded. This is a\n1.0-based percentage, so 0.5 = 50%. Must be >= 0."]
    pub fn threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BillingBudgetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BillingBudgetTimeoutsEl {
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
impl ToListMappable for BillingBudgetTimeoutsEl {
    type O = BlockAssignable<BillingBudgetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingBudgetTimeoutsEl {}
impl BuildBillingBudgetTimeoutsEl {
    pub fn build(self) -> BillingBudgetTimeoutsEl {
        BillingBudgetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BillingBudgetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingBudgetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BillingBudgetTimeoutsElRef {
        BillingBudgetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingBudgetTimeoutsElRef {
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
struct BillingBudgetDynamic {
    all_updates_rule: Option<DynamicBlock<BillingBudgetAllUpdatesRuleEl>>,
    amount: Option<DynamicBlock<BillingBudgetAmountEl>>,
    budget_filter: Option<DynamicBlock<BillingBudgetBudgetFilterEl>>,
    threshold_rules: Option<DynamicBlock<BillingBudgetThresholdRulesEl>>,
}
