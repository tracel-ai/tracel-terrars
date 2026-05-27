use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxGeneratorData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_model_settings: Option<Vec<DialogflowCxGeneratorLlmModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_parameter: Option<Vec<DialogflowCxGeneratorModelParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholders: Option<Vec<DialogflowCxGeneratorPlaceholdersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_text: Option<Vec<DialogflowCxGeneratorPromptTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxGeneratorTimeoutsEl>,
    dynamic: DialogflowCxGeneratorDynamic,
}
struct DialogflowCxGenerator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxGeneratorData>,
}
#[derive(Clone)]
pub struct DialogflowCxGenerator(Rc<DialogflowCxGenerator_>);
impl DialogflowCxGenerator {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGeneratorLlmModelSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_model_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_parameter`.\n"]
    pub fn set_model_parameter(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGeneratorModelParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_parameter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placeholders`.\n"]
    pub fn set_placeholders(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGeneratorPlaceholdersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placeholders = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placeholders = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prompt_text`.\n"]
    pub fn set_prompt_text(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGeneratorPromptTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().prompt_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.prompt_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxGeneratorTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the generator, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Generator.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/generators/<Generator ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> ListRef<DialogflowCxGeneratorLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_parameter` after provisioning.\n"]
    pub fn model_parameter(&self) -> ListRef<DialogflowCxGeneratorModelParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_parameter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placeholders` after provisioning.\n"]
    pub fn placeholders(&self) -> ListRef<DialogflowCxGeneratorPlaceholdersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placeholders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\n"]
    pub fn prompt_text(&self) -> ListRef<DialogflowCxGeneratorPromptTextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGeneratorTimeoutsElRef {
        DialogflowCxGeneratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxGenerator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxGenerator {}
impl ToListMappable for DialogflowCxGenerator {
    type O = ListRef<DialogflowCxGeneratorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxGenerator_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_generator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxGenerator {
    pub tf_id: String,
    #[doc = "The human-readable name of the generator, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxGenerator {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxGenerator {
        let out = DialogflowCxGenerator(Rc::new(DialogflowCxGenerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxGeneratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                model_parameter: core::default::Default::default(),
                placeholders: core::default::Default::default(),
                prompt_text: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxGeneratorRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxGeneratorRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the generator, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Generator.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/generators/<Generator ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> ListRef<DialogflowCxGeneratorLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_parameter` after provisioning.\n"]
    pub fn model_parameter(&self) -> ListRef<DialogflowCxGeneratorModelParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_parameter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placeholders` after provisioning.\n"]
    pub fn placeholders(&self) -> ListRef<DialogflowCxGeneratorPlaceholdersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placeholders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\n"]
    pub fn prompt_text(&self) -> ListRef<DialogflowCxGeneratorPromptTextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGeneratorTimeoutsElRef {
        DialogflowCxGeneratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGeneratorLlmModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_text: Option<PrimField<String>>,
}
impl DialogflowCxGeneratorLlmModelSettingsEl {
    #[doc = "Set the field `model`.\nThe selected LLM model."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_text`.\nThe custom prompt to use."]
    pub fn set_prompt_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt_text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGeneratorLlmModelSettingsEl {
    type O = BlockAssignable<DialogflowCxGeneratorLlmModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGeneratorLlmModelSettingsEl {}
impl BuildDialogflowCxGeneratorLlmModelSettingsEl {
    pub fn build(self) -> DialogflowCxGeneratorLlmModelSettingsEl {
        DialogflowCxGeneratorLlmModelSettingsEl {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGeneratorLlmModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorLlmModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGeneratorLlmModelSettingsElRef {
        DialogflowCxGeneratorLlmModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGeneratorLlmModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe selected LLM model."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\nThe custom prompt to use."]
    pub fn prompt_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt_text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGeneratorModelParameterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_decode_steps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}
impl DialogflowCxGeneratorModelParameterEl {
    #[doc = "Set the field `max_decode_steps`.\nThe maximum number of tokens to generate."]
    pub fn set_max_decode_steps(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_decode_steps = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nThe temperature used for sampling. Temperature sampling occurs after both topP and topK have been applied.\nValid range: [0.0, 1.0] Low temperature = less random. High temperature = more random."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
    #[doc = "Set the field `top_k`.\nIf set, the sampling process in each step is limited to the topK tokens with highest probabilities.\nValid range: [1, 40] or 1000+. Small topK = less random. Large topK = more random."]
    pub fn set_top_k(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_k = Some(v.into());
        self
    }
    #[doc = "Set the field `top_p`.\nIf set, only the tokens comprising the top topP probability mass are considered.\nIf both topP and topK are set, topP will be used for further refining candidates selected with topK.\nValid range: (0.0, 1.0]. Small topP = less random. Large topP = more random."]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGeneratorModelParameterEl {
    type O = BlockAssignable<DialogflowCxGeneratorModelParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGeneratorModelParameterEl {}
impl BuildDialogflowCxGeneratorModelParameterEl {
    pub fn build(self) -> DialogflowCxGeneratorModelParameterEl {
        DialogflowCxGeneratorModelParameterEl {
            max_decode_steps: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGeneratorModelParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorModelParameterElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGeneratorModelParameterElRef {
        DialogflowCxGeneratorModelParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGeneratorModelParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_decode_steps` after provisioning.\nThe maximum number of tokens to generate."]
    pub fn max_decode_steps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_decode_steps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nThe temperature used for sampling. Temperature sampling occurs after both topP and topK have been applied.\nValid range: [0.0, 1.0] Low temperature = less random. High temperature = more random."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
    #[doc = "Get a reference to the value of field `top_k` after provisioning.\nIf set, the sampling process in each step is limited to the topK tokens with highest probabilities.\nValid range: [1, 40] or 1000+. Small topK = less random. Large topK = more random."]
    pub fn top_k(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }
    #[doc = "Get a reference to the value of field `top_p` after provisioning.\nIf set, only the tokens comprising the top topP probability mass are considered.\nIf both topP and topK are set, topP will be used for further refining candidates selected with topK.\nValid range: (0.0, 1.0]. Small topP = less random. Large topP = more random."]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGeneratorPlaceholdersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DialogflowCxGeneratorPlaceholdersEl {
    #[doc = "Set the field `id`.\nUnique ID used to map custom placeholder to parameters in fulfillment."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nCustom placeholder value in the prompt text."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGeneratorPlaceholdersEl {
    type O = BlockAssignable<DialogflowCxGeneratorPlaceholdersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGeneratorPlaceholdersEl {}
impl BuildDialogflowCxGeneratorPlaceholdersEl {
    pub fn build(self) -> DialogflowCxGeneratorPlaceholdersEl {
        DialogflowCxGeneratorPlaceholdersEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGeneratorPlaceholdersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorPlaceholdersElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGeneratorPlaceholdersElRef {
        DialogflowCxGeneratorPlaceholdersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGeneratorPlaceholdersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique ID used to map custom placeholder to parameters in fulfillment."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nCustom placeholder value in the prompt text."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGeneratorPromptTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxGeneratorPromptTextEl {
    #[doc = "Set the field `text`.\nText input which can be used for prompt or banned phrases."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGeneratorPromptTextEl {
    type O = BlockAssignable<DialogflowCxGeneratorPromptTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGeneratorPromptTextEl {}
impl BuildDialogflowCxGeneratorPromptTextEl {
    pub fn build(self) -> DialogflowCxGeneratorPromptTextEl {
        DialogflowCxGeneratorPromptTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGeneratorPromptTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorPromptTextElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGeneratorPromptTextElRef {
        DialogflowCxGeneratorPromptTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGeneratorPromptTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nText input which can be used for prompt or banned phrases."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGeneratorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxGeneratorTimeoutsEl {
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
impl ToListMappable for DialogflowCxGeneratorTimeoutsEl {
    type O = BlockAssignable<DialogflowCxGeneratorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGeneratorTimeoutsEl {}
impl BuildDialogflowCxGeneratorTimeoutsEl {
    pub fn build(self) -> DialogflowCxGeneratorTimeoutsEl {
        DialogflowCxGeneratorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGeneratorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGeneratorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGeneratorTimeoutsElRef {
        DialogflowCxGeneratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGeneratorTimeoutsElRef {
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
struct DialogflowCxGeneratorDynamic {
    llm_model_settings: Option<DynamicBlock<DialogflowCxGeneratorLlmModelSettingsEl>>,
    model_parameter: Option<DynamicBlock<DialogflowCxGeneratorModelParameterEl>>,
    placeholders: Option<DynamicBlock<DialogflowCxGeneratorPlaceholdersEl>>,
    prompt_text: Option<DynamicBlock<DialogflowCxGeneratorPromptTextEl>>,
}
