use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactCenterInsightsQaQuestionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abbreviation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    answer_instructions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    qa_scorecard: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_type: Option<PrimField<String>>,
    revision: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    answer_choices: Option<Vec<ContactCenterInsightsQaQuestionAnswerChoicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<Vec<ContactCenterInsightsQaQuestionMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_question_config:
        Option<Vec<ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qa_question_data_options: Option<Vec<ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContactCenterInsightsQaQuestionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tuning_metadata: Option<Vec<ContactCenterInsightsQaQuestionTuningMetadataEl>>,
    dynamic: ContactCenterInsightsQaQuestionDynamic,
}
struct ContactCenterInsightsQaQuestion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactCenterInsightsQaQuestionData>,
}
#[derive(Clone)]
pub struct ContactCenterInsightsQaQuestion(Rc<ContactCenterInsightsQaQuestion_>);
impl ContactCenterInsightsQaQuestion {
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
    #[doc = "Set the field `abbreviation`.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn set_abbreviation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().abbreviation = Some(v.into());
        self
    }
    #[doc = "Set the field `answer_instructions`.\nInstructions describing how to determine the answer."]
    pub fn set_answer_instructions(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().answer_instructions = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `order`.\nDefines the order of the question within its parent scorecard revision."]
    pub fn set_order(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().order = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `question_body`.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn set_question_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().question_body = Some(v.into());
        self
    }
    #[doc = "Set the field `question_type`.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn set_question_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().question_type = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn set_tags(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `answer_choices`.\n"]
    pub fn set_answer_choices(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsQaQuestionAnswerChoicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().answer_choices = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.answer_choices = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsQaQuestionMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metrics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_question_config`.\n"]
    pub fn set_predefined_question_config(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predefined_question_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predefined_question_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `qa_question_data_options`.\n"]
    pub fn set_qa_question_data_options(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().qa_question_data_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.qa_question_data_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContactCenterInsightsQaQuestionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tuning_metadata`.\n"]
    pub fn set_tuning_metadata(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsQaQuestionTuningMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tuning_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tuning_metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `abbreviation` after provisioning.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn abbreviation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abbreviation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_instructions` after provisioning.\nInstructions describing how to determine the answer."]
    pub fn answer_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.answer_instructions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this question was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the question.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nDefines the order of the question within its parent scorecard revision."]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.order", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `question_body` after provisioning.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn question_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.question_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `question_type` after provisioning.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn question_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.question_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the question was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_choices` after provisioning.\n"]
    pub fn answer_choices(&self) -> ListRef<ContactCenterInsightsQaQuestionAnswerChoicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.answer_choices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> ListRef<ContactCenterInsightsQaQuestionMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_question_config` after provisioning.\n"]
    pub fn predefined_question_config(
        &self,
    ) -> ListRef<ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_question_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_question_data_options` after provisioning.\n"]
    pub fn qa_question_data_options(
        &self,
    ) -> ListRef<ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.qa_question_data_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaQuestionTimeoutsElRef {
        ContactCenterInsightsQaQuestionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tuning_metadata` after provisioning.\n"]
    pub fn tuning_metadata(&self) -> ListRef<ContactCenterInsightsQaQuestionTuningMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tuning_metadata", self.extract_ref()),
        )
    }
}
impl Referable for ContactCenterInsightsQaQuestion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactCenterInsightsQaQuestion {}
impl ToListMappable for ContactCenterInsightsQaQuestion {
    type O = ListRef<ContactCenterInsightsQaQuestionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactCenterInsightsQaQuestion_ {
    fn extract_resource_type(&self) -> String {
        "google_contact_center_insights_qa_question".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactCenterInsightsQaQuestion {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub qa_scorecard: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub revision: PrimField<String>,
}
impl BuildContactCenterInsightsQaQuestion {
    pub fn build(self, stack: &mut Stack) -> ContactCenterInsightsQaQuestion {
        let out = ContactCenterInsightsQaQuestion(Rc::new(ContactCenterInsightsQaQuestion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContactCenterInsightsQaQuestionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                abbreviation: core::default::Default::default(),
                answer_instructions: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                order: core::default::Default::default(),
                project: core::default::Default::default(),
                qa_scorecard: self.qa_scorecard,
                question_body: core::default::Default::default(),
                question_type: core::default::Default::default(),
                revision: self.revision,
                tags: core::default::Default::default(),
                answer_choices: core::default::Default::default(),
                metrics: core::default::Default::default(),
                predefined_question_config: core::default::Default::default(),
                qa_question_data_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tuning_metadata: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactCenterInsightsQaQuestionRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactCenterInsightsQaQuestionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abbreviation` after provisioning.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn abbreviation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abbreviation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_instructions` after provisioning.\nInstructions describing how to determine the answer."]
    pub fn answer_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.answer_instructions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this question was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the question.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nDefines the order of the question within its parent scorecard revision."]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.order", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `question_body` after provisioning.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn question_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.question_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `question_type` after provisioning.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn question_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.question_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the question was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_choices` after provisioning.\n"]
    pub fn answer_choices(&self) -> ListRef<ContactCenterInsightsQaQuestionAnswerChoicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.answer_choices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> ListRef<ContactCenterInsightsQaQuestionMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_question_config` after provisioning.\n"]
    pub fn predefined_question_config(
        &self,
    ) -> ListRef<ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_question_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_question_data_options` after provisioning.\n"]
    pub fn qa_question_data_options(
        &self,
    ) -> ListRef<ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.qa_question_data_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaQuestionTimeoutsElRef {
        ContactCenterInsightsQaQuestionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tuning_metadata` after provisioning.\n"]
    pub fn tuning_metadata(&self) -> ListRef<ContactCenterInsightsQaQuestionTuningMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tuning_metadata", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionAnswerChoicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bool_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    na_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    str_value: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaQuestionAnswerChoicesEl {
    #[doc = "Set the field `bool_value`.\nBoolean value."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nA short string used as an identifier."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `na_value`.\nA value of \"Not Applicable (N/A)\". If provided, this field may only\nbe set to 'true'. If a question receives this answer, it will be\nexcluded from any score calculations."]
    pub fn set_na_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.na_value = Some(v.into());
        self
    }
    #[doc = "Set the field `num_value`.\nNumerical value."]
    pub fn set_num_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_value = Some(v.into());
        self
    }
    #[doc = "Set the field `score`.\nNumerical score of the answer, used for generating the overall score of\na QaScorecardResult. If the answer uses na_value, this field is unused."]
    pub fn set_score(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score = Some(v.into());
        self
    }
    #[doc = "Set the field `str_value`.\nString value."]
    pub fn set_str_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.str_value = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsQaQuestionAnswerChoicesEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionAnswerChoicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionAnswerChoicesEl {}
impl BuildContactCenterInsightsQaQuestionAnswerChoicesEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionAnswerChoicesEl {
        ContactCenterInsightsQaQuestionAnswerChoicesEl {
            bool_value: core::default::Default::default(),
            key: core::default::Default::default(),
            na_value: core::default::Default::default(),
            num_value: core::default::Default::default(),
            score: core::default::Default::default(),
            str_value: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionAnswerChoicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionAnswerChoicesElRef {
    fn new(shared: StackShared, base: String) -> ContactCenterInsightsQaQuestionAnswerChoicesElRef {
        ContactCenterInsightsQaQuestionAnswerChoicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionAnswerChoicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nBoolean value."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA short string used as an identifier."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `na_value` after provisioning.\nA value of \"Not Applicable (N/A)\". If provided, this field may only\nbe set to 'true'. If a question receives this answer, it will be\nexcluded from any score calculations."]
    pub fn na_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.na_value", self.base))
    }
    #[doc = "Get a reference to the value of field `num_value` after provisioning.\nNumerical value."]
    pub fn num_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_value", self.base))
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nNumerical score of the answer, used for generating the overall score of\na QaScorecardResult. If the answer uses na_value, this field is unused."]
    pub fn score(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
    #[doc = "Get a reference to the value of field `str_value` after provisioning.\nString value."]
    pub fn str_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.str_value", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionMetricsEl {}
impl ContactCenterInsightsQaQuestionMetricsEl {}
impl ToListMappable for ContactCenterInsightsQaQuestionMetricsEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionMetricsEl {}
impl BuildContactCenterInsightsQaQuestionMetricsEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionMetricsEl {
        ContactCenterInsightsQaQuestionMetricsEl {}
    }
}
pub struct ContactCenterInsightsQaQuestionMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionMetricsElRef {
    fn new(shared: StackShared, base: String) -> ContactCenterInsightsQaQuestionMetricsElRef {
        ContactCenterInsightsQaQuestionMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accuracy` after provisioning.\nAccuracy of the model. Measures the percentage of correct answers the\nmodel gave on the test set."]
    pub fn accuracy(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.accuracy", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
    #[doc = "Set the field `type_`.\nThe type of the predefined question.\nPossible values:\nCONVERSATION_OUTCOME\nCONVERSATION_OUTCOME_ESCALATION_INITIATOR_ROLE"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {}
impl BuildContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
        ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef {
        ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionPredefinedQuestionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the predefined question.\nPossible values:\nCONVERSATION_OUTCOME\nCONVERSATION_OUTCOME_ESCALATION_INITIATOR_ROLE"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_dialogflow_interaction_data: Option<PrimField<bool>>,
}
impl ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {
    #[doc = "Set the field `include_dialogflow_interaction_data`.\nWhether to include the per turn Dialogflow interaction data in conversation\ntranscript."]
    pub fn set_include_dialogflow_interaction_data(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.include_dialogflow_interaction_data = Some(v.into());
        self
    }
}
impl ToListMappable
    for ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl
{
    type O = BlockAssignable<
        ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {}
impl BuildContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {
    pub fn build(
        self,
    ) -> ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {
        ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl {
            include_dialogflow_interaction_data: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef {
        ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_dialogflow_interaction_data` after provisioning.\nWhether to include the per turn Dialogflow interaction data in conversation\ntranscript."]
    pub fn include_dialogflow_interaction_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_dialogflow_interaction_data", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContactCenterInsightsQaQuestionQaQuestionDataOptionsElDynamic {
    conversation_data_options: Option<
        DynamicBlock<
            ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_data_options: Option<
        Vec<ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl>,
    >,
    dynamic: ContactCenterInsightsQaQuestionQaQuestionDataOptionsElDynamic,
}
impl ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
    #[doc = "Set the field `conversation_data_options`.\n"]
    pub fn set_conversation_data_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_data_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_data_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {}
impl BuildContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
        ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl {
            conversation_data_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef {
        ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionQaQuestionDataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conversation_data_options` after provisioning.\n"]
    pub fn conversation_data_options(
        &self,
    ) -> ListRef<ContactCenterInsightsQaQuestionQaQuestionDataOptionsElConversationDataOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_data_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaQuestionTimeoutsEl {
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
impl ToListMappable for ContactCenterInsightsQaQuestionTimeoutsEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionTimeoutsEl {}
impl BuildContactCenterInsightsQaQuestionTimeoutsEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionTimeoutsEl {
        ContactCenterInsightsQaQuestionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContactCenterInsightsQaQuestionTimeoutsElRef {
        ContactCenterInsightsQaQuestionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionTimeoutsElRef {
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
pub struct ContactCenterInsightsQaQuestionTuningMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_validation_warnings: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_valid_label_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tuning_error: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaQuestionTuningMetadataEl {
    #[doc = "Set the field `dataset_validation_warnings`.\nA list of any applicable data validation warnings about the question's\nfeedback labels."]
    pub fn set_dataset_validation_warnings(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.dataset_validation_warnings = Some(v.into());
        self
    }
    #[doc = "Set the field `total_valid_label_count`.\nTotal number of valid labels provided for the question at the time of\ntuining."]
    pub fn set_total_valid_label_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_valid_label_count = Some(v.into());
        self
    }
    #[doc = "Set the field `tuning_error`.\nError status of the tuning operation for the question. Will only be set\nif the tuning operation failed."]
    pub fn set_tuning_error(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tuning_error = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsQaQuestionTuningMetadataEl {
    type O = BlockAssignable<ContactCenterInsightsQaQuestionTuningMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaQuestionTuningMetadataEl {}
impl BuildContactCenterInsightsQaQuestionTuningMetadataEl {
    pub fn build(self) -> ContactCenterInsightsQaQuestionTuningMetadataEl {
        ContactCenterInsightsQaQuestionTuningMetadataEl {
            dataset_validation_warnings: core::default::Default::default(),
            total_valid_label_count: core::default::Default::default(),
            tuning_error: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaQuestionTuningMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaQuestionTuningMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaQuestionTuningMetadataElRef {
        ContactCenterInsightsQaQuestionTuningMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaQuestionTuningMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_validation_warnings` after provisioning.\nA list of any applicable data validation warnings about the question's\nfeedback labels."]
    pub fn dataset_validation_warnings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataset_validation_warnings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_valid_label_count` after provisioning.\nTotal number of valid labels provided for the question at the time of\ntuining."]
    pub fn total_valid_label_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_valid_label_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tuning_error` after provisioning.\nError status of the tuning operation for the question. Will only be set\nif the tuning operation failed."]
    pub fn tuning_error(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tuning_error", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContactCenterInsightsQaQuestionDynamic {
    answer_choices: Option<DynamicBlock<ContactCenterInsightsQaQuestionAnswerChoicesEl>>,
    metrics: Option<DynamicBlock<ContactCenterInsightsQaQuestionMetricsEl>>,
    predefined_question_config:
        Option<DynamicBlock<ContactCenterInsightsQaQuestionPredefinedQuestionConfigEl>>,
    qa_question_data_options:
        Option<DynamicBlock<ContactCenterInsightsQaQuestionQaQuestionDataOptionsEl>>,
    tuning_metadata: Option<DynamicBlock<ContactCenterInsightsQaQuestionTuningMetadataEl>>,
}
