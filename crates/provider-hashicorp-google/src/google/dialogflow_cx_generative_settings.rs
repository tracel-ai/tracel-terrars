use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxGenerativeSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_settings: Option<Vec<DialogflowCxGenerativeSettingsFallbackSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_safety_settings:
        Option<Vec<DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_connector_settings:
        Option<Vec<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_model_settings: Option<Vec<DialogflowCxGenerativeSettingsLlmModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxGenerativeSettingsTimeoutsEl>,
    dynamic: DialogflowCxGenerativeSettingsDynamic,
}
struct DialogflowCxGenerativeSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxGenerativeSettingsData>,
}
#[derive(Clone)]
pub struct DialogflowCxGenerativeSettings(Rc<DialogflowCxGenerativeSettings_>);
impl DialogflowCxGenerativeSettings {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `fallback_settings`.\n"]
    pub fn set_fallback_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGenerativeSettingsFallbackSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fallback_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fallback_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generative_safety_settings`.\n"]
    pub fn set_generative_safety_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generative_safety_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generative_safety_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `knowledge_connector_settings`.\n"]
    pub fn set_knowledge_connector_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_connector_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .knowledge_connector_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxGenerativeSettingsLlmModelSettingsEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxGenerativeSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage for this settings."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the generativeSettings.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/generativeSettings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_settings` after provisioning.\n"]
    pub fn fallback_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsFallbackSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fallback_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generative_safety_settings` after provisioning.\n"]
    pub fn generative_safety_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_safety_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerativeSettingsTimeoutsElRef {
        DialogflowCxGenerativeSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxGenerativeSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxGenerativeSettings {}
impl ToListMappable for DialogflowCxGenerativeSettings {
    type O = ListRef<DialogflowCxGenerativeSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxGenerativeSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_generative_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxGenerativeSettings {
    pub tf_id: String,
    #[doc = "Language for this settings."]
    pub language_code: PrimField<String>,
}
impl BuildDialogflowCxGenerativeSettings {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxGenerativeSettings {
        let out = DialogflowCxGenerativeSettings(Rc::new(DialogflowCxGenerativeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxGenerativeSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                language_code: self.language_code,
                parent: core::default::Default::default(),
                fallback_settings: core::default::Default::default(),
                generative_safety_settings: core::default::Default::default(),
                knowledge_connector_settings: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxGenerativeSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxGenerativeSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage for this settings."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the generativeSettings.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/generativeSettings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_settings` after provisioning.\n"]
    pub fn fallback_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsFallbackSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fallback_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generative_safety_settings` after provisioning.\n"]
    pub fn generative_safety_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_safety_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerativeSettingsTimeoutsElRef {
        DialogflowCxGenerativeSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frozen: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_text: Option<PrimField<String>>,
}
impl DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
    #[doc = "Set the field `display_name`.\nPrompt name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `frozen`.\nIf the flag is true, the prompt is frozen and cannot be modified by users."]
    pub fn set_frozen(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.frozen = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_text`.\nPrompt text that is sent to a LLM on no-match default, placeholders are filled downstream. For example: \"Here is a conversation $conversation, a response is: \""]
    pub fn set_prompt_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt_text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {}
impl BuildDialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
        DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl {
            display_name: core::default::Default::default(),
            frozen: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef {
        DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nPrompt name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `frozen` after provisioning.\nIf the flag is true, the prompt is frozen and cannot be modified by users."]
    pub fn frozen(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.frozen", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\nPrompt text that is sent to a LLM on no-match default, placeholders are filled downstream. For example: \"Here is a conversation $conversation, a response is: \""]
    pub fn prompt_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt_text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxGenerativeSettingsFallbackSettingsElDynamic {
    prompt_templates:
        Option<DynamicBlock<DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsFallbackSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_prompt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_templates:
        Option<Vec<DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl>>,
    dynamic: DialogflowCxGenerativeSettingsFallbackSettingsElDynamic,
}
impl DialogflowCxGenerativeSettingsFallbackSettingsEl {
    #[doc = "Set the field `selected_prompt`.\nDisplay name of the selected prompt."]
    pub fn set_selected_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.selected_prompt = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt_templates`.\n"]
    pub fn set_prompt_templates(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt_templates = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_templates = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxGenerativeSettingsFallbackSettingsEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsFallbackSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsFallbackSettingsEl {}
impl BuildDialogflowCxGenerativeSettingsFallbackSettingsEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsFallbackSettingsEl {
        DialogflowCxGenerativeSettingsFallbackSettingsEl {
            selected_prompt: core::default::Default::default(),
            prompt_templates: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsFallbackSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsFallbackSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsFallbackSettingsElRef {
        DialogflowCxGenerativeSettingsFallbackSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsFallbackSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `selected_prompt` after provisioning.\nDisplay name of the selected prompt."]
    pub fn selected_prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.selected_prompt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_templates` after provisioning.\n"]
    pub fn prompt_templates(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsFallbackSettingsElPromptTemplatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_templates", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
    language_code: PrimField<String>,
    text: PrimField<String>,
}
impl DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {}
impl ToListMappable for DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
    type O =
        BlockAssignable<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
    #[doc = "Language code of the phrase."]
    pub language_code: PrimField<String>,
    #[doc = "Text input which can be used for prompt or banned phrases."]
    pub text: PrimField<String>,
}
impl BuildDialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
        DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl {
            language_code: self.language_code,
            text: self.text,
        }
    }
}
pub struct DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef {
        DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code of the phrase."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nText input which can be used for prompt or banned phrases."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxGenerativeSettingsGenerativeSafetySettingsElDynamic {
    banned_phrases: Option<
        DynamicBlock<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_banned_phrase_match_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_phrases:
        Option<Vec<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl>>,
    dynamic: DialogflowCxGenerativeSettingsGenerativeSafetySettingsElDynamic,
}
impl DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
    #[doc = "Set the field `default_banned_phrase_match_strategy`.\nOptional. Default phrase match strategy for banned phrases.\nSee [PhraseMatchStrategy](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/GenerativeSettings#phrasematchstrategy) for valid values."]
    pub fn set_default_banned_phrase_match_strategy(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.default_banned_phrase_match_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `banned_phrases`.\n"]
    pub fn set_banned_phrases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.banned_phrases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.banned_phrases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {}
impl BuildDialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
        DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl {
            default_banned_phrase_match_strategy: core::default::Default::default(),
            banned_phrases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef {
        DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsGenerativeSafetySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_banned_phrase_match_strategy` after provisioning.\nOptional. Default phrase match strategy for banned phrases.\nSee [PhraseMatchStrategy](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/GenerativeSettings#phrasematchstrategy) for valid values."]
    pub fn default_banned_phrase_match_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_banned_phrase_match_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `banned_phrases` after provisioning.\n"]
    pub fn banned_phrases(
        &self,
    ) -> ListRef<DialogflowCxGenerativeSettingsGenerativeSafetySettingsElBannedPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_phrases", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_identity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_data_store_fallback: Option<PrimField<bool>>,
}
impl DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
    #[doc = "Set the field `agent`.\nName of the virtual agent. Used for LLM prompt. Can be left empty."]
    pub fn set_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent = Some(v.into());
        self
    }
    #[doc = "Set the field `agent_identity`.\nIdentity of the agent, e.g. \"virtual agent\", \"AI assistant\"."]
    pub fn set_agent_identity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `agent_scope`.\nAgent scope, e.g. \"Example company website\", \"internal Example company website for employees\", \"manual of car owner\"."]
    pub fn set_agent_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `business`.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn set_business(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.business = Some(v.into());
        self
    }
    #[doc = "Set the field `business_description`.\nCompany description, used for LLM prompt, e.g. \"a family company selling freshly roasted coffee beans\".''"]
    pub fn set_business_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.business_description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_data_store_fallback`.\nWhether to disable fallback to Data Store search results (in case the LLM couldn't pick a proper answer). Per default the feature is enabled."]
    pub fn set_disable_data_store_fallback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_data_store_fallback = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {}
impl BuildDialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
        DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl {
            agent: core::default::Default::default(),
            agent_identity: core::default::Default::default(),
            agent_scope: core::default::Default::default(),
            business: core::default::Default::default(),
            business_description: core::default::Default::default(),
            disable_data_store_fallback: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef {
        DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\nName of the virtual agent. Used for LLM prompt. Can be left empty."]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
    #[doc = "Get a reference to the value of field `agent_identity` after provisioning.\nIdentity of the agent, e.g. \"virtual agent\", \"AI assistant\"."]
    pub fn agent_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_identity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `agent_scope` after provisioning.\nAgent scope, e.g. \"Example company website\", \"internal Example company website for employees\", \"manual of car owner\"."]
    pub fn agent_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `business` after provisioning.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn business(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business", self.base))
    }
    #[doc = "Get a reference to the value of field `business_description` after provisioning.\nCompany description, used for LLM prompt, e.g. \"a family company selling freshly roasted coffee beans\".''"]
    pub fn business_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_description", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_data_store_fallback` after provisioning.\nWhether to disable fallback to Data Store search results (in case the LLM couldn't pick a proper answer). Per default the feature is enabled."]
    pub fn disable_data_store_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_data_store_fallback", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettingsLlmModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_text: Option<PrimField<String>>,
}
impl DialogflowCxGenerativeSettingsLlmModelSettingsEl {
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
impl ToListMappable for DialogflowCxGenerativeSettingsLlmModelSettingsEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsLlmModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsLlmModelSettingsEl {}
impl BuildDialogflowCxGenerativeSettingsLlmModelSettingsEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsLlmModelSettingsEl {
        DialogflowCxGenerativeSettingsLlmModelSettingsEl {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsLlmModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsLlmModelSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxGenerativeSettingsLlmModelSettingsElRef {
        DialogflowCxGenerativeSettingsLlmModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsLlmModelSettingsElRef {
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
pub struct DialogflowCxGenerativeSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxGenerativeSettingsTimeoutsEl {
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
impl ToListMappable for DialogflowCxGenerativeSettingsTimeoutsEl {
    type O = BlockAssignable<DialogflowCxGenerativeSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxGenerativeSettingsTimeoutsEl {}
impl BuildDialogflowCxGenerativeSettingsTimeoutsEl {
    pub fn build(self) -> DialogflowCxGenerativeSettingsTimeoutsEl {
        DialogflowCxGenerativeSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxGenerativeSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxGenerativeSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxGenerativeSettingsTimeoutsElRef {
        DialogflowCxGenerativeSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxGenerativeSettingsTimeoutsElRef {
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
struct DialogflowCxGenerativeSettingsDynamic {
    fallback_settings: Option<DynamicBlock<DialogflowCxGenerativeSettingsFallbackSettingsEl>>,
    generative_safety_settings:
        Option<DynamicBlock<DialogflowCxGenerativeSettingsGenerativeSafetySettingsEl>>,
    knowledge_connector_settings:
        Option<DynamicBlock<DialogflowCxGenerativeSettingsKnowledgeConnectorSettingsEl>>,
    llm_model_settings: Option<DynamicBlock<DialogflowCxGenerativeSettingsLlmModelSettingsEl>>,
}
