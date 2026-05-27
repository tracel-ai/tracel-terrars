use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowGeneratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_event: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_parameter: Option<Vec<DialogflowGeneratorInferenceParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_context: Option<Vec<DialogflowGeneratorSummarizationContextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowGeneratorTimeoutsEl>,
    dynamic: DialogflowGeneratorDynamic,
}
struct DialogflowGenerator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowGeneratorData>,
}
#[derive(Clone)]
pub struct DialogflowGenerator(Rc<DialogflowGenerator_>);
impl DialogflowGenerator {
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
    #[doc = "Set the field `description`.\nOptional. Human readable description of the generator."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `generator_id`.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn set_generator_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().generator_id = Some(v.into());
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
    #[doc = "Set the field `published_model`.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn set_published_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().published_model = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_event`.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn set_trigger_event(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().trigger_event = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_parameter`.\n"]
    pub fn set_inference_parameter(
        self,
        v: impl Into<BlockAssignable<DialogflowGeneratorInferenceParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_parameter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `summarization_context`.\n"]
    pub fn set_summarization_context(
        self,
        v: impl Into<BlockAssignable<DialogflowGeneratorSummarizationContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().summarization_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.summarization_context = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowGeneratorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the generator."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generator_id` after provisioning.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn generator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the generator."]
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
    #[doc = "Get a reference to the value of field `published_model` after provisioning.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn published_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.published_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_event` after provisioning.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn trigger_event(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_event", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inference_parameter` after provisioning.\n"]
    pub fn inference_parameter(&self) -> ListRef<DialogflowGeneratorInferenceParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_parameter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_context` after provisioning.\n"]
    pub fn summarization_context(&self) -> ListRef<DialogflowGeneratorSummarizationContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowGeneratorTimeoutsElRef {
        DialogflowGeneratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowGenerator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowGenerator {}
impl ToListMappable for DialogflowGenerator {
    type O = ListRef<DialogflowGeneratorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowGenerator_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_generator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowGenerator {
    pub tf_id: String,
    #[doc = "desc"]
    pub location: PrimField<String>,
}
impl BuildDialogflowGenerator {
    pub fn build(self, stack: &mut Stack) -> DialogflowGenerator {
        let out = DialogflowGenerator(Rc::new(DialogflowGenerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowGeneratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                generator_id: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                published_model: core::default::Default::default(),
                trigger_event: core::default::Default::default(),
                inference_parameter: core::default::Default::default(),
                summarization_context: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowGeneratorRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowGeneratorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the generator."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generator_id` after provisioning.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn generator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the generator."]
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
    #[doc = "Get a reference to the value of field `published_model` after provisioning.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn published_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.published_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_event` after provisioning.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn trigger_event(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_event", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inference_parameter` after provisioning.\n"]
    pub fn inference_parameter(&self) -> ListRef<DialogflowGeneratorInferenceParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_parameter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_context` after provisioning.\n"]
    pub fn summarization_context(&self) -> ListRef<DialogflowGeneratorSummarizationContextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowGeneratorTimeoutsElRef {
        DialogflowGeneratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorInferenceParameterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}
impl DialogflowGeneratorInferenceParameterEl {
    #[doc = "Set the field `max_output_tokens`.\nOptional. Maximum number of the output tokens for the generator."]
    pub fn set_max_output_tokens(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_output_tokens = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nOptional. Controls the randomness of LLM predictions. Low temperature = less random. High temperature = more random. If unset (or 0), uses a default value of 0."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
    #[doc = "Set the field `top_k`.\nOptional. Top-k changes how the model selects tokens for output. A top-k of 1 means the selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a top-k of 3 means that the next token is selected from among the 3 most probable tokens (using temperature). For each token selection step, the top K tokens with the highest probabilities are sampled. Then tokens are further filtered based on topP with the final token selected using temperature sampling. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [1, 40], default to 40."]
    pub fn set_top_k(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_k = Some(v.into());
        self
    }
    #[doc = "Set the field `top_p`.\nOptional. Top-p changes how the model selects tokens for output. Tokens are selected from most K (see topK parameter) probable to least until the sum of their probabilities equals the top-p value. For example, if tokens A, B, and C have a probability of 0.3, 0.2, and 0.1 and the top-p value is 0.5, then the model will select either A or B as the next token (using temperature) and doesn't consider C. The default top-p value is 0.95. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [0.0, 1.0], default to 0.95."]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowGeneratorInferenceParameterEl {
    type O = BlockAssignable<DialogflowGeneratorInferenceParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorInferenceParameterEl {}
impl BuildDialogflowGeneratorInferenceParameterEl {
    pub fn build(self) -> DialogflowGeneratorInferenceParameterEl {
        DialogflowGeneratorInferenceParameterEl {
            max_output_tokens: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}
pub struct DialogflowGeneratorInferenceParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorInferenceParameterElRef {
    fn new(shared: StackShared, base: String) -> DialogflowGeneratorInferenceParameterElRef {
        DialogflowGeneratorInferenceParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorInferenceParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_output_tokens` after provisioning.\nOptional. Maximum number of the output tokens for the generator."]
    pub fn max_output_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_output_tokens", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nOptional. Controls the randomness of LLM predictions. Low temperature = less random. High temperature = more random. If unset (or 0), uses a default value of 0."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
    #[doc = "Get a reference to the value of field `top_k` after provisioning.\nOptional. Top-k changes how the model selects tokens for output. A top-k of 1 means the selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a top-k of 3 means that the next token is selected from among the 3 most probable tokens (using temperature). For each token selection step, the top K tokens with the highest probabilities are sampled. Then tokens are further filtered based on topP with the final token selected using temperature sampling. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [1, 40], default to 40."]
    pub fn top_k(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }
    #[doc = "Get a reference to the value of field `top_p` after provisioning.\nOptional. Top-p changes how the model selects tokens for output. Tokens are selected from most K (see topK parameter) probable to least until the sum of their probabilities equals the top-p value. For example, if tokens A, B, and C have a probability of 0.3, 0.2, and 0.1 and the top-p value is 0.5, then the model will select either A or B as the next token (using temperature) and doesn't consider C. The default top-p value is 0.95. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [0.0, 1.0], default to 0.95."]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl
    DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl
{
    #[doc = "Set the field `create_time`.\nOptional. Create time of the message entry."]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nOptional. The language of the text."]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\nOptional. Participant role of the message. Possible values: [\"HUMAN_AGENT\", \"AUTOMATED_AGENT\", \"END_USER\"]"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nOptional. Transcript content of the message."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl { type O = BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl
{}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl { pub fn build (self) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl { DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl { create_time : core :: default :: Default :: default () , language_code : core :: default :: Default :: default () , role : core :: default :: Default :: default () , text : core :: default :: Default :: default () , } } }
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef { fn new (shared : StackShared , base : String) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef { DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `create_time` after provisioning.\nOptional. Create time of the message entry."] pub fn create_time (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.create_time" , self . base)) } # [doc = "Get a reference to the value of field `language_code` after provisioning.\nOptional. The language of the text."] pub fn language_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.language_code" , self . base)) } # [doc = "Get a reference to the value of field `role` after provisioning.\nOptional. Participant role of the message. Possible values: [\"HUMAN_AGENT\", \"AUTOMATED_AGENT\", \"END_USER\"]"] pub fn role (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.role" , self . base)) } # [doc = "Get a reference to the value of field `text` after provisioning.\nOptional. Transcript content of the message."] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElDynamic { message_entries : Option < DynamicBlock < DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl >> , }
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl { # [serde (skip_serializing_if = "Option::is_none")] message_entries : Option < Vec < DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl > > , dynamic : DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElDynamic , }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl {
    #[doc = "Set the field `message_entries`.\n"]
    pub fn set_message_entries(
        mut self,
        v : impl Into < BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_entries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_entries = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl
{
    type O = BlockAssignable<
        DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl {}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl {
    pub fn build(
        self,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl {
            message_entries: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_entries` after provisioning.\n"]    pub fn message_entries (& self) -> ListRef < DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElMessageEntriesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_entries", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl
{
    section: PrimField<String>,
    summary: PrimField<String>,
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl { }
impl ToListMappable for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl { type O = BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl
{
    #[doc = "Required. Name of the section."]
    pub section: PrimField<String>,
    #[doc = "Required. Summary text for the section."]
    pub summary: PrimField<String>,
}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl { pub fn build (self) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl { DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl { section : self . section , summary : self . summary , } } }
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef { fn new (shared : StackShared , base : String) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef { DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `section` after provisioning.\nRequired. Name of the section."] pub fn section (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.section" , self . base)) } # [doc = "Get a reference to the value of field `summary` after provisioning.\nRequired. Summary text for the section."] pub fn summary (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.summary" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElDynamic { summary_sections : Option < DynamicBlock < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl { # [serde (skip_serializing_if = "Option::is_none")] summary_sections : Option < Vec < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl > > , dynamic : DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElDynamic , }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl {
    #[doc = "Set the field `summary_sections`.\n"]
    pub fn set_summary_sections(
        mut self,
        v : impl Into < BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summary_sections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_sections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl
{
    type O = BlockAssignable<
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl
{}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl {
    pub fn build(
        self,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl {
            summary_sections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef
    {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `summary_sections` after provisioning.\n"]    pub fn summary_sections (& self) -> ListRef < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElSummarySectionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.summary_sections", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElDynamic {
    summary_suggestion: Option<
        DynamicBlock<
            DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_suggestion: Option<
        Vec<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl>,
    >,
    dynamic: DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElDynamic,
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
    #[doc = "Set the field `summary_suggestion`.\n"]
    pub fn set_summary_suggestion(
        mut self,
        v : impl Into < BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summary_suggestion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_suggestion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
    type O = BlockAssignable<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
    pub fn build(self) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl {
            summary_suggestion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `summary_suggestion` after provisioning.\n"]
    pub fn summary_suggestion(
        &self,
    ) -> ListRef<
        DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElSummarySuggestionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summary_suggestion", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl { # [doc = "Set the field `definition`.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""] pub fn set_definition (mut self , v : impl Into < PrimField < String > >) -> Self { self . definition = Some (v . into ()) ; self } # [doc = "Set the field `key`.\nOptional. Name of the section, for example, \"situation\"."] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `type_`.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"] pub fn set_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_ = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl { type O = BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl
{}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl { pub fn build (self) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl { DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl { definition : core :: default :: Default :: default () , key : core :: default :: Default :: default () , type_ : core :: default :: Default :: default () , } } }
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef { fn new (shared : StackShared , base : String) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef { DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `definition` after provisioning.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""] pub fn definition (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.definition" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\nOptional. Name of the section, for example, \"situation\"."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElDynamic { summarization_sections : Option < DynamicBlock < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl { # [serde (skip_serializing_if = "Option::is_none")] summarization_sections : Option < Vec < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl > > , dynamic : DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElDynamic , }
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl {
    #[doc = "Set the field `summarization_sections`.\n"]
    pub fn set_summarization_sections(
        mut self,
        v : impl Into < BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summarization_sections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_sections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl
{
    type O = BlockAssignable<
        DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl
{}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl {
    pub fn build(
        self,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl {
            summarization_sections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef
    {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `summarization_sections` after provisioning.\n"]    pub fn summarization_sections (& self) -> ListRef < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElSummarizationSectionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_sections", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElFewShotExamplesElDynamic {
    conversation_context: Option<
        DynamicBlock<
            DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl,
        >,
    >,
    output:
        Option<DynamicBlock<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl>>,
    summarization_section_list: Option<
        DynamicBlock<
            DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_info: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_context: Option<
        Vec<DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_section_list: Option<
        Vec<DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl>,
    >,
    dynamic: DialogflowGeneratorSummarizationContextElFewShotExamplesElDynamic,
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesEl {
    #[doc = "Set the field `extra_info`.\nOptional. Key is the placeholder field name in input, value is the value of the placeholder. E.g. instruction contains \"@price\", and ingested data has <\"price\", \"10\">"]
    pub fn set_extra_info(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.extra_info = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_context`.\n"]
    pub fn set_conversation_context(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_context = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `summarization_section_list`.\n"]
    pub fn set_summarization_section_list(
        mut self,
        v : impl Into < BlockAssignable < DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summarization_section_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_section_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowGeneratorSummarizationContextElFewShotExamplesEl {
    type O = BlockAssignable<DialogflowGeneratorSummarizationContextElFewShotExamplesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElFewShotExamplesEl {}
impl BuildDialogflowGeneratorSummarizationContextElFewShotExamplesEl {
    pub fn build(self) -> DialogflowGeneratorSummarizationContextElFewShotExamplesEl {
        DialogflowGeneratorSummarizationContextElFewShotExamplesEl {
            extra_info: core::default::Default::default(),
            conversation_context: core::default::Default::default(),
            output: core::default::Default::default(),
            summarization_section_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElFewShotExamplesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElFewShotExamplesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElFewShotExamplesElRef {
        DialogflowGeneratorSummarizationContextElFewShotExamplesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElFewShotExamplesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `extra_info` after provisioning.\nOptional. Key is the placeholder field name in input, value is the value of the placeholder. E.g. instruction contains \"@price\", and ingested data has <\"price\", \"10\">"]
    pub fn extra_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extra_info", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_context` after provisioning.\n"]
    pub fn conversation_context(
        &self,
    ) -> ListRef<DialogflowGeneratorSummarizationContextElFewShotExamplesElConversationContextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_context", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(
        &self,
    ) -> ListRef<DialogflowGeneratorSummarizationContextElFewShotExamplesElOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }
    #[doc = "Get a reference to the value of field `summarization_section_list` after provisioning.\n"]
    pub fn summarization_section_list(
        &self,
    ) -> ListRef<
        DialogflowGeneratorSummarizationContextElFewShotExamplesElSummarizationSectionListElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_section_list", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
    #[doc = "Set the field `definition`.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""]
    pub fn set_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.definition = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nOptional. Name of the section, for example, \"situation\"."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
    type O = BlockAssignable<DialogflowGeneratorSummarizationContextElSummarizationSectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextElSummarizationSectionsEl {}
impl BuildDialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
    pub fn build(self) -> DialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
        DialogflowGeneratorSummarizationContextElSummarizationSectionsEl {
            definition: core::default::Default::default(),
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef {
        DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `definition` after provisioning.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""]
    pub fn definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nOptional. Name of the section, for example, \"situation\"."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowGeneratorSummarizationContextElDynamic {
    few_shot_examples:
        Option<DynamicBlock<DialogflowGeneratorSummarizationContextElFewShotExamplesEl>>,
    summarization_sections:
        Option<DynamicBlock<DialogflowGeneratorSummarizationContextElSummarizationSectionsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowGeneratorSummarizationContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    few_shot_examples: Option<Vec<DialogflowGeneratorSummarizationContextElFewShotExamplesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_sections:
        Option<Vec<DialogflowGeneratorSummarizationContextElSummarizationSectionsEl>>,
    dynamic: DialogflowGeneratorSummarizationContextElDynamic,
}
impl DialogflowGeneratorSummarizationContextEl {
    #[doc = "Set the field `output_language_code`.\nOptional. The target language of the generated summary. The language code for conversation will be used if this field is empty. Supported 2.0 and later versions."]
    pub fn set_output_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"1.0\"]."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `few_shot_examples`.\n"]
    pub fn set_few_shot_examples(
        mut self,
        v: impl Into<BlockAssignable<DialogflowGeneratorSummarizationContextElFewShotExamplesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.few_shot_examples = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.few_shot_examples = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `summarization_sections`.\n"]
    pub fn set_summarization_sections(
        mut self,
        v: impl Into<BlockAssignable<DialogflowGeneratorSummarizationContextElSummarizationSectionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summarization_sections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_sections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowGeneratorSummarizationContextEl {
    type O = BlockAssignable<DialogflowGeneratorSummarizationContextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorSummarizationContextEl {}
impl BuildDialogflowGeneratorSummarizationContextEl {
    pub fn build(self) -> DialogflowGeneratorSummarizationContextEl {
        DialogflowGeneratorSummarizationContextEl {
            output_language_code: core::default::Default::default(),
            version: core::default::Default::default(),
            few_shot_examples: core::default::Default::default(),
            summarization_sections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowGeneratorSummarizationContextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorSummarizationContextElRef {
    fn new(shared: StackShared, base: String) -> DialogflowGeneratorSummarizationContextElRef {
        DialogflowGeneratorSummarizationContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorSummarizationContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_language_code` after provisioning.\nOptional. The target language of the generated summary. The language code for conversation will be used if this field is empty. Supported 2.0 and later versions."]
    pub fn output_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"1.0\"]."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `few_shot_examples` after provisioning.\n"]
    pub fn few_shot_examples(
        &self,
    ) -> ListRef<DialogflowGeneratorSummarizationContextElFewShotExamplesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.few_shot_examples", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_sections` after provisioning.\n"]
    pub fn summarization_sections(
        &self,
    ) -> ListRef<DialogflowGeneratorSummarizationContextElSummarizationSectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_sections", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowGeneratorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowGeneratorTimeoutsEl {
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
impl ToListMappable for DialogflowGeneratorTimeoutsEl {
    type O = BlockAssignable<DialogflowGeneratorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowGeneratorTimeoutsEl {}
impl BuildDialogflowGeneratorTimeoutsEl {
    pub fn build(self) -> DialogflowGeneratorTimeoutsEl {
        DialogflowGeneratorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowGeneratorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowGeneratorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowGeneratorTimeoutsElRef {
        DialogflowGeneratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowGeneratorTimeoutsElRef {
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
struct DialogflowGeneratorDynamic {
    inference_parameter: Option<DynamicBlock<DialogflowGeneratorInferenceParameterEl>>,
    summarization_context: Option<DynamicBlock<DialogflowGeneratorSummarizationContextEl>>,
}
