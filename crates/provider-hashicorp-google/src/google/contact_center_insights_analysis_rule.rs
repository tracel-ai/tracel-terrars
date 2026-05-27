use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactCenterInsightsAnalysisRuleData {
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
    analysis_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_filter: Option<PrimField<String>>,
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
    annotator_selector: Option<Vec<ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContactCenterInsightsAnalysisRuleTimeoutsEl>,
    dynamic: ContactCenterInsightsAnalysisRuleDynamic,
}
struct ContactCenterInsightsAnalysisRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactCenterInsightsAnalysisRuleData>,
}
#[derive(Clone)]
pub struct ContactCenterInsightsAnalysisRule(Rc<ContactCenterInsightsAnalysisRule_>);
impl ContactCenterInsightsAnalysisRule {
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
    #[doc = "Set the field `active`.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }
    #[doc = "Set the field `analysis_percentage`.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn set_analysis_percentage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().analysis_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_filter`.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn set_conversation_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().conversation_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay Name of the analysis rule."]
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
    #[doc = "Set the field `annotator_selector`.\n"]
    pub fn set_annotator_selector(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().annotator_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.annotator_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContactCenterInsightsAnalysisRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `analysis_percentage` after provisioning.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn analysis_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analysis_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conversation_filter` after provisioning.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn conversation_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conversation_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this analysis rule was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the analysis rule."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the analysis rule. Randomly generated by Insights."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The most recent time at which this analysis rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotator_selector` after provisioning.\n"]
    pub fn annotator_selector(
        &self,
    ) -> ListRef<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.annotator_selector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAnalysisRuleTimeoutsElRef {
        ContactCenterInsightsAnalysisRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContactCenterInsightsAnalysisRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactCenterInsightsAnalysisRule {}
impl ToListMappable for ContactCenterInsightsAnalysisRule {
    type O = ListRef<ContactCenterInsightsAnalysisRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactCenterInsightsAnalysisRule_ {
    fn extract_resource_type(&self) -> String {
        "google_contact_center_insights_analysis_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactCenterInsightsAnalysisRule {
    pub tf_id: String,
    #[doc = "Location of the resource."]
    pub location: PrimField<String>,
}
impl BuildContactCenterInsightsAnalysisRule {
    pub fn build(self, stack: &mut Stack) -> ContactCenterInsightsAnalysisRule {
        let out = ContactCenterInsightsAnalysisRule(Rc::new(ContactCenterInsightsAnalysisRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContactCenterInsightsAnalysisRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active: core::default::Default::default(),
                analysis_percentage: core::default::Default::default(),
                conversation_filter: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                annotator_selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactCenterInsightsAnalysisRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactCenterInsightsAnalysisRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `analysis_percentage` after provisioning.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn analysis_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analysis_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conversation_filter` after provisioning.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn conversation_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conversation_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this analysis rule was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the analysis rule."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the analysis rule. Randomly generated by Insights."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The most recent time at which this analysis rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotator_selector` after provisioning.\n"]
    pub fn annotator_selector(
        &self,
    ) -> ListRef<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.annotator_selector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAnalysisRuleTimeoutsElRef {
        ContactCenterInsightsAnalysisRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    qa_scorecard_revisions: Option<ListField<PrimField<String>>>,
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {
    #[doc = "Set the field `qa_scorecard_revisions`.\nList of QaScorecardRevisions."]
    pub fn set_qa_scorecard_revisions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.qa_scorecard_revisions = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl
{
    type O = BlockAssignable<
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {}
impl BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {
    pub fn build(
        self,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl {
            qa_scorecard_revisions: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `qa_scorecard_revisions` after provisioning.\nList of QaScorecardRevisions."]
    pub fn qa_scorecard_revisions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.qa_scorecard_revisions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElDynamic {
    scorecard_list: Option<
        DynamicBlock<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl>,
    >,
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scorecard_list:
        Option<Vec<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl>>,
    dynamic: ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElDynamic,
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
    #[doc = "Set the field `scorecard_list`.\n"]
    pub fn set_scorecard_list(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scorecard_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scorecard_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
    type O = BlockAssignable<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {}
impl BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
    pub fn build(self) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl {
            scorecard_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scorecard_list` after provisioning.\n"]
    pub fn scorecard_list(
        &self,
    ) -> ListRef<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElScorecardListElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scorecard_list", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_model: Option<PrimField<String>>,
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
    #[doc = "Set the field `conversation_profile`.\nResource name of the Dialogflow conversation profile.\nFormat:\nprojects/{project}/locations/{location}/conversationProfiles/{conversation_profile}"]
    pub fn set_conversation_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conversation_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `summarization_model`.\nDefault summarization model to be used.\nPossible values:\nSUMMARIZATION_MODEL_UNSPECIFIED\nBASELINE_MODEL\nBASELINE_MODEL_V2_0 Possible values: [\"BASELINE_MODEL\", \"BASELINE_MODEL_V2_0\"]"]
    pub fn set_summarization_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.summarization_model = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
    type O =
        BlockAssignable<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {}
impl BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
    pub fn build(
        self,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl {
            conversation_profile: core::default::Default::default(),
            summarization_model: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conversation_profile` after provisioning.\nResource name of the Dialogflow conversation profile.\nFormat:\nprojects/{project}/locations/{location}/conversationProfiles/{conversation_profile}"]
    pub fn conversation_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conversation_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_model` after provisioning.\nDefault summarization model to be used.\nPossible values:\nSUMMARIZATION_MODEL_UNSPECIFIED\nBASELINE_MODEL\nBASELINE_MODEL_V2_0 Possible values: [\"BASELINE_MODEL\", \"BASELINE_MODEL_V2_0\"]"]
    pub fn summarization_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.summarization_model", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElDynamic {
    qa_config: Option<DynamicBlock<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl>>,
    summarization_config: Option<
        DynamicBlock<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_models: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phrase_matchers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_entity_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_intent_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_interruption_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_issue_model_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_phrase_matcher_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_qa_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_sentiment_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_silence_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_summarization_annotator: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qa_config: Option<Vec<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_config:
        Option<Vec<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl>>,
    dynamic: ContactCenterInsightsAnalysisRuleAnnotatorSelectorElDynamic,
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
    #[doc = "Set the field `issue_models`.\nThe issue model to run. If not provided, the most recently deployed topic\nmodel will be used. The provided issue model will only be used for\ninference if the issue model is deployed and if run_issue_model_annotator\nis set to true. If more than one issue model is provided, only the first\nprovided issue model will be used for inference."]
    pub fn set_issue_models(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.issue_models = Some(v.into());
        self
    }
    #[doc = "Set the field `phrase_matchers`.\nThe list of phrase matchers to run. If not provided, all active phrase\nmatchers will be used. If inactive phrase matchers are provided, they will\nnot be used. Phrase matchers will be run only if\nrun_phrase_matcher_annotator is set to true. Format:\nprojects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}"]
    pub fn set_phrase_matchers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.phrase_matchers = Some(v.into());
        self
    }
    #[doc = "Set the field `run_entity_annotator`.\nWhether to run the entity annotator."]
    pub fn set_run_entity_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_entity_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_intent_annotator`.\nWhether to run the intent annotator."]
    pub fn set_run_intent_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_intent_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_interruption_annotator`.\nWhether to run the interruption annotator."]
    pub fn set_run_interruption_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_interruption_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_issue_model_annotator`.\nWhether to run the issue model annotator. A model should have already been\ndeployed for this to take effect."]
    pub fn set_run_issue_model_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_issue_model_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_phrase_matcher_annotator`.\nWhether to run the active phrase matcher annotator(s)."]
    pub fn set_run_phrase_matcher_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_phrase_matcher_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_qa_annotator`.\nWhether to run the QA annotator."]
    pub fn set_run_qa_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_qa_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_sentiment_annotator`.\nWhether to run the sentiment annotator."]
    pub fn set_run_sentiment_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_sentiment_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_silence_annotator`.\nWhether to run the silence annotator."]
    pub fn set_run_silence_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_silence_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `run_summarization_annotator`.\nWhether to run the summarization annotator."]
    pub fn set_run_summarization_annotator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_summarization_annotator = Some(v.into());
        self
    }
    #[doc = "Set the field `qa_config`.\n"]
    pub fn set_qa_config(
        mut self,
        v: impl Into<BlockAssignable<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.qa_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.qa_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summarization_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
    type O = BlockAssignable<ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {}
impl BuildContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
    pub fn build(self) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl {
            issue_models: core::default::Default::default(),
            phrase_matchers: core::default::Default::default(),
            run_entity_annotator: core::default::Default::default(),
            run_intent_annotator: core::default::Default::default(),
            run_interruption_annotator: core::default::Default::default(),
            run_issue_model_annotator: core::default::Default::default(),
            run_phrase_matcher_annotator: core::default::Default::default(),
            run_qa_annotator: core::default::Default::default(),
            run_sentiment_annotator: core::default::Default::default(),
            run_silence_annotator: core::default::Default::default(),
            run_summarization_annotator: core::default::Default::default(),
            qa_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef {
        ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAnalysisRuleAnnotatorSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issue_models` after provisioning.\nThe issue model to run. If not provided, the most recently deployed topic\nmodel will be used. The provided issue model will only be used for\ninference if the issue model is deployed and if run_issue_model_annotator\nis set to true. If more than one issue model is provided, only the first\nprovided issue model will be used for inference."]
    pub fn issue_models(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.issue_models", self.base))
    }
    #[doc = "Get a reference to the value of field `phrase_matchers` after provisioning.\nThe list of phrase matchers to run. If not provided, all active phrase\nmatchers will be used. If inactive phrase matchers are provided, they will\nnot be used. Phrase matchers will be run only if\nrun_phrase_matcher_annotator is set to true. Format:\nprojects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}"]
    pub fn phrase_matchers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phrase_matchers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_entity_annotator` after provisioning.\nWhether to run the entity annotator."]
    pub fn run_entity_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_entity_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_intent_annotator` after provisioning.\nWhether to run the intent annotator."]
    pub fn run_intent_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_intent_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_interruption_annotator` after provisioning.\nWhether to run the interruption annotator."]
    pub fn run_interruption_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_interruption_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_issue_model_annotator` after provisioning.\nWhether to run the issue model annotator. A model should have already been\ndeployed for this to take effect."]
    pub fn run_issue_model_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_issue_model_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_phrase_matcher_annotator` after provisioning.\nWhether to run the active phrase matcher annotator(s)."]
    pub fn run_phrase_matcher_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_phrase_matcher_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_qa_annotator` after provisioning.\nWhether to run the QA annotator."]
    pub fn run_qa_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_qa_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_sentiment_annotator` after provisioning.\nWhether to run the sentiment annotator."]
    pub fn run_sentiment_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_sentiment_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_silence_annotator` after provisioning.\nWhether to run the silence annotator."]
    pub fn run_silence_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_silence_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_summarization_annotator` after provisioning.\nWhether to run the summarization annotator."]
    pub fn run_summarization_annotator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_summarization_annotator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `qa_config` after provisioning.\n"]
    pub fn qa_config(
        &self,
    ) -> ListRef<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElQaConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.qa_config", self.base))
    }
    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(
        &self,
    ) -> ListRef<ContactCenterInsightsAnalysisRuleAnnotatorSelectorElSummarizationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContactCenterInsightsAnalysisRuleTimeoutsEl {
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
impl ToListMappable for ContactCenterInsightsAnalysisRuleTimeoutsEl {
    type O = BlockAssignable<ContactCenterInsightsAnalysisRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAnalysisRuleTimeoutsEl {}
impl BuildContactCenterInsightsAnalysisRuleTimeoutsEl {
    pub fn build(self) -> ContactCenterInsightsAnalysisRuleTimeoutsEl {
        ContactCenterInsightsAnalysisRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAnalysisRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAnalysisRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContactCenterInsightsAnalysisRuleTimeoutsElRef {
        ContactCenterInsightsAnalysisRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAnalysisRuleTimeoutsElRef {
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
struct ContactCenterInsightsAnalysisRuleDynamic {
    annotator_selector: Option<DynamicBlock<ContactCenterInsightsAnalysisRuleAnnotatorSelectorEl>>,
}
