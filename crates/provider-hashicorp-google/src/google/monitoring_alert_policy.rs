use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MonitoringAlertPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    combiner: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_channels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert_strategy: Option<Vec<MonitoringAlertPolicyAlertStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<MonitoringAlertPolicyConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<Vec<MonitoringAlertPolicyDocumentationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MonitoringAlertPolicyTimeoutsEl>,
    dynamic: MonitoringAlertPolicyDynamic,
}
struct MonitoringAlertPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitoringAlertPolicyData>,
}
#[derive(Clone)]
pub struct MonitoringAlertPolicy(Rc<MonitoringAlertPolicy_>);
impl MonitoringAlertPolicy {
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
    #[doc = "Set the field `enabled`.\nWhether or not the policy is enabled. The default is true."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_channels`.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn set_notification_channels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_channels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn set_severity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().severity = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn set_user_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `alert_strategy`.\n"]
    pub fn set_alert_strategy(
        self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyAlertStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alert_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alert_strategy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `documentation`.\n"]
    pub fn set_documentation(
        self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyDocumentationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().documentation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.documentation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MonitoringAlertPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `combiner` after provisioning.\nHow to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub fn combiner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.combiner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_record` after provisioning.\nA read-only record of the creation of the alerting policy.\nIf provided in a call to create or update, this field will\nbe ignored."]
    pub fn creation_record(&self) -> ListRef<MonitoringAlertPolicyCreationRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.creation_record", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the policy is enabled. The default is true."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this policy.\nIts syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_channels` after provisioning.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alert_strategy` after provisioning.\n"]
    pub fn alert_strategy(&self) -> ListRef<MonitoringAlertPolicyAlertStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alert_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<MonitoringAlertPolicyConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> ListRef<MonitoringAlertPolicyDocumentationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringAlertPolicyTimeoutsElRef {
        MonitoringAlertPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MonitoringAlertPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MonitoringAlertPolicy {}
impl ToListMappable for MonitoringAlertPolicy {
    type O = ListRef<MonitoringAlertPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MonitoringAlertPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_monitoring_alert_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMonitoringAlertPolicy {
    pub tf_id: String,
    #[doc = "How to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub combiner: PrimField<String>,
    #[doc = "A short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub display_name: PrimField<String>,
}
impl BuildMonitoringAlertPolicy {
    pub fn build(self, stack: &mut Stack) -> MonitoringAlertPolicy {
        let out = MonitoringAlertPolicy(Rc::new(MonitoringAlertPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitoringAlertPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                combiner: self.combiner,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                notification_channels: core::default::Default::default(),
                project: core::default::Default::default(),
                severity: core::default::Default::default(),
                user_labels: core::default::Default::default(),
                alert_strategy: core::default::Default::default(),
                conditions: core::default::Default::default(),
                documentation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MonitoringAlertPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MonitoringAlertPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `combiner` after provisioning.\nHow to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub fn combiner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.combiner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_record` after provisioning.\nA read-only record of the creation of the alerting policy.\nIf provided in a call to create or update, this field will\nbe ignored."]
    pub fn creation_record(&self) -> ListRef<MonitoringAlertPolicyCreationRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.creation_record", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the policy is enabled. The default is true."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this policy.\nIts syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_channels` after provisioning.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alert_strategy` after provisioning.\n"]
    pub fn alert_strategy(&self) -> ListRef<MonitoringAlertPolicyAlertStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alert_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<MonitoringAlertPolicyConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> ListRef<MonitoringAlertPolicyDocumentationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringAlertPolicyTimeoutsElRef {
        MonitoringAlertPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyCreationRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mutate_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutated_by: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyCreationRecordEl {
    #[doc = "Set the field `mutate_time`.\n"]
    pub fn set_mutate_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mutate_time = Some(v.into());
        self
    }
    #[doc = "Set the field `mutated_by`.\n"]
    pub fn set_mutated_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mutated_by = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyCreationRecordEl {
    type O = BlockAssignable<MonitoringAlertPolicyCreationRecordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyCreationRecordEl {}
impl BuildMonitoringAlertPolicyCreationRecordEl {
    pub fn build(self) -> MonitoringAlertPolicyCreationRecordEl {
        MonitoringAlertPolicyCreationRecordEl {
            mutate_time: core::default::Default::default(),
            mutated_by: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyCreationRecordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyCreationRecordElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyCreationRecordElRef {
        MonitoringAlertPolicyCreationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyCreationRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mutate_time` after provisioning.\n"]
    pub fn mutate_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mutate_time", self.base))
    }
    #[doc = "Get a reference to the value of field `mutated_by` after provisioning.\n"]
    pub fn mutated_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mutated_by", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_channel_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    renotify_interval: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
    #[doc = "Set the field `notification_channel_names`.\nThe notification channels that these settings apply to. Each of these\ncorrespond to the name field in one of the NotificationChannel objects\nreferenced in the notification_channels field of this AlertPolicy. The format is\n'projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]'"]
    pub fn set_notification_channel_names(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.notification_channel_names = Some(v.into());
        self
    }
    #[doc = "Set the field `renotify_interval`.\nThe frequency at which to send reminder notifications for open incidents."]
    pub fn set_renotify_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.renotify_interval = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
    type O = BlockAssignable<MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {}
impl BuildMonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
    pub fn build(self) -> MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
        MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl {
            notification_channel_names: core::default::Default::default(),
            renotify_interval: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef {
        MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `notification_channel_names` after provisioning.\nThe notification channels that these settings apply to. Each of these\ncorrespond to the name field in one of the NotificationChannel objects\nreferenced in the notification_channels field of this AlertPolicy. The format is\n'projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channel_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_channel_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `renotify_interval` after provisioning.\nThe frequency at which to send reminder notifications for open incidents."]
    pub fn renotify_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renotify_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
    #[doc = "Set the field `period`.\nNot more than one notification per period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"60.5s\"."]
    pub fn set_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.period = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
    type O = BlockAssignable<MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {}
impl BuildMonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
    pub fn build(self) -> MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
        MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl {
            period: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef {
        MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\nNot more than one notification per period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"60.5s\"."]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyAlertStrategyElDynamic {
    notification_channel_strategy:
        Option<DynamicBlock<MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl>>,
    notification_rate_limit:
        Option<DynamicBlock<MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyAlertStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_close: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_prompts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_channel_strategy:
        Option<Vec<MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_rate_limit:
        Option<Vec<MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl>>,
    dynamic: MonitoringAlertPolicyAlertStrategyElDynamic,
}
impl MonitoringAlertPolicyAlertStrategyEl {
    #[doc = "Set the field `auto_close`.\nIf an alert policy that was active has no data for this long, any open incidents will close."]
    pub fn set_auto_close(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_close = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_prompts`.\nControl when notifications will be sent out. Possible values: [\"NOTIFICATION_PROMPT_UNSPECIFIED\", \"OPENED\", \"CLOSED\"]"]
    pub fn set_notification_prompts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.notification_prompts = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_channel_strategy`.\n"]
    pub fn set_notification_channel_strategy(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_channel_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_channel_strategy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_rate_limit`.\n"]
    pub fn set_notification_rate_limit(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyAlertStrategyElNotificationRateLimitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_rate_limit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_rate_limit = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyAlertStrategyEl {
    type O = BlockAssignable<MonitoringAlertPolicyAlertStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyAlertStrategyEl {}
impl BuildMonitoringAlertPolicyAlertStrategyEl {
    pub fn build(self) -> MonitoringAlertPolicyAlertStrategyEl {
        MonitoringAlertPolicyAlertStrategyEl {
            auto_close: core::default::Default::default(),
            notification_prompts: core::default::Default::default(),
            notification_channel_strategy: core::default::Default::default(),
            notification_rate_limit: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyAlertStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyAlertStrategyElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyAlertStrategyElRef {
        MonitoringAlertPolicyAlertStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyAlertStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_close` after provisioning.\nIf an alert policy that was active has no data for this long, any open incidents will close."]
    pub fn auto_close(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_close", self.base))
    }
    #[doc = "Get a reference to the value of field `notification_prompts` after provisioning.\nControl when notifications will be sent out. Possible values: [\"NOTIFICATION_PROMPT_UNSPECIFIED\", \"OPENED\", \"CLOSED\"]"]
    pub fn notification_prompts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_prompts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notification_channel_strategy` after provisioning.\n"]
    pub fn notification_channel_strategy(
        &self,
    ) -> ListRef<MonitoringAlertPolicyAlertStrategyElNotificationChannelStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_channel_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notification_rate_limit` after provisioning.\n"]
    pub fn notification_rate_limit(
        &self,
    ) -> ListRef<MonitoringAlertPolicyAlertStrategyElNotificationRateLimitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_rate_limit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_series_reducer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_series_aligner: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
    #[doc = "Set the field `alignment_period`.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn set_alignment_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alignment_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_series_reducer`.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn set_cross_series_reducer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_series_reducer = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by_fields`.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn set_group_by_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group_by_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `per_series_aligner`.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn set_per_series_aligner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.per_series_aligner = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
        MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl {
            alignment_period: core::default::Default::default(),
            cross_series_reducer: core::default::Default::default(),
            group_by_fields: core::default::Default::default(),
            per_series_aligner: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef {
        MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alignment_period` after provisioning.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn alignment_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alignment_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_series_reducer` after provisioning.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn cross_series_reducer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_series_reducer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group_by_fields` after provisioning.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn group_by_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_by_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_series_aligner` after provisioning.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn per_series_aligner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_series_aligner", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
    #[doc = "Set the field `count`.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
        MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl {
            count: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef {
        MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElConditionAbsentElDynamic {
    aggregations:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl>>,
    trigger: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionAbsentEl {
    duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregations: Option<Vec<MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Vec<MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl>>,
    dynamic: MonitoringAlertPolicyConditionsElConditionAbsentElDynamic,
}
impl MonitoringAlertPolicyConditionsElConditionAbsentEl {
    #[doc = "Set the field `filter`.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `aggregations`.\n"]
    pub fn set_aggregations(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aggregations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentElTriggerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionAbsentEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionAbsentEl {
    #[doc = "The amount of time that a time series must\nfail to report new data to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g. 60s, 120s, or 300s\n--are supported."]
    pub duration: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionAbsentEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionAbsentEl {
        MonitoringAlertPolicyConditionsElConditionAbsentEl {
            duration: self.duration,
            filter: core::default::Default::default(),
            aggregations: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionAbsentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionAbsentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionAbsentElRef {
        MonitoringAlertPolicyConditionsElConditionAbsentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionAbsentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nfail to report new data to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g. 60s, 120s, or 300s\n--are supported."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `aggregations` after provisioning.\n"]
    pub fn aggregations(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionAbsentElAggregationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aggregations", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionAbsentElTriggerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionMatchedLogEl {
    filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_extractors: Option<RecField<PrimField<String>>>,
}
impl MonitoringAlertPolicyConditionsElConditionMatchedLogEl {
    #[doc = "Set the field `label_extractors`.\nA map from a label key to an extractor expression, which is used to\nextract the value for this label key. Each entry in this map is\na specification for how data should be extracted from log entries that\nmatch filter. Each combination of extracted values is treated as\na separate rule for the purposes of triggering notifications.\nLabel keys and corresponding values can be used in notifications\ngenerated by this condition."]
    pub fn set_label_extractors(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.label_extractors = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionMatchedLogEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionMatchedLogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionMatchedLogEl {
    #[doc = "A logs-based filter."]
    pub filter: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionMatchedLogEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionMatchedLogEl {
        MonitoringAlertPolicyConditionsElConditionMatchedLogEl {
            filter: self.filter,
            label_extractors: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionMatchedLogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionMatchedLogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionMatchedLogElRef {
        MonitoringAlertPolicyConditionsElConditionMatchedLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionMatchedLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA logs-based filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key to an extractor expression, which is used to\nextract the value for this label key. Each entry in this map is\na specification for how data should be extracted from log entries that\nmatch filter. Each combination of extracted values is treated as\na separate rule for the purposes of triggering notifications.\nLabel keys and corresponding values can be used in notifications\ngenerated by this condition."]
    pub fn label_extractors(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.label_extractors", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {
    #[doc = "Set the field `count`.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl
{
    type O = BlockAssignable<
        MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {
    pub fn build(
        self,
    ) -> MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {
        MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl {
            count: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef {
        MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElDynamic {
    trigger: Option<
        DynamicBlock<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl>,
    >,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
    duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_missing_data: Option<PrimField<String>>,
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl>>,
    dynamic: MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElDynamic,
}
impl MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
    #[doc = "Set the field `evaluation_missing_data`.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn set_evaluation_missing_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_missing_data = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v: impl Into<
            BlockAssignable<
                MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
    #[doc = "The amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub duration: PrimField<String>,
    #[doc = "Monitoring Query Language query that outputs a boolean stream."]
    pub query: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
        MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl {
            duration: self.duration,
            evaluation_missing_data: core::default::Default::default(),
            query: self.query,
            trigger: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef {
        MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluation_missing_data` after provisioning.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn evaluation_missing_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_missing_data", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\nMonitoring Query Language query that outputs a boolean stream."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElTriggerElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_metric_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_group: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
    #[doc = "Set the field `alert_rule`.\nThe alerting rule name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future.\n\nThis field is optional. If this field is not empty, then it must be a\nvalid Prometheus label name."]
    pub fn set_alert_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alert_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_metric_validation`.\nWhether to disable metric existence validation for this condition.\n\nThis allows alerting policies to be defined on metrics that do not yet\nexist, improving advanced customer workflows such as configuring\nalerting policies using Terraform.\n\nUsers with the 'monitoring.alertPolicyViewer' role are able to see the\nname of the non-existent metric in the alerting policy condition."]
    pub fn set_disable_metric_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_metric_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\nAlerts are considered firing once their PromQL expression evaluated\nto be \"true\" for this long. Alerts whose PromQL expression was not\nevaluated to be \"true\" for long enough are considered pending. The\ndefault value is zero. Must be zero or positive."]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluation_interval`.\nHow often this rule should be evaluated. Must be a positive multiple\nof 30 seconds or missing. The default value is 30 seconds. If this\nPrometheusQueryLanguageCondition was generated from a Prometheus\nalerting rule, then this value should be taken from the enclosing\nrule group."]
    pub fn set_evaluation_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to add to or overwrite in the PromQL query result. Label names\nmust be valid.\n\nLabel values can be templatized by using variables. The only available\nvariable names are the names of the labels in the PromQL result,\nalthough label names beginning with \\_\\_ (two \"\\_\") are reserved for\ninternal use. \"labels\" may be empty. This field is intended to be used\nfor organizing and identifying the AlertPolicy."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_group`.\nThe rule group name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future. This field is optional."]
    pub fn set_rule_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_group = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
    #[doc = "The PromQL expression to evaluate. Every evaluation cycle this\nexpression is evaluated at the current time, and all resultant time\nseries become pending/firing alerts. This field must not be empty."]
    pub query: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
        MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl {
            alert_rule: core::default::Default::default(),
            disable_metric_validation: core::default::Default::default(),
            duration: core::default::Default::default(),
            evaluation_interval: core::default::Default::default(),
            labels: core::default::Default::default(),
            query: self.query,
            rule_group: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef {
        MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alert_rule` after provisioning.\nThe alerting rule name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future.\n\nThis field is optional. If this field is not empty, then it must be a\nvalid Prometheus label name."]
    pub fn alert_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alert_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_metric_validation` after provisioning.\nWhether to disable metric existence validation for this condition.\n\nThis allows alerting policies to be defined on metrics that do not yet\nexist, improving advanced customer workflows such as configuring\nalerting policies using Terraform.\n\nUsers with the 'monitoring.alertPolicyViewer' role are able to see the\nname of the non-existent metric in the alerting policy condition."]
    pub fn disable_metric_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_metric_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nAlerts are considered firing once their PromQL expression evaluated\nto be \"true\" for this long. Alerts whose PromQL expression was not\nevaluated to be \"true\" for long enough are considered pending. The\ndefault value is zero. Must be zero or positive."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluation_interval` after provisioning.\nHow often this rule should be evaluated. Must be a positive multiple\nof 30 seconds or missing. The default value is 30 seconds. If this\nPrometheusQueryLanguageCondition was generated from a Prometheus\nalerting rule, then this value should be taken from the enclosing\nrule group."]
    pub fn evaluation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to add to or overwrite in the PromQL query result. Label names\nmust be valid.\n\nLabel values can be templatized by using variables. The only available\nvariable names are the names of the labels in the PromQL result,\nalthough label names beginning with \\_\\_ (two \"\\_\") are reserved for\ninternal use. \"labels\" may be empty. This field is intended to be used\nfor organizing and identifying the AlertPolicy."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\nThe PromQL expression to evaluate. Every evaluation cycle this\nexpression is evaluated at the current time, and all resultant time\nseries become pending/firing alerts. This field must not be empty."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\nThe rule group name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future. This field is optional."]
    pub fn rule_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_group", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
    column: PrimField<String>,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
    #[doc = "The name of the column containing the boolean value. If the value in a row is\nNULL, that row is ignored."]
    pub column: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
        MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl {
            column: self.column,
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe name of the column containing the boolean value. If the value in a row is\nNULL, that row is ignored."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal\nto 0 and typically must be less than or equal to 23. An API may\nchoose to allow the value \"24:00:00\" for scenarios like business\nclosing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and\nless than or equal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or\nequal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and\ntypically must be less than or equal to 59. An API may allow the\nvalue 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
        MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal\nto 0 and typically must be less than or equal to 23. An API may\nchoose to allow the value \"24:00:00\" for scenarios like business\nclosing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and\nless than or equal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or\nequal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and\ntypically must be less than or equal to 59. An API may allow the\nvalue 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElConditionSqlElDailyElDynamic {
    execution_time:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
    periodicity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_time:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl>>,
    dynamic: MonitoringAlertPolicyConditionsElConditionSqlElDailyElDynamic,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
    #[doc = "Set the field `execution_time`.\n"]
    pub fn set_execution_time(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execution_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElDailyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
    #[doc = "The number of days between runs. Must be greater than or equal\nto 1 day and less than or equal to 30 days."]
    pub periodicity: PrimField<f64>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
        MonitoringAlertPolicyConditionsElConditionSqlElDailyEl {
            periodicity: self.periodicity,
            execution_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `periodicity` after provisioning.\nThe number of days between runs. Must be greater than or equal\nto 1 day and less than or equal to 30 days."]
    pub fn periodicity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }
    #[doc = "Get a reference to the value of field `execution_time` after provisioning.\n"]
    pub fn execution_time(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElDailyElExecutionTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_offset: Option<PrimField<f64>>,
    periodicity: PrimField<f64>,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
    #[doc = "Set the field `minute_offset`.\nThe number of minutes after the hour (in UTC) to run the query.\nMust be greater than or equal to 0 minutes and less than or equal to\n59 minutes.  If left unspecified, then an arbitrary offset is used."]
    pub fn set_minute_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_offset = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
    #[doc = "Number of hours between runs. The interval must be greater than or\nequal to 1 hour and less than or equal to 48 hours."]
    pub periodicity: PrimField<f64>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
        MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl {
            minute_offset: core::default::Default::default(),
            periodicity: self.periodicity,
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minute_offset` after provisioning.\nThe number of minutes after the hour (in UTC) to run the query.\nMust be greater than or equal to 0 minutes and less than or equal to\n59 minutes.  If left unspecified, then an arbitrary offset is used."]
    pub fn minute_offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_offset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `periodicity` after provisioning.\nNumber of hours between runs. The interval must be greater than or\nequal to 1 hour and less than or equal to 48 hours."]
    pub fn periodicity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
    periodicity: PrimField<f64>,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
    #[doc = "Number of minutes between runs. The interval must be greater than or\nequal to 5 minutes and less than or equal to 1440 minutes."]
    pub periodicity: PrimField<f64>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
        MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl {
            periodicity: self.periodicity,
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `periodicity` after provisioning.\nNumber of minutes between runs. The interval must be greater than or\nequal to 5 minutes and less than or equal to 1440 minutes."]
    pub fn periodicity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
    comparison: PrimField<String>,
    threshold: PrimField<f64>,
}
impl MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
    #[doc = "The comparison to apply between the time\nseries (indicated by filter and aggregation)\nand the threshold (indicated by\nthreshold_value). The comparison is applied\non each time series, with the time series on\nthe left-hand side and the threshold on the\nright-hand side. Only COMPARISON_LT and\nCOMPARISON_GT are supported currently. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub comparison: PrimField<String>,
    #[doc = "The value against which to compare the row count."]
    pub threshold: PrimField<f64>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
        MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl {
            comparison: self.comparison,
            threshold: self.threshold,
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\nThe comparison to apply between the time\nseries (indicated by filter and aggregation)\nand the threshold (indicated by\nthreshold_value). The comparison is applied\non each time series, with the time series on\nthe left-hand side and the threshold on the\nright-hand side. Only COMPARISON_LT and\nCOMPARISON_GT are supported currently. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nThe value against which to compare the row count."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElConditionSqlElDynamic {
    boolean_test:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl>>,
    daily: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElDailyEl>>,
    hourly: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl>>,
    minutes: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl>>,
    row_count_test:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionSqlEl {
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boolean_test: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElDailyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hourly: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_count_test: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl>>,
    dynamic: MonitoringAlertPolicyConditionsElConditionSqlElDynamic,
}
impl MonitoringAlertPolicyConditionsElConditionSqlEl {
    #[doc = "Set the field `boolean_test`.\n"]
    pub fn set_boolean_test(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boolean_test = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boolean_test = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `daily`.\n"]
    pub fn set_daily(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElDailyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.daily = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.daily = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hourly`.\n"]
    pub fn set_hourly(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElHourlyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hourly = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElMinutesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.minutes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.minutes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `row_count_test`.\n"]
    pub fn set_row_count_test(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.row_count_test = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.row_count_test = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionSqlEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionSqlEl {
    #[doc = "The Log Analytics SQL query to run, as a string.  The query must\nconform to the required shape. Specifically, the query must not try to\nfilter the input by time.  A filter will automatically be applied\nto filter the input so that the query receives all rows received\nsince the last time the query was run."]
    pub query: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionSqlEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionSqlEl {
        MonitoringAlertPolicyConditionsElConditionSqlEl {
            query: self.query,
            boolean_test: core::default::Default::default(),
            daily: core::default::Default::default(),
            hourly: core::default::Default::default(),
            minutes: core::default::Default::default(),
            row_count_test: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionSqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionSqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionSqlElRef {
        MonitoringAlertPolicyConditionsElConditionSqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionSqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\nThe Log Analytics SQL query to run, as a string.  The query must\nconform to the required shape. Specifically, the query must not try to\nfilter the input by time.  A filter will automatically be applied\nto filter the input so that the query receives all rows received\nsince the last time the query was run."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `boolean_test` after provisioning.\n"]
    pub fn boolean_test(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElBooleanTestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boolean_test", self.base))
    }
    #[doc = "Get a reference to the value of field `daily` after provisioning.\n"]
    pub fn daily(&self) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElDailyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.daily", self.base))
    }
    #[doc = "Get a reference to the value of field `hourly` after provisioning.\n"]
    pub fn hourly(&self) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElHourlyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hourly", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElMinutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `row_count_test` after provisioning.\n"]
    pub fn row_count_test(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElRowCountTestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_count_test", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_series_reducer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_series_aligner: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
    #[doc = "Set the field `alignment_period`.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn set_alignment_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alignment_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_series_reducer`.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn set_cross_series_reducer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_series_reducer = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by_fields`.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn set_group_by_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group_by_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `per_series_aligner`.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn set_per_series_aligner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.per_series_aligner = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
        MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl {
            alignment_period: core::default::Default::default(),
            cross_series_reducer: core::default::Default::default(),
            group_by_fields: core::default::Default::default(),
            per_series_aligner: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef {
        MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alignment_period` after provisioning.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn alignment_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alignment_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_series_reducer` after provisioning.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn cross_series_reducer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_series_reducer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group_by_fields` after provisioning.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn group_by_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_by_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_series_aligner` after provisioning.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn per_series_aligner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_series_aligner", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_series_reducer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_series_aligner: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {
    #[doc = "Set the field `alignment_period`.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn set_alignment_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alignment_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_series_reducer`.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn set_cross_series_reducer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_series_reducer = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by_fields`.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn set_group_by_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group_by_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `per_series_aligner`.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn set_per_series_aligner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.per_series_aligner = Some(v.into());
        self
    }
}
impl ToListMappable
    for MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl
{
    type O = BlockAssignable<
        MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {
    pub fn build(
        self,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {
        MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl {
            alignment_period: core::default::Default::default(),
            cross_series_reducer: core::default::Default::default(),
            group_by_fields: core::default::Default::default(),
            per_series_aligner: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef {
        MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alignment_period` after provisioning.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn alignment_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alignment_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_series_reducer` after provisioning.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn cross_series_reducer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_series_reducer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group_by_fields` after provisioning.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn group_by_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_by_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_series_aligner` after provisioning.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn per_series_aligner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_series_aligner", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
    forecast_horizon: PrimField<String>,
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
    type O =
        BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
    #[doc = "The length of time into the future to forecast\nwhether a timeseries will violate the threshold.\nIf the predicted value is found to violate the\nthreshold, and the violation is observed in all\nforecasts made for the Configured 'duration',\nthen the timeseries is considered to be failing."]
    pub forecast_horizon: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
        MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl {
            forecast_horizon: self.forecast_horizon,
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef {
        MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `forecast_horizon` after provisioning.\nThe length of time into the future to forecast\nwhether a timeseries will violate the threshold.\nIf the predicted value is found to violate the\nthreshold, and the violation is observed in all\nforecasts made for the Configured 'duration',\nthen the timeseries is considered to be failing."]
    pub fn forecast_horizon(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forecast_horizon", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
    #[doc = "Set the field `count`.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {}
impl BuildMonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
        MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl {
            count: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef {
        MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElConditionThresholdElDynamic {
    aggregations:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl>>,
    denominator_aggregations: Option<
        DynamicBlock<
            MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl,
        >,
    >,
    forecast_options: Option<
        DynamicBlock<MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl>,
    >,
    trigger: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsElConditionThresholdEl {
    comparison: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denominator_filter: Option<PrimField<String>>,
    duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_missing_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregations: Option<Vec<MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denominator_aggregations:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forecast_options:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Vec<MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl>>,
    dynamic: MonitoringAlertPolicyConditionsElConditionThresholdElDynamic,
}
impl MonitoringAlertPolicyConditionsElConditionThresholdEl {
    #[doc = "Set the field `denominator_filter`.\nA filter that identifies a time series that\nshould be used as the denominator of a ratio\nthat will be compared with the threshold. If\na denominator_filter is specified, the time\nseries specified by the filter field will be\nused as the numerator.The filter is similar\nto the one that is specified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_denominator_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.denominator_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluation_missing_data`.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn set_evaluation_missing_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_missing_data = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold_value`.\nA value against which to compare the time\nseries."]
    pub fn set_threshold_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold_value = Some(v.into());
        self
    }
    #[doc = "Set the field `aggregations`.\n"]
    pub fn set_aggregations(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aggregations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `denominator_aggregations`.\n"]
    pub fn set_denominator_aggregations(
        mut self,
        v: impl Into<
            BlockAssignable<
                MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.denominator_aggregations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.denominator_aggregations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forecast_options`.\n"]
    pub fn set_forecast_options(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forecast_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forecast_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdElTriggerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsElConditionThresholdEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsElConditionThresholdEl {
    #[doc = "The comparison to apply between the time\nseries (indicated by filter and aggregation)\nand the threshold (indicated by\nthreshold_value). The comparison is applied\non each time series, with the time series on\nthe left-hand side and the threshold on the\nright-hand side. Only COMPARISON_LT and\nCOMPARISON_GT are supported currently. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub comparison: PrimField<String>,
    #[doc = "The amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub duration: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsElConditionThresholdEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsElConditionThresholdEl {
        MonitoringAlertPolicyConditionsElConditionThresholdEl {
            comparison: self.comparison,
            denominator_filter: core::default::Default::default(),
            duration: self.duration,
            evaluation_missing_data: core::default::Default::default(),
            filter: core::default::Default::default(),
            threshold_value: core::default::Default::default(),
            aggregations: core::default::Default::default(),
            denominator_aggregations: core::default::Default::default(),
            forecast_options: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElConditionThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElConditionThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringAlertPolicyConditionsElConditionThresholdElRef {
        MonitoringAlertPolicyConditionsElConditionThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElConditionThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\nThe comparison to apply between the time\nseries (indicated by filter and aggregation)\nand the threshold (indicated by\nthreshold_value). The comparison is applied\non each time series, with the time series on\nthe left-hand side and the threshold on the\nright-hand side. Only COMPARISON_LT and\nCOMPARISON_GT are supported currently. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `denominator_filter` after provisioning.\nA filter that identifies a time series that\nshould be used as the denominator of a ratio\nthat will be compared with the threshold. If\na denominator_filter is specified, the time\nseries specified by the filter field will be\nused as the numerator.The filter is similar\nto the one that is specified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn denominator_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.denominator_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluation_missing_data` after provisioning.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn evaluation_missing_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_missing_data", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold_value` after provisioning.\nA value against which to compare the time\nseries."]
    pub fn threshold_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aggregations` after provisioning.\n"]
    pub fn aggregations(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionThresholdElAggregationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aggregations", self.base))
    }
    #[doc = "Get a reference to the value of field `denominator_aggregations` after provisioning.\n"]
    pub fn denominator_aggregations(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionThresholdElDenominatorAggregationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denominator_aggregations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forecast_options` after provisioning.\n"]
    pub fn forecast_options(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionThresholdElForecastOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forecast_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionThresholdElTriggerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyConditionsElDynamic {
    condition_absent: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionAbsentEl>>,
    condition_matched_log:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionMatchedLogEl>>,
    condition_monitoring_query_language:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl>>,
    condition_prometheus_query_language:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl>>,
    condition_sql: Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionSqlEl>>,
    condition_threshold:
        Option<DynamicBlock<MonitoringAlertPolicyConditionsElConditionThresholdEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyConditionsEl {
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_absent: Option<Vec<MonitoringAlertPolicyConditionsElConditionAbsentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_matched_log: Option<Vec<MonitoringAlertPolicyConditionsElConditionMatchedLogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_monitoring_query_language:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_prometheus_query_language:
        Option<Vec<MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_sql: Option<Vec<MonitoringAlertPolicyConditionsElConditionSqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_threshold: Option<Vec<MonitoringAlertPolicyConditionsElConditionThresholdEl>>,
    dynamic: MonitoringAlertPolicyConditionsElDynamic,
}
impl MonitoringAlertPolicyConditionsEl {
    #[doc = "Set the field `condition_absent`.\n"]
    pub fn set_condition_absent(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionAbsentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_absent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_absent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_matched_log`.\n"]
    pub fn set_condition_matched_log(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionMatchedLogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_matched_log = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_matched_log = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_monitoring_query_language`.\n"]
    pub fn set_condition_monitoring_query_language(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_monitoring_query_language = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_monitoring_query_language = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_prometheus_query_language`.\n"]
    pub fn set_condition_prometheus_query_language(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_prometheus_query_language = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_prometheus_query_language = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_sql`.\n"]
    pub fn set_condition_sql(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionSqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_sql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_sql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_threshold`.\n"]
    pub fn set_condition_threshold(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyConditionsElConditionThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_threshold = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyConditionsEl {
    type O = BlockAssignable<MonitoringAlertPolicyConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyConditionsEl {
    #[doc = "A short name or phrase used to identify the\ncondition in dashboards, notifications, and\nincidents. To avoid confusion, don't use the same\ndisplay name for multiple conditions in the same\npolicy."]
    pub display_name: PrimField<String>,
}
impl BuildMonitoringAlertPolicyConditionsEl {
    pub fn build(self) -> MonitoringAlertPolicyConditionsEl {
        MonitoringAlertPolicyConditionsEl {
            display_name: self.display_name,
            condition_absent: core::default::Default::default(),
            condition_matched_log: core::default::Default::default(),
            condition_monitoring_query_language: core::default::Default::default(),
            condition_prometheus_query_language: core::default::Default::default(),
            condition_sql: core::default::Default::default(),
            condition_threshold: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyConditionsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyConditionsElRef {
        MonitoringAlertPolicyConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the\ncondition in dashboards, notifications, and\nincidents. To avoid confusion, don't use the same\ndisplay name for multiple conditions in the same\npolicy."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this condition.\nIts syntax is:\nprojects/[PROJECT_ID]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]\n[CONDITION_ID] is assigned by Stackdriver Monitoring when\nthe condition is created as part of a new or updated alerting\npolicy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `condition_absent` after provisioning.\n"]
    pub fn condition_absent(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionAbsentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_absent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition_matched_log` after provisioning.\n"]
    pub fn condition_matched_log(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionMatchedLogElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_matched_log", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition_monitoring_query_language` after provisioning.\n"]
    pub fn condition_monitoring_query_language(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionMonitoringQueryLanguageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_monitoring_query_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition_prometheus_query_language` after provisioning.\n"]
    pub fn condition_prometheus_query_language(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionPrometheusQueryLanguageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_prometheus_query_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition_sql` after provisioning.\n"]
    pub fn condition_sql(&self) -> ListRef<MonitoringAlertPolicyConditionsElConditionSqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_sql", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition_threshold` after provisioning.\n"]
    pub fn condition_threshold(
        &self,
    ) -> ListRef<MonitoringAlertPolicyConditionsElConditionThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyDocumentationElLinksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyDocumentationElLinksEl {
    #[doc = "Set the field `display_name`.\nA short display name for the link. The display name must not be empty or exceed 63 characters. Example: \"playbook\"."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nThe url of a webpage. A url can be templatized by using variables in the path or the query parameters. The total length of a URL should not exceed 2083 characters before and after variable expansion. Example: \"https://my_domain.com/playbook?name=${resource.name}\"."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyDocumentationElLinksEl {
    type O = BlockAssignable<MonitoringAlertPolicyDocumentationElLinksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyDocumentationElLinksEl {}
impl BuildMonitoringAlertPolicyDocumentationElLinksEl {
    pub fn build(self) -> MonitoringAlertPolicyDocumentationElLinksEl {
        MonitoringAlertPolicyDocumentationElLinksEl {
            display_name: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyDocumentationElLinksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyDocumentationElLinksElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyDocumentationElLinksElRef {
        MonitoringAlertPolicyDocumentationElLinksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyDocumentationElLinksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA short display name for the link. The display name must not be empty or exceed 63 characters. Example: \"playbook\"."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe url of a webpage. A url can be templatized by using variables in the path or the query parameters. The total length of a URL should not exceed 2083 characters before and after variable expansion. Example: \"https://my_domain.com/playbook?name=${resource.name}\"."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringAlertPolicyDocumentationElDynamic {
    links: Option<DynamicBlock<MonitoringAlertPolicyDocumentationElLinksEl>>,
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyDocumentationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<MonitoringAlertPolicyDocumentationElLinksEl>>,
    dynamic: MonitoringAlertPolicyDocumentationElDynamic,
}
impl MonitoringAlertPolicyDocumentationEl {
    #[doc = "Set the field `content`.\nThe text of the documentation, interpreted according to mimeType.\nThe content may not exceed 8,192 Unicode characters and may not\nexceed more than 10,240 bytes when encoded in UTF-8 format,\nwhichever is smaller."]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }
    #[doc = "Set the field `mime_type`.\nThe format of the content field. Presently, only the value\n\"text/markdown\" is supported."]
    pub fn set_mime_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mime_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subject`.\nThe subject line of the notification. The subject line may not\nexceed 10,240 bytes. In notifications generated by this policy the contents\nof the subject line after variable expansion will be truncated to 255 bytes\nor shorter at the latest UTF-8 character boundary."]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }
    #[doc = "Set the field `links`.\n"]
    pub fn set_links(
        mut self,
        v: impl Into<BlockAssignable<MonitoringAlertPolicyDocumentationElLinksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.links = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.links = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringAlertPolicyDocumentationEl {
    type O = BlockAssignable<MonitoringAlertPolicyDocumentationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyDocumentationEl {}
impl BuildMonitoringAlertPolicyDocumentationEl {
    pub fn build(self) -> MonitoringAlertPolicyDocumentationEl {
        MonitoringAlertPolicyDocumentationEl {
            content: core::default::Default::default(),
            mime_type: core::default::Default::default(),
            subject: core::default::Default::default(),
            links: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyDocumentationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyDocumentationElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyDocumentationElRef {
        MonitoringAlertPolicyDocumentationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyDocumentationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe text of the documentation, interpreted according to mimeType.\nThe content may not exceed 8,192 Unicode characters and may not\nexceed more than 10,240 bytes when encoded in UTF-8 format,\nwhichever is smaller."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\nThe format of the content field. Presently, only the value\n\"text/markdown\" is supported."]
    pub fn mime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\nThe subject line of the notification. The subject line may not\nexceed 10,240 bytes. In notifications generated by this policy the contents\nof the subject line after variable expansion will be truncated to 255 bytes\nor shorter at the latest UTF-8 character boundary."]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(&self) -> ListRef<MonitoringAlertPolicyDocumentationElLinksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringAlertPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MonitoringAlertPolicyTimeoutsEl {
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
impl ToListMappable for MonitoringAlertPolicyTimeoutsEl {
    type O = BlockAssignable<MonitoringAlertPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringAlertPolicyTimeoutsEl {}
impl BuildMonitoringAlertPolicyTimeoutsEl {
    pub fn build(self) -> MonitoringAlertPolicyTimeoutsEl {
        MonitoringAlertPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MonitoringAlertPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringAlertPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringAlertPolicyTimeoutsElRef {
        MonitoringAlertPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringAlertPolicyTimeoutsElRef {
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
struct MonitoringAlertPolicyDynamic {
    alert_strategy: Option<DynamicBlock<MonitoringAlertPolicyAlertStrategyEl>>,
    conditions: Option<DynamicBlock<MonitoringAlertPolicyConditionsEl>>,
    documentation: Option<DynamicBlock<MonitoringAlertPolicyDocumentationEl>>,
}
