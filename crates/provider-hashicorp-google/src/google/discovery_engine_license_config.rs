use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineLicenseConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_renew: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    free_trial: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    license_config_id: PrimField<String>,
    license_count: PrimField<f64>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    subscription_term: PrimField<String>,
    subscription_tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<Vec<DiscoveryEngineLicenseConfigEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<Vec<DiscoveryEngineLicenseConfigStartDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineLicenseConfigTimeoutsEl>,
    dynamic: DiscoveryEngineLicenseConfigDynamic,
}
struct DiscoveryEngineLicenseConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineLicenseConfigData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineLicenseConfig(Rc<DiscoveryEngineLicenseConfig_>);
impl DiscoveryEngineLicenseConfig {
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
    #[doc = "Set the field `auto_renew`.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn set_auto_renew(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew = Some(v.into());
        self
    }
    #[doc = "Set the field `free_trial`.\nWhether the license config is for free trial."]
    pub fn set_free_trial(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().free_trial = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineLicenseConfigEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineLicenseConfigStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.start_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineLicenseConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `free_trial` after provisioning.\nWhether the license config is for free trial."]
    pub fn free_trial(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.free_trial", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `license_config_id` after provisioning.\nThe unique id of the license config."]
    pub fn license_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_count` after provisioning.\nNumber of licenses purchased."]
    pub fn license_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the license config. Values are of the format\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_term` after provisioning.\nSubscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub fn subscription_term(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_term", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_tier` after provisioning.\nSubscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub fn subscription_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> ListRef<DiscoveryEngineLicenseConfigEndDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.end_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> ListRef<DiscoveryEngineLicenseConfigStartDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.start_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineLicenseConfigTimeoutsElRef {
        DiscoveryEngineLicenseConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineLicenseConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineLicenseConfig {}
impl ToListMappable for DiscoveryEngineLicenseConfig {
    type O = ListRef<DiscoveryEngineLicenseConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineLicenseConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_license_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineLicenseConfig {
    pub tf_id: String,
    #[doc = "The unique id of the license config."]
    pub license_config_id: PrimField<String>,
    #[doc = "Number of licenses purchased."]
    pub license_count: PrimField<f64>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
    #[doc = "Subscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub subscription_term: PrimField<String>,
    #[doc = "Subscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub subscription_tier: PrimField<String>,
}
impl BuildDiscoveryEngineLicenseConfig {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineLicenseConfig {
        let out = DiscoveryEngineLicenseConfig(Rc::new(DiscoveryEngineLicenseConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineLicenseConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_renew: core::default::Default::default(),
                free_trial: core::default::Default::default(),
                id: core::default::Default::default(),
                license_config_id: self.license_config_id,
                license_count: self.license_count,
                location: self.location,
                project: core::default::Default::default(),
                subscription_term: self.subscription_term,
                subscription_tier: self.subscription_tier,
                end_date: core::default::Default::default(),
                start_date: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineLicenseConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineLicenseConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineLicenseConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `free_trial` after provisioning.\nWhether the license config is for free trial."]
    pub fn free_trial(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.free_trial", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `license_config_id` after provisioning.\nThe unique id of the license config."]
    pub fn license_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_count` after provisioning.\nNumber of licenses purchased."]
    pub fn license_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.license_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the license config. Values are of the format\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_term` after provisioning.\nSubscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub fn subscription_term(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_term", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_tier` after provisioning.\nSubscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub fn subscription_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> ListRef<DiscoveryEngineLicenseConfigEndDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.end_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> ListRef<DiscoveryEngineLicenseConfigStartDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.start_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineLicenseConfigTimeoutsElRef {
        DiscoveryEngineLicenseConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfigEndDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl DiscoveryEngineLicenseConfigEndDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineLicenseConfigEndDateEl {
    type O = BlockAssignable<DiscoveryEngineLicenseConfigEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineLicenseConfigEndDateEl {}
impl BuildDiscoveryEngineLicenseConfigEndDateEl {
    pub fn build(self) -> DiscoveryEngineLicenseConfigEndDateEl {
        DiscoveryEngineLicenseConfigEndDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineLicenseConfigEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineLicenseConfigEndDateElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineLicenseConfigEndDateElRef {
        DiscoveryEngineLicenseConfigEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineLicenseConfigEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfigStartDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl DiscoveryEngineLicenseConfigStartDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineLicenseConfigStartDateEl {
    type O = BlockAssignable<DiscoveryEngineLicenseConfigStartDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineLicenseConfigStartDateEl {}
impl BuildDiscoveryEngineLicenseConfigStartDateEl {
    pub fn build(self) -> DiscoveryEngineLicenseConfigStartDateEl {
        DiscoveryEngineLicenseConfigStartDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineLicenseConfigStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineLicenseConfigStartDateElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineLicenseConfigStartDateElRef {
        DiscoveryEngineLicenseConfigStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineLicenseConfigStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineLicenseConfigTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineLicenseConfigTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineLicenseConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineLicenseConfigTimeoutsEl {}
impl BuildDiscoveryEngineLicenseConfigTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineLicenseConfigTimeoutsEl {
        DiscoveryEngineLicenseConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineLicenseConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineLicenseConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineLicenseConfigTimeoutsElRef {
        DiscoveryEngineLicenseConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineLicenseConfigTimeoutsElRef {
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
struct DiscoveryEngineLicenseConfigDynamic {
    end_date: Option<DynamicBlock<DiscoveryEngineLicenseConfigEndDateEl>>,
    start_date: Option<DynamicBlock<DiscoveryEngineLicenseConfigStartDateEl>>,
}
