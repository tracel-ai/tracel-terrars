use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxAgentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_uri: Option<PrimField<String>>,
    default_language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_chat_engine_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_multi_language_training: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_spell_correction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_playbook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_language_codes: Option<ListField<PrimField<String>>>,
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    answer_feedback_settings: Option<Vec<DialogflowCxAgentAnswerFeedbackSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_settings: Option<Vec<DialogflowCxAgentClientCertificateSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gen_app_builder_settings: Option<Vec<DialogflowCxAgentGenAppBuilderSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_integration_settings: Option<Vec<DialogflowCxAgentGitIntegrationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personalization_settings: Option<Vec<DialogflowCxAgentPersonalizationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_to_text_settings: Option<Vec<DialogflowCxAgentSpeechToTextSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_to_speech_settings: Option<Vec<DialogflowCxAgentTextToSpeechSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxAgentTimeoutsEl>,
    dynamic: DialogflowCxAgentDynamic,
}
struct DialogflowCxAgent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxAgentData>,
}
#[derive(Clone)]
pub struct DialogflowCxAgent(Rc<DialogflowCxAgent_>);
impl DialogflowCxAgent {
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
    #[doc = "Set the field `avatar_uri`.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn set_avatar_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_chat_engine_on_destroy`.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn set_delete_chat_engine_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_chat_engine_on_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_multi_language_training`.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn set_enable_multi_language_training(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_multi_language_training = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_spell_correction`.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn set_enable_spell_correction(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_spell_correction = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_stackdriver_logging`.\nDetermines whether this agent should log conversation queries."]
    pub fn set_enable_stackdriver_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `locked`.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn set_locked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().locked = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `security_settings`.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `start_playbook`.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn set_start_playbook(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_playbook = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_language_codes`.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn set_supported_language_codes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_language_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `answer_feedback_settings`.\n"]
    pub fn set_answer_feedback_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAnswerFeedbackSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().answer_feedback_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.answer_feedback_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentClientCertificateSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gen_app_builder_settings`.\n"]
    pub fn set_gen_app_builder_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentGenAppBuilderSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gen_app_builder_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gen_app_builder_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `git_integration_settings`.\n"]
    pub fn set_git_integration_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentGitIntegrationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_integration_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_integration_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `personalization_settings`.\n"]
    pub fn set_personalization_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentPersonalizationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().personalization_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.personalization_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `speech_to_text_settings`.\n"]
    pub fn set_speech_to_text_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentSpeechToTextSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().speech_to_text_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.speech_to_text_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_to_speech_settings`.\n"]
    pub fn set_text_to_speech_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentTextToSpeechSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().text_to_speech_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.text_to_speech_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxAgentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_chat_engine_on_destroy` after provisioning.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn delete_chat_engine_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_chat_engine_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multi_language_training` after provisioning.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn enable_multi_language_training(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_training", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_spell_correction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locked", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
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
    #[doc = "Get a reference to the value of field `satisfies_pzi` after provisioning.\nA read only boolean field reflecting Zone Isolation status of the agent."]
    pub fn satisfies_pzi(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzi", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nA read only boolean field reflecting Zone Separation status of the agent."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_playbook` after provisioning.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn start_playbook(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_playbook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_feedback_settings` after provisioning.\n"]
    pub fn answer_feedback_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentAnswerFeedbackSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.answer_feedback_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentClientCertificateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gen_app_builder_settings` after provisioning.\n"]
    pub fn gen_app_builder_settings(&self) -> ListRef<DialogflowCxAgentGenAppBuilderSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gen_app_builder_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_integration_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `personalization_settings` after provisioning.\n"]
    pub fn personalization_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentPersonalizationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.personalization_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> ListRef<DialogflowCxAgentSpeechToTextSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_to_text_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> ListRef<DialogflowCxAgentTextToSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_to_speech_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxAgent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxAgent {}
impl ToListMappable for DialogflowCxAgent {
    type O = ListRef<DialogflowCxAgentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxAgent_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_agent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxAgent {
    pub tf_id: String,
    #[doc = "The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub default_language_code: PrimField<String>,
    #[doc = "The human-readable name of the agent, unique within the location."]
    pub display_name: PrimField<String>,
    #[doc = "The name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub location: PrimField<String>,
    #[doc = "The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub time_zone: PrimField<String>,
}
impl BuildDialogflowCxAgent {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxAgent {
        let out = DialogflowCxAgent(Rc::new(DialogflowCxAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxAgentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                avatar_uri: core::default::Default::default(),
                default_language_code: self.default_language_code,
                delete_chat_engine_on_destroy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                enable_multi_language_training: core::default::Default::default(),
                enable_spell_correction: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                locked: core::default::Default::default(),
                project: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                start_playbook: core::default::Default::default(),
                supported_language_codes: core::default::Default::default(),
                time_zone: self.time_zone,
                advanced_settings: core::default::Default::default(),
                answer_feedback_settings: core::default::Default::default(),
                client_certificate_settings: core::default::Default::default(),
                gen_app_builder_settings: core::default::Default::default(),
                git_integration_settings: core::default::Default::default(),
                personalization_settings: core::default::Default::default(),
                speech_to_text_settings: core::default::Default::default(),
                text_to_speech_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxAgentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxAgentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.avatar_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_chat_engine_on_destroy` after provisioning.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn delete_chat_engine_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_chat_engine_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multi_language_training` after provisioning.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn enable_multi_language_training(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_training", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_spell_correction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locked", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
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
    #[doc = "Get a reference to the value of field `satisfies_pzi` after provisioning.\nA read only boolean field reflecting Zone Isolation status of the agent."]
    pub fn satisfies_pzi(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzi", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nA read only boolean field reflecting Zone Separation status of the agent."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_playbook` after provisioning.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn start_playbook(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_playbook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `answer_feedback_settings` after provisioning.\n"]
    pub fn answer_feedback_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentAnswerFeedbackSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.answer_feedback_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentClientCertificateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gen_app_builder_settings` after provisioning.\n"]
    pub fn gen_app_builder_settings(&self) -> ListRef<DialogflowCxAgentGenAppBuilderSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gen_app_builder_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_integration_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `personalization_settings` after provisioning.\n"]
    pub fn personalization_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentPersonalizationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.personalization_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> ListRef<DialogflowCxAgentSpeechToTextSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_to_text_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> ListRef<DialogflowCxAgentTextToSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_to_speech_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    #[doc = "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {}
impl BuildDialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
        DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
        DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}
impl DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    #[doc = "Set the field `enabled`.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_digit`.\nThe digit that terminates a DTMF digit sequence."]
    pub fn set_finish_digit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_digit = Some(v.into());
        self
    }
    #[doc = "Set the field `max_digits`.\nMax length of DTMF digits."]
    pub fn set_max_digits(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_digits = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {}
impl BuildDialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."]
    pub fn finish_digit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }
    #[doc = "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."]
    pub fn max_digits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_digits", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_consent_based_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_interaction_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
}
impl DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
    #[doc = "Set the field `enable_consent_based_redaction`.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."]
    pub fn set_enable_consent_based_redaction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_consent_based_redaction = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_interaction_logging`.\nEnables DF Interaction logging."]
    pub fn set_enable_interaction_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_interaction_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_stackdriver_logging`.\nEnables Google Cloud Logging."]
    pub fn set_enable_stackdriver_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_stackdriver_logging = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {}
impl BuildDialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
        DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl {
            enable_consent_based_redaction: core::default::Default::default(),
            enable_interaction_logging: core::default::Default::default(),
            enable_stackdriver_logging: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_consent_based_redaction` after provisioning.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."]
    pub fn enable_consent_based_redaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_consent_based_redaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_interaction_logging` after provisioning.\nEnables DF Interaction logging."]
    pub fn enable_interaction_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_interaction_logging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nEnables Google Cloud Logging."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpointer_sensitivity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    models: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_speech_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_timeout_based_endpointing: Option<PrimField<bool>>,
}
impl DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
    #[doc = "Set the field `endpointer_sensitivity`.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."]
    pub fn set_endpointer_sensitivity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.endpointer_sensitivity = Some(v.into());
        self
    }
    #[doc = "Set the field `models`.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."]
    pub fn set_models(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.models = Some(v.into());
        self
    }
    #[doc = "Set the field `no_speech_timeout`.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_no_speech_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_speech_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn set_use_timeout_based_endpointing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_timeout_based_endpointing = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {}
impl BuildDialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
        DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl {
            endpointer_sensitivity: core::default::Default::default(),
            models: core::default::Default::default(),
            no_speech_timeout: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpointer_sensitivity` after provisioning.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."]
    pub fn endpointer_sensitivity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpointer_sensitivity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `models` after provisioning.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."]
    pub fn models(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.models", self.base))
    }
    #[doc = "Get a reference to the value of field `no_speech_timeout` after provisioning.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn no_speech_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_speech_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn use_timeout_based_endpointing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_timeout_based_endpointing", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxAgentAdvancedSettingsElDynamic {
    audio_export_gcs_destination:
        Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>>,
    dtmf_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
    logging_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl>>,
    speech_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_export_gcs_destination:
        Option<Vec<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl>>,
    dynamic: DialogflowCxAgentAdvancedSettingsElDynamic,
}
impl DialogflowCxAgentAdvancedSettingsEl {
    #[doc = "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_export_gcs_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_export_gcs_destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dtmf_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElLoggingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `speech_settings`.\n"]
    pub fn set_speech_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElSpeechSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.speech_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.speech_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxAgentAdvancedSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAdvancedSettingsEl {}
impl BuildDialogflowCxAgentAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsEl {
        DialogflowCxAgentAdvancedSettingsEl {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAdvancedSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentAdvancedSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> ListRef<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_export_gcs_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentAdvancedSettingsElLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]
    pub fn speech_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentAdvancedSettingsElSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentAnswerFeedbackSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_answer_feedback: Option<PrimField<bool>>,
}
impl DialogflowCxAgentAnswerFeedbackSettingsEl {
    #[doc = "Set the field `enable_answer_feedback`.\nIf enabled, end users will be able to provide [answer feedback](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/submitAnswerFeedback#body.AnswerFeedback)\nto Dialogflow responses. Feature works only if interaction logging is enabled in the Dialogflow agent."]
    pub fn set_enable_answer_feedback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_answer_feedback = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentAnswerFeedbackSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAnswerFeedbackSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentAnswerFeedbackSettingsEl {}
impl BuildDialogflowCxAgentAnswerFeedbackSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAnswerFeedbackSettingsEl {
        DialogflowCxAgentAnswerFeedbackSettingsEl {
            enable_answer_feedback: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentAnswerFeedbackSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentAnswerFeedbackSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentAnswerFeedbackSettingsElRef {
        DialogflowCxAgentAnswerFeedbackSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentAnswerFeedbackSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_answer_feedback` after provisioning.\nIf enabled, end users will be able to provide [answer feedback](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/submitAnswerFeedback#body.AnswerFeedback)\nto Dialogflow responses. Feature works only if interaction logging is enabled in the Dialogflow agent."]
    pub fn enable_answer_feedback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_answer_feedback", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentClientCertificateSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase: Option<PrimField<String>>,
    private_key: PrimField<String>,
    ssl_certificate: PrimField<String>,
}
impl DialogflowCxAgentClientCertificateSettingsEl {
    #[doc = "Set the field `passphrase`.\nThe name of the SecretManager secret version resource storing the passphrase. 'passphrase' should be left unset if the private key is not encrypted. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn set_passphrase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passphrase = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentClientCertificateSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentClientCertificateSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentClientCertificateSettingsEl {
    #[doc = "The name of the SecretManager secret version resource storing the private key encoded in PEM format. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub private_key: PrimField<String>,
    #[doc = "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub ssl_certificate: PrimField<String>,
}
impl BuildDialogflowCxAgentClientCertificateSettingsEl {
    pub fn build(self) -> DialogflowCxAgentClientCertificateSettingsEl {
        DialogflowCxAgentClientCertificateSettingsEl {
            passphrase: core::default::Default::default(),
            private_key: self.private_key,
            ssl_certificate: self.ssl_certificate,
        }
    }
}
pub struct DialogflowCxAgentClientCertificateSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentClientCertificateSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentClientCertificateSettingsElRef {
        DialogflowCxAgentClientCertificateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentClientCertificateSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\nThe name of the SecretManager secret version resource storing the passphrase. 'passphrase' should be left unset if the private key is not encrypted. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nThe name of the SecretManager secret version resource storing the private key encoded in PEM format. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_certificate` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn ssl_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentGenAppBuilderSettingsEl {
    engine: PrimField<String>,
}
impl DialogflowCxAgentGenAppBuilderSettingsEl {}
impl ToListMappable for DialogflowCxAgentGenAppBuilderSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentGenAppBuilderSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentGenAppBuilderSettingsEl {
    #[doc = "The full name of the Gen App Builder engine related to this agent if there is one.\nFormat: projects/{Project ID}/locations/{Location ID}/collections/{Collection ID}/engines/{Engine ID}"]
    pub engine: PrimField<String>,
}
impl BuildDialogflowCxAgentGenAppBuilderSettingsEl {
    pub fn build(self) -> DialogflowCxAgentGenAppBuilderSettingsEl {
        DialogflowCxAgentGenAppBuilderSettingsEl {
            engine: self.engine,
        }
    }
}
pub struct DialogflowCxAgentGenAppBuilderSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentGenAppBuilderSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentGenAppBuilderSettingsElRef {
        DialogflowCxAgentGenAppBuilderSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentGenAppBuilderSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\nThe full name of the Gen App Builder engine related to this agent if there is one.\nFormat: projects/{Project ID}/locations/{Location ID}/collections/{Collection ID}/engines/{Engine ID}"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_branch: Option<PrimField<String>>,
}
impl DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    #[doc = "Set the field `access_token`.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `branches`.\nA list of branches configured to be used from Dialogflow."]
    pub fn set_branches(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.branches = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe unique repository display name for the GitHub repository."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `repository_uri`.\nThe GitHub repository URI related to the agent."]
    pub fn set_repository_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `tracking_branch`.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn set_tracking_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tracking_branch = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {}
impl BuildDialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    pub fn build(self) -> DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
        DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
            access_token: core::default::Default::default(),
            branches: core::default::Default::default(),
            display_name: core::default::Default::default(),
            repository_uri: core::default::Default::default(),
            tracking_branch: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
        DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `branches` after provisioning.\nA list of branches configured to be used from Dialogflow."]
    pub fn branches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe unique repository display name for the GitHub repository."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `repository_uri` after provisioning.\nThe GitHub repository URI related to the agent."]
    pub fn repository_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_branch` after provisioning.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn tracking_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tracking_branch", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxAgentGitIntegrationSettingsElDynamic {
    github_settings:
        Option<DynamicBlock<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxAgentGitIntegrationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    github_settings: Option<Vec<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
    dynamic: DialogflowCxAgentGitIntegrationSettingsElDynamic,
}
impl DialogflowCxAgentGitIntegrationSettingsEl {
    #[doc = "Set the field `github_settings`.\n"]
    pub fn set_github_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxAgentGitIntegrationSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentGitIntegrationSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentGitIntegrationSettingsEl {}
impl BuildDialogflowCxAgentGitIntegrationSettingsEl {
    pub fn build(self) -> DialogflowCxAgentGitIntegrationSettingsEl {
        DialogflowCxAgentGitIntegrationSettingsEl {
            github_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxAgentGitIntegrationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentGitIntegrationSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentGitIntegrationSettingsElRef {
        DialogflowCxAgentGitIntegrationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentGitIntegrationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `github_settings` after provisioning.\n"]
    pub fn github_settings(
        &self,
    ) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentPersonalizationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_end_user_metadata: Option<PrimField<String>>,
}
impl DialogflowCxAgentPersonalizationSettingsEl {
    #[doc = "Set the field `default_end_user_metadata`.\nDefault end user metadata, used when processing DetectIntent requests. Recommended to be filled as a template instead of hard-coded value, for example { \"age\": \"$session.params.age\" }.\nThe data will be merged with the [QueryParameters.end_user_metadata](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/QueryParameters#FIELDS.end_user_metadata)\nin [DetectIntentRequest.query_params](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#body.request_body.FIELDS.query_params) during query processing.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct)."]
    pub fn set_default_end_user_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_end_user_metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentPersonalizationSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentPersonalizationSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentPersonalizationSettingsEl {}
impl BuildDialogflowCxAgentPersonalizationSettingsEl {
    pub fn build(self) -> DialogflowCxAgentPersonalizationSettingsEl {
        DialogflowCxAgentPersonalizationSettingsEl {
            default_end_user_metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentPersonalizationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentPersonalizationSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentPersonalizationSettingsElRef {
        DialogflowCxAgentPersonalizationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentPersonalizationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_end_user_metadata` after provisioning.\nDefault end user metadata, used when processing DetectIntent requests. Recommended to be filled as a template instead of hard-coded value, for example { \"age\": \"$session.params.age\" }.\nThe data will be merged with the [QueryParameters.end_user_metadata](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/QueryParameters#FIELDS.end_user_metadata)\nin [DetectIntentRequest.query_params](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#body.request_body.FIELDS.query_params) during query processing.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct)."]
    pub fn default_end_user_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_end_user_metadata", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentSpeechToTextSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_speech_adaptation: Option<PrimField<bool>>,
}
impl DialogflowCxAgentSpeechToTextSettingsEl {
    #[doc = "Set the field `enable_speech_adaptation`.\nWhether to use speech adaptation for speech recognition."]
    pub fn set_enable_speech_adaptation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_speech_adaptation = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentSpeechToTextSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentSpeechToTextSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentSpeechToTextSettingsEl {}
impl BuildDialogflowCxAgentSpeechToTextSettingsEl {
    pub fn build(self) -> DialogflowCxAgentSpeechToTextSettingsEl {
        DialogflowCxAgentSpeechToTextSettingsEl {
            enable_speech_adaptation: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentSpeechToTextSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentSpeechToTextSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentSpeechToTextSettingsElRef {
        DialogflowCxAgentSpeechToTextSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentSpeechToTextSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_speech_adaptation` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn enable_speech_adaptation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_speech_adaptation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentTextToSpeechSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesize_speech_configs: Option<PrimField<String>>,
}
impl DialogflowCxAgentTextToSpeechSettingsEl {
    #[doc = "Set the field `synthesize_speech_configs`.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn set_synthesize_speech_configs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.synthesize_speech_configs = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxAgentTextToSpeechSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentTextToSpeechSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentTextToSpeechSettingsEl {}
impl BuildDialogflowCxAgentTextToSpeechSettingsEl {
    pub fn build(self) -> DialogflowCxAgentTextToSpeechSettingsEl {
        DialogflowCxAgentTextToSpeechSettingsEl {
            synthesize_speech_configs: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentTextToSpeechSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentTextToSpeechSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentTextToSpeechSettingsElRef {
        DialogflowCxAgentTextToSpeechSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentTextToSpeechSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `synthesize_speech_configs` after provisioning.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn synthesize_speech_configs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.synthesize_speech_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxAgentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxAgentTimeoutsEl {
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
impl ToListMappable for DialogflowCxAgentTimeoutsEl {
    type O = BlockAssignable<DialogflowCxAgentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxAgentTimeoutsEl {}
impl BuildDialogflowCxAgentTimeoutsEl {
    pub fn build(self) -> DialogflowCxAgentTimeoutsEl {
        DialogflowCxAgentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxAgentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxAgentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxAgentTimeoutsElRef {
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
struct DialogflowCxAgentDynamic {
    advanced_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsEl>>,
    answer_feedback_settings: Option<DynamicBlock<DialogflowCxAgentAnswerFeedbackSettingsEl>>,
    client_certificate_settings: Option<DynamicBlock<DialogflowCxAgentClientCertificateSettingsEl>>,
    gen_app_builder_settings: Option<DynamicBlock<DialogflowCxAgentGenAppBuilderSettingsEl>>,
    git_integration_settings: Option<DynamicBlock<DialogflowCxAgentGitIntegrationSettingsEl>>,
    personalization_settings: Option<DynamicBlock<DialogflowCxAgentPersonalizationSettingsEl>>,
    speech_to_text_settings: Option<DynamicBlock<DialogflowCxAgentSpeechToTextSettingsEl>>,
    text_to_speech_settings: Option<DynamicBlock<DialogflowCxAgentTextToSpeechSettingsEl>>,
}
