use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactCenterInsightsAssessmentRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assessment_rule_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rule: Option<Vec<ContactCenterInsightsAssessmentRuleSampleRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_info: Option<Vec<ContactCenterInsightsAssessmentRuleScheduleInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContactCenterInsightsAssessmentRuleTimeoutsEl>,
    dynamic: ContactCenterInsightsAssessmentRuleDynamic,
}
struct ContactCenterInsightsAssessmentRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactCenterInsightsAssessmentRuleData>,
}
#[derive(Clone)]
pub struct ContactCenterInsightsAssessmentRule(Rc<ContactCenterInsightsAssessmentRule_>);
impl ContactCenterInsightsAssessmentRule {
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
    #[doc = "Set the field `active`.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }
    #[doc = "Set the field `assessment_rule_id`.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn set_assessment_rule_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().assessment_rule_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay Name of the assessment rule."]
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
    #[doc = "Set the field `sample_rule`.\n"]
    pub fn set_sample_rule(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsAssessmentRuleSampleRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sample_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sample_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule_info`.\n"]
    pub fn set_schedule_info(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsAssessmentRuleScheduleInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContactCenterInsightsAssessmentRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assessment_rule_id` after provisioning.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn assessment_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assessment_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this assessment rule was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the assessment rule."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the assessment rule.\nFormat:\nprojects/{project}/locations/{location}/assessmentRules/{assessment_rule}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this assessment rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rule` after provisioning.\n"]
    pub fn sample_rule(&self) -> ListRef<ContactCenterInsightsAssessmentRuleSampleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> ListRef<ContactCenterInsightsAssessmentRuleScheduleInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAssessmentRuleTimeoutsElRef {
        ContactCenterInsightsAssessmentRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContactCenterInsightsAssessmentRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactCenterInsightsAssessmentRule {}
impl ToListMappable for ContactCenterInsightsAssessmentRule {
    type O = ListRef<ContactCenterInsightsAssessmentRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactCenterInsightsAssessmentRule_ {
    fn extract_resource_type(&self) -> String {
        "google_contact_center_insights_assessment_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactCenterInsightsAssessmentRule {
    pub tf_id: String,
    #[doc = "Location of the resource."]
    pub location: PrimField<String>,
}
impl BuildContactCenterInsightsAssessmentRule {
    pub fn build(self, stack: &mut Stack) -> ContactCenterInsightsAssessmentRule {
        let out =
            ContactCenterInsightsAssessmentRule(Rc::new(ContactCenterInsightsAssessmentRule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ContactCenterInsightsAssessmentRuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    active: core::default::Default::default(),
                    assessment_rule_id: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    sample_rule: core::default::Default::default(),
                    schedule_info: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactCenterInsightsAssessmentRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAssessmentRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactCenterInsightsAssessmentRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assessment_rule_id` after provisioning.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn assessment_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assessment_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this assessment rule was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the assessment rule."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the assessment rule.\nFormat:\nprojects/{project}/locations/{location}/assessmentRules/{assessment_rule}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this assessment rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rule` after provisioning.\n"]
    pub fn sample_rule(&self) -> ListRef<ContactCenterInsightsAssessmentRuleSampleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> ListRef<ContactCenterInsightsAssessmentRuleScheduleInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAssessmentRuleTimeoutsElRef {
        ContactCenterInsightsAssessmentRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRuleSampleRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_row: Option<PrimField<f64>>,
}
impl ContactCenterInsightsAssessmentRuleSampleRuleEl {
    #[doc = "Set the field `conversation_filter`.\nTo specify the filter for the conversions that should apply this sample\nrule. An empty filter means this sample rule applies to all conversations."]
    pub fn set_conversation_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conversation_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension`.\nGroup by dimension to sample the conversation. If no dimension is\nprovided, the sampling will be applied to the project level.\nCurrent supported dimensions is 'quality_metadata.agent_info.agent_id'."]
    pub fn set_dimension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_percentage`.\nPercentage of conversations that we should sample  based on the dimension\nbetween [0, 100]."]
    pub fn set_sample_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_row`.\nNumber of the conversations that we should sample based on the dimension."]
    pub fn set_sample_row(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_row = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsAssessmentRuleSampleRuleEl {
    type O = BlockAssignable<ContactCenterInsightsAssessmentRuleSampleRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAssessmentRuleSampleRuleEl {}
impl BuildContactCenterInsightsAssessmentRuleSampleRuleEl {
    pub fn build(self) -> ContactCenterInsightsAssessmentRuleSampleRuleEl {
        ContactCenterInsightsAssessmentRuleSampleRuleEl {
            conversation_filter: core::default::Default::default(),
            dimension: core::default::Default::default(),
            sample_percentage: core::default::Default::default(),
            sample_row: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAssessmentRuleSampleRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAssessmentRuleSampleRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAssessmentRuleSampleRuleElRef {
        ContactCenterInsightsAssessmentRuleSampleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAssessmentRuleSampleRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conversation_filter` after provisioning.\nTo specify the filter for the conversions that should apply this sample\nrule. An empty filter means this sample rule applies to all conversations."]
    pub fn conversation_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conversation_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\nGroup by dimension to sample the conversation. If no dimension is\nprovided, the sampling will be applied to the project level.\nCurrent supported dimensions is 'quality_metadata.agent_info.agent_id'."]
    pub fn dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_percentage` after provisioning.\nPercentage of conversations that we should sample  based on the dimension\nbetween [0, 100]."]
    pub fn sample_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sample_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_row` after provisioning.\nNumber of the conversations that we should sample based on the dimension."]
    pub fn sample_row(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_row", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRuleScheduleInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl ContactCenterInsightsAssessmentRuleScheduleInfoEl {
    #[doc = "Set the field `end_time`.\nEnd time of the schedule. If not specified, will keep scheduling new\npipelines for execution until the schedule is no longer active or deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\nThe groc expression.\nFormat: 'every number [synchronized]'\nCron syntax is not supported.\nTime units can be: minutes, hours\nSynchronized is optional and indicates that the schedule should be\nsynchronized to the start of the interval: every 5 minutes synchronized\nmeans 00:00, 00:05 ...\nOtherwise the start time is random within the interval.\nExample: 'every 5 minutes'\ncould be  00:02, 00:07, 00:12, ..."]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nStart time of the schedule. If not specified, will start as soon as the\nschedule is created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nThe timezone to use for the groc expression.\nIf not specified, defaults to UTC."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsAssessmentRuleScheduleInfoEl {
    type O = BlockAssignable<ContactCenterInsightsAssessmentRuleScheduleInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAssessmentRuleScheduleInfoEl {}
impl BuildContactCenterInsightsAssessmentRuleScheduleInfoEl {
    pub fn build(self) -> ContactCenterInsightsAssessmentRuleScheduleInfoEl {
        ContactCenterInsightsAssessmentRuleScheduleInfoEl {
            end_time: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAssessmentRuleScheduleInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAssessmentRuleScheduleInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAssessmentRuleScheduleInfoElRef {
        ContactCenterInsightsAssessmentRuleScheduleInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAssessmentRuleScheduleInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nEnd time of the schedule. If not specified, will keep scheduling new\npipelines for execution until the schedule is no longer active or deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nThe groc expression.\nFormat: 'every number [synchronized]'\nCron syntax is not supported.\nTime units can be: minutes, hours\nSynchronized is optional and indicates that the schedule should be\nsynchronized to the start of the interval: every 5 minutes synchronized\nmeans 00:00, 00:05 ...\nOtherwise the start time is random within the interval.\nExample: 'every 5 minutes'\ncould be  00:02, 00:07, 00:12, ..."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nStart time of the schedule. If not specified, will start as soon as the\nschedule is created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe timezone to use for the groc expression.\nIf not specified, defaults to UTC."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContactCenterInsightsAssessmentRuleTimeoutsEl {
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
impl ToListMappable for ContactCenterInsightsAssessmentRuleTimeoutsEl {
    type O = BlockAssignable<ContactCenterInsightsAssessmentRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAssessmentRuleTimeoutsEl {}
impl BuildContactCenterInsightsAssessmentRuleTimeoutsEl {
    pub fn build(self) -> ContactCenterInsightsAssessmentRuleTimeoutsEl {
        ContactCenterInsightsAssessmentRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAssessmentRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAssessmentRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContactCenterInsightsAssessmentRuleTimeoutsElRef {
        ContactCenterInsightsAssessmentRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAssessmentRuleTimeoutsElRef {
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
struct ContactCenterInsightsAssessmentRuleDynamic {
    sample_rule: Option<DynamicBlock<ContactCenterInsightsAssessmentRuleSampleRuleEl>>,
    schedule_info: Option<DynamicBlock<ContactCenterInsightsAssessmentRuleScheduleInfoEl>>,
}
