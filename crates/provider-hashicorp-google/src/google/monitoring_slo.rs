use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MonitoringSloData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    goal: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rolling_period_days: Option<PrimField<f64>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slo_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_sli: Option<Vec<MonitoringSloBasicSliEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_based_sli: Option<Vec<MonitoringSloRequestBasedSliEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MonitoringSloTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_based_sli: Option<Vec<MonitoringSloWindowsBasedSliEl>>,
    dynamic: MonitoringSloDynamic,
}
struct MonitoringSlo_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitoringSloData>,
}
#[derive(Clone)]
pub struct MonitoringSlo(Rc<MonitoringSlo_>);
impl MonitoringSlo {
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
    #[doc = "Set the field `calendar_period`.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn set_calendar_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().calendar_period = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nName used for UI elements listing this SLO."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `rolling_period_days`.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn set_rolling_period_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rolling_period_days = Some(v.into());
        self
    }
    #[doc = "Set the field `slo_id`.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn set_slo_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slo_id = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn set_user_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_sli`.\n"]
    pub fn set_basic_sli(self, v: impl Into<BlockAssignable<MonitoringSloBasicSliEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_sli = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_sli = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_based_sli`.\n"]
    pub fn set_request_based_sli(
        self,
        v: impl Into<BlockAssignable<MonitoringSloRequestBasedSliEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_based_sli = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_based_sli = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MonitoringSloTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `windows_based_sli`.\n"]
    pub fn set_windows_based_sli(
        self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().windows_based_sli = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.windows_based_sli = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `calendar_period` after provisioning.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn calendar_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.calendar_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this SLO."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `goal` after provisioning.\nThe fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub fn goal(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.goal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]"]
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
    #[doc = "Get a reference to the value of field `rolling_period_days` after provisioning.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn rolling_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rolling_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nID of the service to which this SLO belongs."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slo_id` after provisioning.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn slo_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slo_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_sli` after provisioning.\n"]
    pub fn basic_sli(&self) -> ListRef<MonitoringSloBasicSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_sli", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_based_sli` after provisioning.\n"]
    pub fn request_based_sli(&self) -> ListRef<MonitoringSloRequestBasedSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_based_sli", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSloTimeoutsElRef {
        MonitoringSloTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `windows_based_sli` after provisioning.\n"]
    pub fn windows_based_sli(&self) -> ListRef<MonitoringSloWindowsBasedSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_based_sli", self.extract_ref()),
        )
    }
}
impl Referable for MonitoringSlo {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MonitoringSlo {}
impl ToListMappable for MonitoringSlo {
    type O = ListRef<MonitoringSloRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MonitoringSlo_ {
    fn extract_resource_type(&self) -> String {
        "google_monitoring_slo".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMonitoringSlo {
    pub tf_id: String,
    #[doc = "The fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub goal: PrimField<f64>,
    #[doc = "ID of the service to which this SLO belongs."]
    pub service: PrimField<String>,
}
impl BuildMonitoringSlo {
    pub fn build(self, stack: &mut Stack) -> MonitoringSlo {
        let out = MonitoringSlo(Rc::new(MonitoringSlo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitoringSloData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                calendar_period: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                goal: self.goal,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                rolling_period_days: core::default::Default::default(),
                service: self.service,
                slo_id: core::default::Default::default(),
                user_labels: core::default::Default::default(),
                basic_sli: core::default::Default::default(),
                request_based_sli: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                windows_based_sli: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MonitoringSloRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MonitoringSloRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `calendar_period` after provisioning.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn calendar_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.calendar_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this SLO."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `goal` after provisioning.\nThe fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub fn goal(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.goal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]"]
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
    #[doc = "Get a reference to the value of field `rolling_period_days` after provisioning.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn rolling_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rolling_period_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nID of the service to which this SLO belongs."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slo_id` after provisioning.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn slo_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slo_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_sli` after provisioning.\n"]
    pub fn basic_sli(&self) -> ListRef<MonitoringSloBasicSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_sli", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_based_sli` after provisioning.\n"]
    pub fn request_based_sli(&self) -> ListRef<MonitoringSloRequestBasedSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_based_sli", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSloTimeoutsElRef {
        MonitoringSloTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `windows_based_sli` after provisioning.\n"]
    pub fn windows_based_sli(&self) -> ListRef<MonitoringSloWindowsBasedSliElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_based_sli", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringSloBasicSliElAvailabilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl MonitoringSloBasicSliElAvailabilityEl {
    #[doc = "Set the field `enabled`.\nWhether an availability SLI is enabled or not. Must be set to true. Defaults to 'true'."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringSloBasicSliElAvailabilityEl {
    type O = BlockAssignable<MonitoringSloBasicSliElAvailabilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloBasicSliElAvailabilityEl {}
impl BuildMonitoringSloBasicSliElAvailabilityEl {
    pub fn build(self) -> MonitoringSloBasicSliElAvailabilityEl {
        MonitoringSloBasicSliElAvailabilityEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloBasicSliElAvailabilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloBasicSliElAvailabilityElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloBasicSliElAvailabilityElRef {
        MonitoringSloBasicSliElAvailabilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloBasicSliElAvailabilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether an availability SLI is enabled or not. Must be set to true. Defaults to 'true'."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloBasicSliElLatencyEl {
    threshold: PrimField<String>,
}
impl MonitoringSloBasicSliElLatencyEl {}
impl ToListMappable for MonitoringSloBasicSliElLatencyEl {
    type O = BlockAssignable<MonitoringSloBasicSliElLatencyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloBasicSliElLatencyEl {
    #[doc = "A duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub threshold: PrimField<String>,
}
impl BuildMonitoringSloBasicSliElLatencyEl {
    pub fn build(self) -> MonitoringSloBasicSliElLatencyEl {
        MonitoringSloBasicSliElLatencyEl {
            threshold: self.threshold,
        }
    }
}
pub struct MonitoringSloBasicSliElLatencyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloBasicSliElLatencyElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloBasicSliElLatencyElRef {
        MonitoringSloBasicSliElLatencyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloBasicSliElLatencyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nA duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub fn threshold(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloBasicSliElDynamic {
    availability: Option<DynamicBlock<MonitoringSloBasicSliElAvailabilityEl>>,
    latency: Option<DynamicBlock<MonitoringSloBasicSliElLatencyEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloBasicSliEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability: Option<Vec<MonitoringSloBasicSliElAvailabilityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latency: Option<Vec<MonitoringSloBasicSliElLatencyEl>>,
    dynamic: MonitoringSloBasicSliElDynamic,
}
impl MonitoringSloBasicSliEl {
    #[doc = "Set the field `location`.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn set_location(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `method`.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn set_method(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn set_version(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `availability`.\n"]
    pub fn set_availability(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloBasicSliElAvailabilityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.availability = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.availability = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `latency`.\n"]
    pub fn set_latency(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloBasicSliElLatencyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.latency = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.latency = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloBasicSliEl {
    type O = BlockAssignable<MonitoringSloBasicSliEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloBasicSliEl {}
impl BuildMonitoringSloBasicSliEl {
    pub fn build(self) -> MonitoringSloBasicSliEl {
        MonitoringSloBasicSliEl {
            location: core::default::Default::default(),
            method: core::default::Default::default(),
            version: core::default::Default::default(),
            availability: core::default::Default::default(),
            latency: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloBasicSliElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloBasicSliElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloBasicSliElRef {
        MonitoringSloBasicSliElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloBasicSliElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn location(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn method(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn version(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `availability` after provisioning.\n"]
    pub fn availability(&self) -> ListRef<MonitoringSloBasicSliElAvailabilityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.availability", self.base))
    }
    #[doc = "Get a reference to the value of field `latency` after provisioning.\n"]
    pub fn latency(&self) -> ListRef<MonitoringSloBasicSliElLatencyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.latency", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloRequestBasedSliElDistributionCutElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl MonitoringSloRequestBasedSliElDistributionCutElRangeEl {
    #[doc = "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringSloRequestBasedSliElDistributionCutElRangeEl {
    type O = BlockAssignable<MonitoringSloRequestBasedSliElDistributionCutElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloRequestBasedSliElDistributionCutElRangeEl {}
impl BuildMonitoringSloRequestBasedSliElDistributionCutElRangeEl {
    pub fn build(self) -> MonitoringSloRequestBasedSliElDistributionCutElRangeEl {
        MonitoringSloRequestBasedSliElDistributionCutElRangeEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloRequestBasedSliElDistributionCutElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloRequestBasedSliElDistributionCutElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloRequestBasedSliElDistributionCutElRangeElRef {
        MonitoringSloRequestBasedSliElDistributionCutElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloRequestBasedSliElDistributionCutElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloRequestBasedSliElDistributionCutElDynamic {
    range: Option<DynamicBlock<MonitoringSloRequestBasedSliElDistributionCutElRangeEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloRequestBasedSliElDistributionCutEl {
    distribution_filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<MonitoringSloRequestBasedSliElDistributionCutElRangeEl>>,
    dynamic: MonitoringSloRequestBasedSliElDistributionCutElDynamic,
}
impl MonitoringSloRequestBasedSliElDistributionCutEl {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloRequestBasedSliElDistributionCutElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloRequestBasedSliElDistributionCutEl {
    type O = BlockAssignable<MonitoringSloRequestBasedSliElDistributionCutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloRequestBasedSliElDistributionCutEl {
    #[doc = "A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub distribution_filter: PrimField<String>,
}
impl BuildMonitoringSloRequestBasedSliElDistributionCutEl {
    pub fn build(self) -> MonitoringSloRequestBasedSliElDistributionCutEl {
        MonitoringSloRequestBasedSliElDistributionCutEl {
            distribution_filter: self.distribution_filter,
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloRequestBasedSliElDistributionCutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloRequestBasedSliElDistributionCutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloRequestBasedSliElDistributionCutElRef {
        MonitoringSloRequestBasedSliElDistributionCutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloRequestBasedSliElDistributionCutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `distribution_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn distribution_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<MonitoringSloRequestBasedSliElDistributionCutElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloRequestBasedSliElGoodTotalRatioEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bad_service_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    good_service_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_service_filter: Option<PrimField<String>>,
}
impl MonitoringSloRequestBasedSliElGoodTotalRatioEl {
    #[doc = "Set the field `bad_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_bad_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bad_service_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `good_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided.\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_good_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.good_service_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `total_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_total_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_service_filter = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringSloRequestBasedSliElGoodTotalRatioEl {
    type O = BlockAssignable<MonitoringSloRequestBasedSliElGoodTotalRatioEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloRequestBasedSliElGoodTotalRatioEl {}
impl BuildMonitoringSloRequestBasedSliElGoodTotalRatioEl {
    pub fn build(self) -> MonitoringSloRequestBasedSliElGoodTotalRatioEl {
        MonitoringSloRequestBasedSliElGoodTotalRatioEl {
            bad_service_filter: core::default::Default::default(),
            good_service_filter: core::default::Default::default(),
            total_service_filter: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloRequestBasedSliElGoodTotalRatioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloRequestBasedSliElGoodTotalRatioElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloRequestBasedSliElGoodTotalRatioElRef {
        MonitoringSloRequestBasedSliElGoodTotalRatioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloRequestBasedSliElGoodTotalRatioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bad_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn bad_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bad_service_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `good_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided.\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn good_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.good_service_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn total_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_service_filter", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloRequestBasedSliElDynamic {
    distribution_cut: Option<DynamicBlock<MonitoringSloRequestBasedSliElDistributionCutEl>>,
    good_total_ratio: Option<DynamicBlock<MonitoringSloRequestBasedSliElGoodTotalRatioEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloRequestBasedSliEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution_cut: Option<Vec<MonitoringSloRequestBasedSliElDistributionCutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    good_total_ratio: Option<Vec<MonitoringSloRequestBasedSliElGoodTotalRatioEl>>,
    dynamic: MonitoringSloRequestBasedSliElDynamic,
}
impl MonitoringSloRequestBasedSliEl {
    #[doc = "Set the field `distribution_cut`.\n"]
    pub fn set_distribution_cut(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloRequestBasedSliElDistributionCutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.distribution_cut = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.distribution_cut = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `good_total_ratio`.\n"]
    pub fn set_good_total_ratio(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloRequestBasedSliElGoodTotalRatioEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.good_total_ratio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloRequestBasedSliEl {
    type O = BlockAssignable<MonitoringSloRequestBasedSliEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloRequestBasedSliEl {}
impl BuildMonitoringSloRequestBasedSliEl {
    pub fn build(self) -> MonitoringSloRequestBasedSliEl {
        MonitoringSloRequestBasedSliEl {
            distribution_cut: core::default::Default::default(),
            good_total_ratio: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloRequestBasedSliElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloRequestBasedSliElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloRequestBasedSliElRef {
        MonitoringSloRequestBasedSliElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloRequestBasedSliElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `distribution_cut` after provisioning.\n"]
    pub fn distribution_cut(&self) -> ListRef<MonitoringSloRequestBasedSliElDistributionCutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.distribution_cut", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `good_total_ratio` after provisioning.\n"]
    pub fn good_total_ratio(&self) -> ListRef<MonitoringSloRequestBasedSliElGoodTotalRatioElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.good_total_ratio", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringSloTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MonitoringSloTimeoutsEl {
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
impl ToListMappable for MonitoringSloTimeoutsEl {
    type O = BlockAssignable<MonitoringSloTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloTimeoutsEl {}
impl BuildMonitoringSloTimeoutsEl {
    pub fn build(self) -> MonitoringSloTimeoutsEl {
        MonitoringSloTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloTimeoutsElRef {
        MonitoringSloTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloTimeoutsElRef {
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
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl {
    #[doc = "Set the field `enabled`.\nWhether an availability SLI is enabled or not. Must be set to 'true. Defaults to 'true'."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl
{
    type O = BlockAssignable<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl
{}
impl
    BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl
{
    pub fn build(
        self,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl
    {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef { fn new (shared : StackShared , base : String) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef { MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef { shared : shared , base : base . to_string () , } } }
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether an availability SLI is enabled or not. Must be set to 'true. Defaults to 'true'."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl {
    threshold: PrimField<String>,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl {}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl
{
    type O = BlockAssignable<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl
{
    #[doc = "A duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub threshold: PrimField<String>,
}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl {
    pub fn build(
        self,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl {
            threshold: self.threshold,
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef
    {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nA duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub fn threshold(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElDynamic { availability : Option < DynamicBlock < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl >> , latency : Option < DynamicBlock < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl >> , }
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl { # [serde (skip_serializing_if = "Option::is_none")] location : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] method : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] version : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] availability : Option < Vec < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl > > , # [serde (skip_serializing_if = "Option::is_none")] latency : Option < Vec < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl > > , dynamic : MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElDynamic , }
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl {
    #[doc = "Set the field `location`.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn set_location(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `method`.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn set_method(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn set_version(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `availability`.\n"]
    pub fn set_availability(
        mut self,
        v : impl Into < BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.availability = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.availability = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `latency`.\n"]
    pub fn set_latency(
        mut self,
        v : impl Into < BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.latency = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.latency = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl
{
    type O = BlockAssignable<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl {}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl {
    pub fn build(
        self,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl {
            location: core::default::Default::default(),
            method: core::default::Default::default(),
            version: core::default::Default::default(),
            availability: core::default::Default::default(),
            latency: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn location(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn method(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn version(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `availability` after provisioning.\n"]    pub fn availability (& self) -> ListRef < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElAvailabilityElRef >{
        ListRef::new(self.shared().clone(), format!("{}.availability", self.base))
    }
    #[doc = "Get a reference to the value of field `latency` after provisioning.\n"]
    pub fn latency(
        &self,
    ) -> ListRef<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElLatencyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.latency", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl {
    #[doc = "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl
{
    type O = BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl
{}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl { pub fn build (self) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl { MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef { fn new (shared : StackShared , base : String) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef { MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef { shared : shared , base : base . to_string () , } } }
impl
    MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElDynamic { range : Option < DynamicBlock < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl >> , }
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl { distribution_filter : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] range : Option < Vec < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl > > , dynamic : MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElDynamic , }
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v : impl Into < BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl
{
    type O = BlockAssignable<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl
{
    #[doc = "A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub distribution_filter: PrimField<String>,
}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl {
    pub fn build(
        self,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl {
            distribution_filter: self.distribution_filter,
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef
    {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `distribution_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn distribution_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]    pub fn range (& self) -> ListRef < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRangeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bad_service_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    good_service_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_service_filter: Option<PrimField<String>>,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl {
    #[doc = "Set the field `bad_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_bad_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bad_service_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `good_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_good_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.good_service_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `total_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_total_service_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_service_filter = Some(v.into());
        self
    }
}
impl ToListMappable
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl
{
    type O = BlockAssignable<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl
{}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl {
    pub fn build(
        self,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl {
            bad_service_filter: core::default::Default::default(),
            good_service_filter: core::default::Default::default(),
            total_service_filter: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef
    {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bad_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn bad_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bad_service_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `good_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn good_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.good_service_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn total_service_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_service_filter", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDynamic {
    distribution_cut: Option<
        DynamicBlock<
            MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl,
        >,
    >,
    good_total_ratio: Option<
        DynamicBlock<
            MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution_cut: Option<
        Vec<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    good_total_ratio: Option<
        Vec<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl>,
    >,
    dynamic: MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDynamic,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
    #[doc = "Set the field `distribution_cut`.\n"]
    pub fn set_distribution_cut(
        mut self,
        v : impl Into < BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.distribution_cut = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.distribution_cut = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `good_total_ratio`.\n"]
    pub fn set_good_total_ratio(
        mut self,
        v : impl Into < BlockAssignable < MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.good_total_ratio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl {
            distribution_cut: core::default::Default::default(),
            good_total_ratio: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `distribution_cut` after provisioning.\n"]
    pub fn distribution_cut(
        &self,
    ) -> ListRef<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElDistributionCutElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.distribution_cut", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `good_total_ratio` after provisioning.\n"]
    pub fn good_total_ratio(
        &self,
    ) -> ListRef<
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElGoodTotalRatioElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.good_total_ratio", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElDynamic {
    basic_sli_performance: Option<
        DynamicBlock<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl>,
    >,
    performance:
        Option<DynamicBlock<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_sli_performance:
        Option<Vec<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance: Option<Vec<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl>>,
    dynamic: MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElDynamic,
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
    #[doc = "Set the field `threshold`.\nIf window performance >= threshold, the window is counted\nas good."]
    pub fn set_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_sli_performance`.\n"]
    pub fn set_basic_sli_performance(
        mut self,
        v: impl Into<
            BlockAssignable<
                MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_sli_performance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_sli_performance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `performance`.\n"]
    pub fn set_performance(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.performance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.performance = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {}
impl BuildMonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl {
            threshold: core::default::Default::default(),
            basic_sli_performance: core::default::Default::default(),
            performance: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef {
        MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nIf window performance >= threshold, the window is counted\nas good."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
    #[doc = "Get a reference to the value of field `basic_sli_performance` after provisioning.\n"]
    pub fn basic_sli_performance(
        &self,
    ) -> ListRef<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElBasicSliPerformanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_sli_performance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance` after provisioning.\n"]
    pub fn performance(
        &self,
    ) -> ListRef<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElPerformanceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.performance", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
    #[doc = "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {}
impl BuildMonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
        MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef {
        MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElMetricMeanInRangeElDynamic {
    range: Option<DynamicBlock<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
    time_series: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl>>,
    dynamic: MonitoringSloWindowsBasedSliElMetricMeanInRangeElDynamic,
}
impl MonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElMetricMeanInRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
    #[doc = "A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nThe provided TimeSeries must have ValueType = INT64 or\nValueType = DOUBLE and MetricKind = GAUGE. Mean value 'X'\nshould satisfy 'range.min <= X <= range.max'\nunder good service."]
    pub time_series: PrimField<String>,
}
impl BuildMonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
        MonitoringSloWindowsBasedSliElMetricMeanInRangeEl {
            time_series: self.time_series,
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef {
        MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_series` after provisioning.\nA [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nThe provided TimeSeries must have ValueType = INT64 or\nValueType = DOUBLE and MetricKind = GAUGE. Mean value 'X'\nshould satisfy 'range.min <= X <= range.max'\nunder good service."]
    pub fn time_series(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_series", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
    #[doc = "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {}
impl BuildMonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
        MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef {
        MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElMetricSumInRangeElDynamic {
    range: Option<DynamicBlock<MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliElMetricSumInRangeEl {
    time_series: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl>>,
    dynamic: MonitoringSloWindowsBasedSliElMetricSumInRangeElDynamic,
}
impl MonitoringSloWindowsBasedSliElMetricSumInRangeEl {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliElMetricSumInRangeEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliElMetricSumInRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliElMetricSumInRangeEl {
    #[doc = "A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nquality. The provided TimeSeries must have\nValueType = INT64 or ValueType = DOUBLE and\nMetricKind = GAUGE.\n\nSummed value 'X' should satisfy\n'range.min <= X <= range.max' for a good window."]
    pub time_series: PrimField<String>,
}
impl BuildMonitoringSloWindowsBasedSliElMetricSumInRangeEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliElMetricSumInRangeEl {
        MonitoringSloWindowsBasedSliElMetricSumInRangeEl {
            time_series: self.time_series,
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElMetricSumInRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElMetricSumInRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringSloWindowsBasedSliElMetricSumInRangeElRef {
        MonitoringSloWindowsBasedSliElMetricSumInRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElMetricSumInRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_series` after provisioning.\nA [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nquality. The provided TimeSeries must have\nValueType = INT64 or ValueType = DOUBLE and\nMetricKind = GAUGE.\n\nSummed value 'X' should satisfy\n'range.min <= X <= range.max' for a good window."]
    pub fn time_series(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_series", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<MonitoringSloWindowsBasedSliElMetricSumInRangeElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloWindowsBasedSliElDynamic {
    good_total_ratio_threshold:
        Option<DynamicBlock<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl>>,
    metric_mean_in_range: Option<DynamicBlock<MonitoringSloWindowsBasedSliElMetricMeanInRangeEl>>,
    metric_sum_in_range: Option<DynamicBlock<MonitoringSloWindowsBasedSliElMetricSumInRangeEl>>,
}
#[derive(Serialize)]
pub struct MonitoringSloWindowsBasedSliEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    good_bad_metric_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    good_total_ratio_threshold:
        Option<Vec<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_mean_in_range: Option<Vec<MonitoringSloWindowsBasedSliElMetricMeanInRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_sum_in_range: Option<Vec<MonitoringSloWindowsBasedSliElMetricSumInRangeEl>>,
    dynamic: MonitoringSloWindowsBasedSliElDynamic,
}
impl MonitoringSloWindowsBasedSliEl {
    #[doc = "Set the field `good_bad_metric_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nwith ValueType = BOOL. The window is good if any true values\nappear in the window. One of 'good_bad_metric_filter',\n'good_total_ratio_threshold', 'metric_mean_in_range',\n'metric_sum_in_range' must be set for 'windows_based_sli'."]
    pub fn set_good_bad_metric_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.good_bad_metric_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `window_period`.\nDuration over which window quality is evaluated, given as a\nduration string \"{X}s\" representing X seconds. Must be an\ninteger fraction of a day and at least 60s."]
    pub fn set_window_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window_period = Some(v.into());
        self
    }
    #[doc = "Set the field `good_total_ratio_threshold`.\n"]
    pub fn set_good_total_ratio_threshold(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.good_total_ratio_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio_threshold = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metric_mean_in_range`.\n"]
    pub fn set_metric_mean_in_range(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliElMetricMeanInRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_mean_in_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_mean_in_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metric_sum_in_range`.\n"]
    pub fn set_metric_sum_in_range(
        mut self,
        v: impl Into<BlockAssignable<MonitoringSloWindowsBasedSliElMetricSumInRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_sum_in_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_sum_in_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringSloWindowsBasedSliEl {
    type O = BlockAssignable<MonitoringSloWindowsBasedSliEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringSloWindowsBasedSliEl {}
impl BuildMonitoringSloWindowsBasedSliEl {
    pub fn build(self) -> MonitoringSloWindowsBasedSliEl {
        MonitoringSloWindowsBasedSliEl {
            good_bad_metric_filter: core::default::Default::default(),
            window_period: core::default::Default::default(),
            good_total_ratio_threshold: core::default::Default::default(),
            metric_mean_in_range: core::default::Default::default(),
            metric_sum_in_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringSloWindowsBasedSliElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringSloWindowsBasedSliElRef {
    fn new(shared: StackShared, base: String) -> MonitoringSloWindowsBasedSliElRef {
        MonitoringSloWindowsBasedSliElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringSloWindowsBasedSliElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `good_bad_metric_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nwith ValueType = BOOL. The window is good if any true values\nappear in the window. One of 'good_bad_metric_filter',\n'good_total_ratio_threshold', 'metric_mean_in_range',\n'metric_sum_in_range' must be set for 'windows_based_sli'."]
    pub fn good_bad_metric_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.good_bad_metric_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `window_period` after provisioning.\nDuration over which window quality is evaluated, given as a\nduration string \"{X}s\" representing X seconds. Must be an\ninteger fraction of a day and at least 60s."]
    pub fn window_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.window_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `good_total_ratio_threshold` after provisioning.\n"]
    pub fn good_total_ratio_threshold(
        &self,
    ) -> ListRef<MonitoringSloWindowsBasedSliElGoodTotalRatioThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.good_total_ratio_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_mean_in_range` after provisioning.\n"]
    pub fn metric_mean_in_range(
        &self,
    ) -> ListRef<MonitoringSloWindowsBasedSliElMetricMeanInRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_mean_in_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_sum_in_range` after provisioning.\n"]
    pub fn metric_sum_in_range(
        &self,
    ) -> ListRef<MonitoringSloWindowsBasedSliElMetricSumInRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_sum_in_range", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MonitoringSloDynamic {
    basic_sli: Option<DynamicBlock<MonitoringSloBasicSliEl>>,
    request_based_sli: Option<DynamicBlock<MonitoringSloRequestBasedSliEl>>,
    windows_based_sli: Option<DynamicBlock<MonitoringSloWindowsBasedSliEl>>,
}
