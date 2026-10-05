use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RuleGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_provenance: Option<PrimField<bool>>,
    folder_uid: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    interval_seconds: PrimField<f64>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<RuleGroupRuleEl>>,
    dynamic: RuleGroupDynamic,
}
struct RuleGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RuleGroupData>,
}
#[derive(Clone)]
pub struct RuleGroup(Rc<RuleGroup_>);
impl RuleGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
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
    #[doc = "Set the field `disable_provenance`.\nAllow modifying the rule group from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn set_disable_provenance(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_provenance = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<RuleGroupRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `disable_provenance` after provisioning.\nAllow modifying the rule group from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn disable_provenance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_provenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder_uid` after provisioning.\nThe UID of the folder that the group belongs to."]
    pub fn folder_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interval_seconds` after provisioning.\nThe interval, in seconds, at which all rules in the group are evaluated. If a group contains many rules, the rules are evaluated sequentially."]
    pub fn interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the rule group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<RuleGroupRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}
impl Referable for RuleGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RuleGroup {}
impl ToListMappable for RuleGroup {
    type O = ListRef<RuleGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RuleGroup_ {
    fn extract_resource_type(&self) -> String {
        "grafana_rule_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRuleGroup {
    pub tf_id: String,
    #[doc = "The UID of the folder that the group belongs to."]
    pub folder_uid: PrimField<String>,
    #[doc = "The interval, in seconds, at which all rules in the group are evaluated. If a group contains many rules, the rules are evaluated sequentially."]
    pub interval_seconds: PrimField<f64>,
    #[doc = "The name of the rule group."]
    pub name: PrimField<String>,
}
impl BuildRuleGroup {
    pub fn build(self, stack: &mut Stack) -> RuleGroup {
        let out = RuleGroup(Rc::new(RuleGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RuleGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                disable_provenance: core::default::Default::default(),
                folder_uid: self.folder_uid,
                id: core::default::Default::default(),
                interval_seconds: self.interval_seconds,
                name: self.name,
                org_id: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RuleGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RuleGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_provenance` after provisioning.\nAllow modifying the rule group from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn disable_provenance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_provenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder_uid` after provisioning.\nThe UID of the folder that the group belongs to."]
    pub fn folder_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interval_seconds` after provisioning.\nThe interval, in seconds, at which all rules in the group are evaluated. If a group contains many rules, the rules are evaluated sequentially."]
    pub fn interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the rule group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<RuleGroupRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct RuleGroupRuleElDataElRelativeTimeRangeEl {
    from: PrimField<f64>,
    to: PrimField<f64>,
}
impl RuleGroupRuleElDataElRelativeTimeRangeEl {}
impl ToListMappable for RuleGroupRuleElDataElRelativeTimeRangeEl {
    type O = BlockAssignable<RuleGroupRuleElDataElRelativeTimeRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRuleGroupRuleElDataElRelativeTimeRangeEl {
    #[doc = "The number of seconds in the past, relative to when the rule is evaluated, at which the time range begins."]
    pub from: PrimField<f64>,
    #[doc = "The number of seconds in the past, relative to when the rule is evaluated, at which the time range ends."]
    pub to: PrimField<f64>,
}
impl BuildRuleGroupRuleElDataElRelativeTimeRangeEl {
    pub fn build(self) -> RuleGroupRuleElDataElRelativeTimeRangeEl {
        RuleGroupRuleElDataElRelativeTimeRangeEl {
            from: self.from,
            to: self.to,
        }
    }
}
pub struct RuleGroupRuleElDataElRelativeTimeRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRuleElDataElRelativeTimeRangeElRef {
    fn new(shared: StackShared, base: String) -> RuleGroupRuleElDataElRelativeTimeRangeElRef {
        RuleGroupRuleElDataElRelativeTimeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RuleGroupRuleElDataElRelativeTimeRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from` after provisioning.\nThe number of seconds in the past, relative to when the rule is evaluated, at which the time range begins."]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }
    #[doc = "Get a reference to the value of field `to` after provisioning.\nThe number of seconds in the past, relative to when the rule is evaluated, at which the time range ends."]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}
#[derive(Serialize, Default)]
struct RuleGroupRuleElDataElDynamic {
    relative_time_range: Option<DynamicBlock<RuleGroupRuleElDataElRelativeTimeRangeEl>>,
}
#[derive(Serialize)]
pub struct RuleGroupRuleElDataEl {
    datasource_uid: PrimField<String>,
    model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_type: Option<PrimField<String>>,
    ref_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_time_range: Option<Vec<RuleGroupRuleElDataElRelativeTimeRangeEl>>,
    dynamic: RuleGroupRuleElDataElDynamic,
}
impl RuleGroupRuleElDataEl {
    #[doc = "Set the field `query_type`.\nAn optional identifier for the type of query being executed. Defaults to ``."]
    pub fn set_query_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_type = Some(v.into());
        self
    }
    #[doc = "Set the field `relative_time_range`.\n"]
    pub fn set_relative_time_range(
        mut self,
        v: impl Into<BlockAssignable<RuleGroupRuleElDataElRelativeTimeRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.relative_time_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.relative_time_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RuleGroupRuleElDataEl {
    type O = BlockAssignable<RuleGroupRuleElDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRuleGroupRuleElDataEl {
    #[doc = "The UID of the datasource being queried, or \"-100\" if this stage is an expression stage."]
    pub datasource_uid: PrimField<String>,
    #[doc = "Custom JSON data to send to the specified datasource when querying."]
    pub model: PrimField<String>,
    #[doc = "A unique string to identify this query stage within a rule."]
    pub ref_id: PrimField<String>,
}
impl BuildRuleGroupRuleElDataEl {
    pub fn build(self) -> RuleGroupRuleElDataEl {
        RuleGroupRuleElDataEl {
            datasource_uid: self.datasource_uid,
            model: self.model,
            query_type: core::default::Default::default(),
            ref_id: self.ref_id,
            relative_time_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RuleGroupRuleElDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRuleElDataElRef {
    fn new(shared: StackShared, base: String) -> RuleGroupRuleElDataElRef {
        RuleGroupRuleElDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RuleGroupRuleElDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `datasource_uid` after provisioning.\nThe UID of the datasource being queried, or \"-100\" if this stage is an expression stage."]
    pub fn datasource_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.datasource_uid", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nCustom JSON data to send to the specified datasource when querying."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `query_type` after provisioning.\nAn optional identifier for the type of query being executed. Defaults to ``."]
    pub fn query_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_type", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_id` after provisioning.\nA unique string to identify this query stage within a rule."]
    pub fn ref_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref_id", self.base))
    }
    #[doc = "Get a reference to the value of field `relative_time_range` after provisioning.\n"]
    pub fn relative_time_range(&self) -> ListRef<RuleGroupRuleElDataElRelativeTimeRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relative_time_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RuleGroupRuleElNotificationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_timings: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_point: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_wait: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute_timings: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_interval: Option<PrimField<String>>,
}
impl RuleGroupRuleElNotificationSettingsEl {
    #[doc = "Set the field `active_timings`.\nA list of time interval names to apply to alerts that match this policy to suppress them unless they are sent at the specified time. Supported in Grafana 12.1.0 and later"]
    pub fn set_active_timings(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.active_timings = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_point`.\nThe contact point to route notifications that match this rule to. Exactly one of `contact_point` or `policy` must be set."]
    pub fn set_contact_point(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_point = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by`.\nA list of alert labels to group alerts into notifications by. Use the special label `...` to group alerts by all labels, effectively disabling grouping. If empty, no grouping is used. If specified, requires labels 'alertname' and 'grafana_folder' to be included."]
    pub fn set_group_by(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group_by = Some(v.into());
        self
    }
    #[doc = "Set the field `group_interval`.\nMinimum time interval between two notifications for the same group. Default is 5 minutes."]
    pub fn set_group_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `group_wait`.\nTime to wait to buffer alerts of the same group before sending a notification. Default is 30 seconds."]
    pub fn set_group_wait(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_wait = Some(v.into());
        self
    }
    #[doc = "Set the field `mute_timings`.\nA list of mute timing names to apply to alerts that match this policy."]
    pub fn set_mute_timings(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mute_timings = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\nThe name of the notification policy to route notifications that match this rule through. Mutually exclusive with `contact_point` and all other fields in this block. Exactly one of `contact_point` or `policy` must be set."]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `repeat_interval`.\nMinimum time interval for re-sending a notification if an alert is still firing. Default is 4 hours."]
    pub fn set_repeat_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repeat_interval = Some(v.into());
        self
    }
}
impl ToListMappable for RuleGroupRuleElNotificationSettingsEl {
    type O = BlockAssignable<RuleGroupRuleElNotificationSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRuleGroupRuleElNotificationSettingsEl {}
impl BuildRuleGroupRuleElNotificationSettingsEl {
    pub fn build(self) -> RuleGroupRuleElNotificationSettingsEl {
        RuleGroupRuleElNotificationSettingsEl {
            active_timings: core::default::Default::default(),
            contact_point: core::default::Default::default(),
            group_by: core::default::Default::default(),
            group_interval: core::default::Default::default(),
            group_wait: core::default::Default::default(),
            mute_timings: core::default::Default::default(),
            policy: core::default::Default::default(),
            repeat_interval: core::default::Default::default(),
        }
    }
}
pub struct RuleGroupRuleElNotificationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRuleElNotificationSettingsElRef {
    fn new(shared: StackShared, base: String) -> RuleGroupRuleElNotificationSettingsElRef {
        RuleGroupRuleElNotificationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RuleGroupRuleElNotificationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_timings` after provisioning.\nA list of time interval names to apply to alerts that match this policy to suppress them unless they are sent at the specified time. Supported in Grafana 12.1.0 and later"]
    pub fn active_timings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_timings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `contact_point` after provisioning.\nThe contact point to route notifications that match this rule to. Exactly one of `contact_point` or `policy` must be set."]
    pub fn contact_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_point", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group_by` after provisioning.\nA list of alert labels to group alerts into notifications by. Use the special label `...` to group alerts by all labels, effectively disabling grouping. If empty, no grouping is used. If specified, requires labels 'alertname' and 'grafana_folder' to be included."]
    pub fn group_by(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group_by", self.base))
    }
    #[doc = "Get a reference to the value of field `group_interval` after provisioning.\nMinimum time interval between two notifications for the same group. Default is 5 minutes."]
    pub fn group_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group_wait` after provisioning.\nTime to wait to buffer alerts of the same group before sending a notification. Default is 30 seconds."]
    pub fn group_wait(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_wait", self.base))
    }
    #[doc = "Get a reference to the value of field `mute_timings` after provisioning.\nA list of mute timing names to apply to alerts that match this policy."]
    pub fn mute_timings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.mute_timings", self.base))
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe name of the notification policy to route notifications that match this rule through. Mutually exclusive with `contact_point` and all other fields in this block. Exactly one of `contact_point` or `policy` must be set."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `repeat_interval` after provisioning.\nMinimum time interval for re-sending a notification if an alert is still firing. Default is 4 hours."]
    pub fn repeat_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repeat_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RuleGroupRuleElRecordEl {
    from: PrimField<String>,
    metric: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_datasource_uid: Option<PrimField<String>>,
}
impl RuleGroupRuleElRecordEl {
    #[doc = "Set the field `target_datasource_uid`.\nThe UID of the datasource to write the metric to."]
    pub fn set_target_datasource_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_datasource_uid = Some(v.into());
        self
    }
}
impl ToListMappable for RuleGroupRuleElRecordEl {
    type O = BlockAssignable<RuleGroupRuleElRecordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRuleGroupRuleElRecordEl {
    #[doc = "The ref id of the query node in the data field to use as the source of the metric."]
    pub from: PrimField<String>,
    #[doc = "The name of the metric to write to."]
    pub metric: PrimField<String>,
}
impl BuildRuleGroupRuleElRecordEl {
    pub fn build(self) -> RuleGroupRuleElRecordEl {
        RuleGroupRuleElRecordEl {
            from: self.from,
            metric: self.metric,
            target_datasource_uid: core::default::Default::default(),
        }
    }
}
pub struct RuleGroupRuleElRecordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRuleElRecordElRef {
    fn new(shared: StackShared, base: String) -> RuleGroupRuleElRecordElRef {
        RuleGroupRuleElRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RuleGroupRuleElRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from` after provisioning.\nThe ref id of the query node in the data field to use as the source of the metric."]
    pub fn from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\nThe name of the metric to write to."]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }
    #[doc = "Get a reference to the value of field `target_datasource_uid` after provisioning.\nThe UID of the datasource to write the metric to."]
    pub fn target_datasource_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_datasource_uid", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct RuleGroupRuleElDynamic {
    data: Option<DynamicBlock<RuleGroupRuleElDataEl>>,
    notification_settings: Option<DynamicBlock<RuleGroupRuleElNotificationSettingsEl>>,
    record: Option<DynamicBlock<RuleGroupRuleElRecordEl>>,
}
#[derive(Serialize)]
pub struct RuleGroupRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exec_err_state: Option<PrimField<String>>,
    #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
    for_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_paused: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_firing_for: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    missing_series_evals_to_resolve: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_data_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<RuleGroupRuleElDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_settings: Option<Vec<RuleGroupRuleElNotificationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record: Option<Vec<RuleGroupRuleElRecordEl>>,
    dynamic: RuleGroupRuleElDynamic,
}
impl RuleGroupRuleEl {
    #[doc = "Set the field `annotations`.\nKey-value pairs of metadata to attach to the alert rule. They add additional information, such as a `summary` or `runbook_url`, to help identify and investigate alerts. The `__dashboardUid__` and `__panelId__` annotations, which link alerts to a panel, must be set together. Defaults to `map[]`."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\nThe `ref_id` of the query node in the `data` field to use as the alert condition."]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }
    #[doc = "Set the field `exec_err_state`.\nDescribes what state to enter when the rule's query is invalid and the rule cannot be executed. Options are OK, Error, KeepLast, and Alerting.  Defaults to Alerting if not set."]
    pub fn set_exec_err_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exec_err_state = Some(v.into());
        self
    }
    #[doc = "Set the field `for_`.\nThe amount of time for which the rule must be breached for the rule to be considered to be Firing. Before this time has elapsed, the rule is only considered to be Pending. Defaults to `0`."]
    pub fn set_for(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.for_ = Some(v.into());
        self
    }
    #[doc = "Set the field `is_paused`.\nSets whether the alert should be paused or not. Defaults to `false`."]
    pub fn set_is_paused(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_paused = Some(v.into());
        self
    }
    #[doc = "Set the field `keep_firing_for`.\nThe amount of time for which the rule will considered to be Recovering after initially Firing. Before this time has elapsed, the rule will continue to fire once it's been triggered."]
    pub fn set_keep_firing_for(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keep_firing_for = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nKey-value pairs to attach to the alert rule that can be used in matching, grouping, and routing. Defaults to `map[]`."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `missing_series_evals_to_resolve`.\nThe number of missing series evaluations that must occur before the rule is considered to be resolved."]
    pub fn set_missing_series_evals_to_resolve(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.missing_series_evals_to_resolve = Some(v.into());
        self
    }
    #[doc = "Set the field `no_data_state`.\nDescribes what state to enter when the rule's query returns No Data. Options are OK, NoData, KeepLast, and Alerting. Defaults to NoData if not set."]
    pub fn set_no_data_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_data_state = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\nThe unique identifier of the alert rule."]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<BlockAssignable<RuleGroupRuleElDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_settings`.\n"]
    pub fn set_notification_settings(
        mut self,
        v: impl Into<BlockAssignable<RuleGroupRuleElNotificationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `record`.\n"]
    pub fn set_record(mut self, v: impl Into<BlockAssignable<RuleGroupRuleElRecordEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RuleGroupRuleEl {
    type O = BlockAssignable<RuleGroupRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRuleGroupRuleEl {
    #[doc = "The name of the alert rule."]
    pub name: PrimField<String>,
}
impl BuildRuleGroupRuleEl {
    pub fn build(self) -> RuleGroupRuleEl {
        RuleGroupRuleEl {
            annotations: core::default::Default::default(),
            condition: core::default::Default::default(),
            exec_err_state: core::default::Default::default(),
            for_: core::default::Default::default(),
            is_paused: core::default::Default::default(),
            keep_firing_for: core::default::Default::default(),
            labels: core::default::Default::default(),
            missing_series_evals_to_resolve: core::default::Default::default(),
            name: self.name,
            no_data_state: core::default::Default::default(),
            uid: core::default::Default::default(),
            data: core::default::Default::default(),
            notification_settings: core::default::Default::default(),
            record: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RuleGroupRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RuleGroupRuleElRef {
    fn new(shared: StackShared, base: String) -> RuleGroupRuleElRef {
        RuleGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RuleGroupRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nKey-value pairs of metadata to attach to the alert rule. They add additional information, such as a `summary` or `runbook_url`, to help identify and investigate alerts. The `__dashboardUid__` and `__panelId__` annotations, which link alerts to a panel, must be set together. Defaults to `map[]`."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nThe `ref_id` of the query node in the `data` field to use as the alert condition."]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `exec_err_state` after provisioning.\nDescribes what state to enter when the rule's query is invalid and the rule cannot be executed. Options are OK, Error, KeepLast, and Alerting.  Defaults to Alerting if not set."]
    pub fn exec_err_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exec_err_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `for_` after provisioning.\nThe amount of time for which the rule must be breached for the rule to be considered to be Firing. Before this time has elapsed, the rule is only considered to be Pending. Defaults to `0`."]
    pub fn for_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.for", self.base))
    }
    #[doc = "Get a reference to the value of field `is_paused` after provisioning.\nSets whether the alert should be paused or not. Defaults to `false`."]
    pub fn is_paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_paused", self.base))
    }
    #[doc = "Get a reference to the value of field `keep_firing_for` after provisioning.\nThe amount of time for which the rule will considered to be Recovering after initially Firing. Before this time has elapsed, the rule will continue to fire once it's been triggered."]
    pub fn keep_firing_for(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keep_firing_for", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nKey-value pairs to attach to the alert rule that can be used in matching, grouping, and routing. Defaults to `map[]`."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `missing_series_evals_to_resolve` after provisioning.\nThe number of missing series evaluations that must occur before the rule is considered to be resolved."]
    pub fn missing_series_evals_to_resolve(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.missing_series_evals_to_resolve", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the alert rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `no_data_state` after provisioning.\nDescribes what state to enter when the rule's query returns No Data. Options are OK, NoData, KeepLast, and Alerting. Defaults to NoData if not set."]
    pub fn no_data_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_data_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the alert rule."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<RuleGroupRuleElDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `notification_settings` after provisioning.\n"]
    pub fn notification_settings(&self) -> ListRef<RuleGroupRuleElNotificationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record` after provisioning.\n"]
    pub fn record(&self) -> ListRef<RuleGroupRuleElRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.record", self.base))
    }
}
#[derive(Serialize, Default)]
struct RuleGroupDynamic {
    rule: Option<DynamicBlock<RuleGroupRuleEl>>,
}
