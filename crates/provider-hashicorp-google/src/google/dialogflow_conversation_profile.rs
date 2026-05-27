use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowConversationProfileData {
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
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automated_agent_config: Option<Vec<DialogflowConversationProfileAutomatedAgentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_agent_assistant_config:
        Option<Vec<DialogflowConversationProfileHumanAgentAssistantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_agent_handoff_config: Option<Vec<DialogflowConversationProfileHumanAgentHandoffConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DialogflowConversationProfileLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_message_event_notification_config:
        Option<Vec<DialogflowConversationProfileNewMessageEventNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_recognition_result_notification_config:
        Option<Vec<DialogflowConversationProfileNewRecognitionResultNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<Vec<DialogflowConversationProfileNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stt_config: Option<Vec<DialogflowConversationProfileSttConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowConversationProfileTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts_config: Option<Vec<DialogflowConversationProfileTtsConfigEl>>,
    dynamic: DialogflowConversationProfileDynamic,
}
struct DialogflowConversationProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowConversationProfileData>,
}
#[derive(Clone)]
pub struct DialogflowConversationProfile(Rc<DialogflowConversationProfile_>);
impl DialogflowConversationProfile {
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
    #[doc = "Set the field `language_code`.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `security_settings`.\nName of the CX SecuritySettings reference for the agent."]
    pub fn set_security_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nThe time zone of this conversational profile."]
    pub fn set_time_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().time_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `automated_agent_config`.\n"]
    pub fn set_automated_agent_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileAutomatedAgentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automated_agent_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automated_agent_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `human_agent_assistant_config`.\n"]
    pub fn set_human_agent_assistant_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileHumanAgentAssistantConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_agent_assistant_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .human_agent_assistant_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `human_agent_handoff_config`.\n"]
    pub fn set_human_agent_handoff_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileHumanAgentHandoffConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_agent_handoff_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_agent_handoff_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_message_event_notification_config`.\n"]
    pub fn set_new_message_event_notification_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileNewMessageEventNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .new_message_event_notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .new_message_event_notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_recognition_result_notification_config`.\n"]
    pub fn set_new_recognition_result_notification_config(
        self,
        v: impl Into<
            BlockAssignable<DialogflowConversationProfileNewRecognitionResultNotificationConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .new_recognition_result_notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .new_recognition_result_notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stt_config`.\n"]
    pub fn set_stt_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileSttConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stt_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stt_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowConversationProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tts_config`.\n"]
    pub fn set_tts_config(
        self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileTtsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tts_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tts_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. Human readable name for this profile. Max length 1024 bytes."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nname"]
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
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nName of the CX SecuritySettings reference for the agent."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this conversational profile."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_agent_config` after provisioning.\n"]
    pub fn automated_agent_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileAutomatedAgentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_agent_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `human_agent_assistant_config` after provisioning.\n"]
    pub fn human_agent_assistant_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentAssistantConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_agent_assistant_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `human_agent_handoff_config` after provisioning.\n"]
    pub fn human_agent_handoff_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentHandoffConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_agent_handoff_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DialogflowConversationProfileLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `new_message_event_notification_config` after provisioning.\n"]
    pub fn new_message_event_notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNewMessageEventNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.new_message_event_notification_config",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `new_recognition_result_notification_config` after provisioning.\n"]
    pub fn new_recognition_result_notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.new_recognition_result_notification_config",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stt_config` after provisioning.\n"]
    pub fn stt_config(&self) -> ListRef<DialogflowConversationProfileSttConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stt_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowConversationProfileTimeoutsElRef {
        DialogflowConversationProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tts_config` after provisioning.\n"]
    pub fn tts_config(&self) -> ListRef<DialogflowConversationProfileTtsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tts_config", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowConversationProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowConversationProfile {}
impl ToListMappable for DialogflowConversationProfile {
    type O = ListRef<DialogflowConversationProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowConversationProfile_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_conversation_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowConversationProfile {
    pub tf_id: String,
    #[doc = "Required. Human readable name for this profile. Max length 1024 bytes."]
    pub display_name: PrimField<String>,
    #[doc = "desc"]
    pub location: PrimField<String>,
}
impl BuildDialogflowConversationProfile {
    pub fn build(self, stack: &mut Stack) -> DialogflowConversationProfile {
        let out = DialogflowConversationProfile(Rc::new(DialogflowConversationProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowConversationProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                time_zone: core::default::Default::default(),
                automated_agent_config: core::default::Default::default(),
                human_agent_assistant_config: core::default::Default::default(),
                human_agent_handoff_config: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                new_message_event_notification_config: core::default::Default::default(),
                new_recognition_result_notification_config: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                stt_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tts_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowConversationProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowConversationProfileRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. Human readable name for this profile. Max length 1024 bytes."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nname"]
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
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\nName of the CX SecuritySettings reference for the agent."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this conversational profile."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_agent_config` after provisioning.\n"]
    pub fn automated_agent_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileAutomatedAgentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_agent_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `human_agent_assistant_config` after provisioning.\n"]
    pub fn human_agent_assistant_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentAssistantConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_agent_assistant_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `human_agent_handoff_config` after provisioning.\n"]
    pub fn human_agent_handoff_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentHandoffConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_agent_handoff_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DialogflowConversationProfileLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `new_message_event_notification_config` after provisioning.\n"]
    pub fn new_message_event_notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNewMessageEventNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.new_message_event_notification_config",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `new_recognition_result_notification_config` after provisioning.\n"]
    pub fn new_recognition_result_notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.new_recognition_result_notification_config",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stt_config` after provisioning.\n"]
    pub fn stt_config(&self) -> ListRef<DialogflowConversationProfileSttConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stt_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowConversationProfileTimeoutsElRef {
        DialogflowConversationProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tts_config` after provisioning.\n"]
    pub fn tts_config(&self) -> ListRef<DialogflowConversationProfileTtsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tts_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileAutomatedAgentConfigEl {
    agent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_ttl: Option<PrimField<String>>,
}
impl DialogflowConversationProfileAutomatedAgentConfigEl {
    #[doc = "Set the field `session_ttl`.\nConfigure lifetime of the Dialogflow session."]
    pub fn set_session_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileAutomatedAgentConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileAutomatedAgentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileAutomatedAgentConfigEl {
    #[doc = "ID of the Dialogflow agent environment to use.\nExpects the format \"projects/<Project ID>/locations/<Location ID>/agent/environments/<EnvironmentID>\""]
    pub agent: PrimField<String>,
}
impl BuildDialogflowConversationProfileAutomatedAgentConfigEl {
    pub fn build(self) -> DialogflowConversationProfileAutomatedAgentConfigEl {
        DialogflowConversationProfileAutomatedAgentConfigEl {
            agent: self.agent,
            session_ttl: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileAutomatedAgentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileAutomatedAgentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileAutomatedAgentConfigElRef {
        DialogflowConversationProfileAutomatedAgentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileAutomatedAgentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\nID of the Dialogflow agent environment to use.\nExpects the format \"projects/<Project ID>/locations/<Location ID>/agent/environments/<EnvironmentID>\""]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
    #[doc = "Get a reference to the value of field `session_ttl` after provisioning.\nConfigure lifetime of the Dialogflow session."]
    pub fn session_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_model_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl { # [doc = "Set the field `baseline_model_version`.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"] pub fn set_baseline_model_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . baseline_model_version = Some (v . into ()) ; self } # [doc = "Set the field `model`.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."] pub fn set_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . model = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl { baseline_model_version : core :: default :: Default :: default () , model : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `baseline_model_version` after provisioning.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"] pub fn baseline_model_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.baseline_model_version" , self . base)) } # [doc = "Get a reference to the value of field `model` after provisioning.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."] pub fn model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.model" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_sentences_count: Option<PrimField<f64>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { # [doc = "Set the field `recent_sentences_count`.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"] pub fn set_recent_sentences_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . recent_sentences_count = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { recent_sentences_count : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `recent_sentences_count` after provisioning.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"] pub fn recent_sentences_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.recent_sentences_count" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_handoff_messages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_ivr_messages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_virtual_agent_messages: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { # [doc = "Set the field `drop_handoff_messages`.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."] pub fn set_drop_handoff_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_handoff_messages = Some (v . into ()) ; self } # [doc = "Set the field `drop_ivr_messages`.\nIf set to true, all messages from ivr stage are dropped."] pub fn set_drop_ivr_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_ivr_messages = Some (v . into ()) ; self } # [doc = "Set the field `drop_virtual_agent_messages`.\nIf set to true, all messages from virtual agent are dropped."] pub fn set_drop_virtual_agent_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_virtual_agent_messages = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { drop_handoff_messages : core :: default :: Default :: default () , drop_ivr_messages : core :: default :: Default :: default () , drop_virtual_agent_messages : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `drop_handoff_messages` after provisioning.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."] pub fn drop_handoff_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_handoff_messages" , self . base)) } # [doc = "Get a reference to the value of field `drop_ivr_messages` after provisioning.\nIf set to true, all messages from ivr stage are dropped."] pub fn drop_ivr_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_ivr_messages" , self . base)) } # [doc = "Get a reference to the value of field `drop_virtual_agent_messages` after provisioning.\nIf set to true, all messages from virtual agent are dropped."] pub fn drop_virtual_agent_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_virtual_agent_messages" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { # [doc = "Set the field `agent`.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn set_agent (mut self , v : impl Into < PrimField < String > >) -> Self { self . agent = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { agent : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `agent` after provisioning.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn agent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.agent" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElDynamic { human_agent_side_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { agent : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] human_agent_side_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { # [doc = "Set the field `human_agent_side_config`.\n"] pub fn set_human_agent_side_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . human_agent_side_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . human_agent_side_config = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl
{
    #[doc = "he name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub agent: PrimField<String>,
}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { agent : self . agent , human_agent_side_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `agent` after provisioning.\nhe name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn agent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.agent" , self . base)) } # [doc = "Get a reference to the value of field `human_agent_side_config` after provisioning.\n"] pub fn human_agent_side_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.human_agent_side_config" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl
{
    documents: ListField<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl { }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl
{
    #[doc = "Knowledge documents to query from. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<KnowledgeBase ID>/documents/<Document ID>."]
    pub documents: ListField<PrimField<String>>,
}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl { documents : self . documents , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `documents` after provisioning.\nKnowledge documents to query from. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<KnowledgeBase ID>/documents/<Document ID>."] pub fn documents (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.documents" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl
{
    knowledge_bases: ListField<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl { }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl
{
    #[doc = "Knowledge bases to query. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<Knowledge Base ID>."]
    pub knowledge_bases: ListField<PrimField<String>>,
}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl { knowledge_bases : self . knowledge_bases , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\nKnowledge bases to query. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<Knowledge Base ID>."] pub fn knowledge_bases (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.knowledge_bases" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    section_types: Option<ListField<PrimField<String>>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { # [doc = "Set the field `section_types`.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"] pub fn set_section_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . section_types = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { section_types : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `section_types` after provisioning.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"] pub fn section_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.section_types" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDynamic { context_filter_settings : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl >> , dialogflow_query_source : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl >> , document_query_source : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl >> , knowledge_base_query_source : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl >> , sections : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { # [serde (skip_serializing_if = "Option::is_none")] confidence_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_results : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] context_filter_settings : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dialogflow_query_source : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] document_query_source : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] knowledge_base_query_source : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] sections : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { # [doc = "Set the field `confidence_threshold`.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."] pub fn set_confidence_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . confidence_threshold = Some (v . into ()) ; self } # [doc = "Set the field `max_results`.\nMaximum number of results to return."] pub fn set_max_results (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_results = Some (v . into ()) ; self } # [doc = "Set the field `context_filter_settings`.\n"] pub fn set_context_filter_settings (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . context_filter_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . context_filter_settings = Some (d) ; } } self } # [doc = "Set the field `dialogflow_query_source`.\n"] pub fn set_dialogflow_query_source (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dialogflow_query_source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dialogflow_query_source = Some (d) ; } } self } # [doc = "Set the field `document_query_source`.\n"] pub fn set_document_query_source (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . document_query_source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . document_query_source = Some (d) ; } } self } # [doc = "Set the field `knowledge_base_query_source`.\n"] pub fn set_knowledge_base_query_source (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . knowledge_base_query_source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . knowledge_base_query_source = Some (d) ; } } self } # [doc = "Set the field `sections`.\n"] pub fn set_sections (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . sections = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . sections = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl { confidence_threshold : core :: default :: Default :: default () , max_results : core :: default :: Default :: default () , context_filter_settings : core :: default :: Default :: default () , dialogflow_query_source : core :: default :: Default :: default () , document_query_source : core :: default :: Default :: default () , knowledge_base_query_source : core :: default :: Default :: default () , sections : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."] pub fn confidence_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.confidence_threshold" , self . base)) } # [doc = "Get a reference to the value of field `max_results` after provisioning.\nMaximum number of results to return."] pub fn max_results (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_results" , self . base)) } # [doc = "Get a reference to the value of field `context_filter_settings` after provisioning.\n"] pub fn context_filter_settings (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.context_filter_settings" , self . base)) } # [doc = "Get a reference to the value of field `dialogflow_query_source` after provisioning.\n"] pub fn dialogflow_query_source (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dialogflow_query_source" , self . base)) } # [doc = "Get a reference to the value of field `document_query_source` after provisioning.\n"] pub fn document_query_source (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElDocumentQuerySourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.document_query_source" , self . base)) } # [doc = "Get a reference to the value of field `knowledge_base_query_source` after provisioning.\n"] pub fn knowledge_base_query_source (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElKnowledgeBaseQuerySourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.knowledge_base_query_source" , self . base)) } # [doc = "Get a reference to the value of field `sections` after provisioning.\n"] pub fn sections (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.sections" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl
{
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { # [doc = "Set the field `type_`.\nType of Human Agent Assistant API feature to request."] pub fn set_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_ = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { type_ : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_` after provisioning.\nType of Human Agent Assistant API feature to request."] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    no_small_talk: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_end_user: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { # [doc = "Set the field `no_small_talk`.\nDo not trigger if last utterance is small talk."] pub fn set_no_small_talk (mut self , v : impl Into < PrimField < bool > >) -> Self { self . no_small_talk = Some (v . into ()) ; self } # [doc = "Set the field `only_end_user`.\nOnly trigger suggestion if participant role of last utterance is END_USER."] pub fn set_only_end_user (mut self , v : impl Into < PrimField < bool > >) -> Self { self . only_end_user = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { no_small_talk : core :: default :: Default :: default () , only_end_user : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `no_small_talk` after provisioning.\nDo not trigger if last utterance is small talk."] pub fn no_small_talk (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.no_small_talk" , self . base)) } # [doc = "Get a reference to the value of field `only_end_user` after provisioning.\nOnly trigger suggestion if participant role of last utterance is END_USER."] pub fn only_end_user (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.only_end_user" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElDynamic { conversation_model_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl >> , conversation_process_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl >> , query_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl >> , suggestion_feature : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl >> , suggestion_trigger_settings : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { # [serde (skip_serializing_if = "Option::is_none")] disable_agent_query_logging : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_conversation_augmented_query : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_event_based_suggestion : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_query_suggestion_only : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_query_suggestion_when_no_answer : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_model_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_process_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] query_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] suggestion_feature : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl > > , # [serde (skip_serializing_if = "Option::is_none")] suggestion_trigger_settings : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { # [doc = "Set the field `disable_agent_query_logging`.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn set_disable_agent_query_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . disable_agent_query_logging = Some (v . into ()) ; self } # [doc = "Set the field `enable_conversation_augmented_query`.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn set_enable_conversation_augmented_query (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_conversation_augmented_query = Some (v . into ()) ; self } # [doc = "Set the field `enable_event_based_suggestion`.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."] pub fn set_enable_event_based_suggestion (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_event_based_suggestion = Some (v . into ()) ; self } # [doc = "Set the field `enable_query_suggestion_only`.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"] pub fn set_enable_query_suggestion_only (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_query_suggestion_only = Some (v . into ()) ; self } # [doc = "Set the field `enable_query_suggestion_when_no_answer`.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."] pub fn set_enable_query_suggestion_when_no_answer (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_query_suggestion_when_no_answer = Some (v . into ()) ; self } # [doc = "Set the field `conversation_model_config`.\n"] pub fn set_conversation_model_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . conversation_model_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . conversation_model_config = Some (d) ; } } self } # [doc = "Set the field `conversation_process_config`.\n"] pub fn set_conversation_process_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . conversation_process_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . conversation_process_config = Some (d) ; } } self } # [doc = "Set the field `query_config`.\n"] pub fn set_query_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . query_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . query_config = Some (d) ; } } self } # [doc = "Set the field `suggestion_feature`.\n"] pub fn set_suggestion_feature (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . suggestion_feature = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . suggestion_feature = Some (d) ; } } self } # [doc = "Set the field `suggestion_trigger_settings`.\n"] pub fn set_suggestion_trigger_settings (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . suggestion_trigger_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . suggestion_trigger_settings = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl { disable_agent_query_logging : core :: default :: Default :: default () , enable_conversation_augmented_query : core :: default :: Default :: default () , enable_event_based_suggestion : core :: default :: Default :: default () , enable_query_suggestion_only : core :: default :: Default :: default () , enable_query_suggestion_when_no_answer : core :: default :: Default :: default () , conversation_model_config : core :: default :: Default :: default () , conversation_process_config : core :: default :: Default :: default () , query_config : core :: default :: Default :: default () , suggestion_feature : core :: default :: Default :: default () , suggestion_trigger_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `disable_agent_query_logging` after provisioning.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn disable_agent_query_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.disable_agent_query_logging" , self . base)) } # [doc = "Get a reference to the value of field `enable_conversation_augmented_query` after provisioning.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn enable_conversation_augmented_query (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_conversation_augmented_query" , self . base)) } # [doc = "Get a reference to the value of field `enable_event_based_suggestion` after provisioning.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."] pub fn enable_event_based_suggestion (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_event_based_suggestion" , self . base)) } # [doc = "Get a reference to the value of field `enable_query_suggestion_only` after provisioning.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"] pub fn enable_query_suggestion_only (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_query_suggestion_only" , self . base)) } # [doc = "Get a reference to the value of field `enable_query_suggestion_when_no_answer` after provisioning.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."] pub fn enable_query_suggestion_when_no_answer (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_query_suggestion_when_no_answer" , self . base)) } # [doc = "Get a reference to the value of field `conversation_model_config` after provisioning.\n"] pub fn conversation_model_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationModelConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.conversation_model_config" , self . base)) } # [doc = "Get a reference to the value of field `conversation_process_config` after provisioning.\n"] pub fn conversation_process_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.conversation_process_config" , self . base)) } # [doc = "Get a reference to the value of field `query_config` after provisioning.\n"] pub fn query_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElQueryConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.query_config" , self . base)) } # [doc = "Get a reference to the value of field `suggestion_feature` after provisioning.\n"] pub fn suggestion_feature (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.suggestion_feature" , self . base)) } # [doc = "Get a reference to the value of field `suggestion_trigger_settings` after provisioning.\n"] pub fn suggestion_trigger_settings (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.suggestion_trigger_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElDynamic { feature_configs : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl { # [serde (skip_serializing_if = "Option::is_none")] disable_high_latency_features_sync_delivery : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] generators : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] group_suggestion_responses : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] feature_configs : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl {
    #[doc = "Set the field `disable_high_latency_features_sync_delivery`.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn set_disable_high_latency_features_sync_delivery(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_high_latency_features_sync_delivery = Some(v.into());
        self
    }
    #[doc = "Set the field `generators`.\nList of various generator resource names used in the conversation profile."]
    pub fn set_generators(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.generators = Some(v.into());
        self
    }
    #[doc = "Set the field `group_suggestion_responses`.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn set_group_suggestion_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.group_suggestion_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_configs`.\n"]
    pub fn set_feature_configs(
        mut self,
        v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.feature_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl
{
    type O = BlockAssignable<
        DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl {
}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl {
    pub fn build(
        self,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl {
        DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl {
            disable_high_latency_features_sync_delivery: core::default::Default::default(),
            generators: core::default::Default::default(),
            group_suggestion_responses: core::default::Default::default(),
            feature_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef {
        DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_high_latency_features_sync_delivery` after provisioning.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn disable_high_latency_features_sync_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_latency_features_sync_delivery", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generators` after provisioning.\nList of various generator resource names used in the conversation profile."]
    pub fn generators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.generators", self.base))
    }
    #[doc = "Get a reference to the value of field `group_suggestion_responses` after provisioning.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn group_suggestion_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_suggestion_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_configs` after provisioning.\n"]    pub fn feature_configs (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElFeatureConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_model_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl { # [doc = "Set the field `baseline_model_version`.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"] pub fn set_baseline_model_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . baseline_model_version = Some (v . into ()) ; self } # [doc = "Set the field `model`.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."] pub fn set_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . model = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl { baseline_model_version : core :: default :: Default :: default () , model : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `baseline_model_version` after provisioning.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"] pub fn baseline_model_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.baseline_model_version" , self . base)) } # [doc = "Get a reference to the value of field `model` after provisioning.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."] pub fn model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.model" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_sentences_count: Option<PrimField<f64>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { # [doc = "Set the field `recent_sentences_count`.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"] pub fn set_recent_sentences_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . recent_sentences_count = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl { recent_sentences_count : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `recent_sentences_count` after provisioning.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"] pub fn recent_sentences_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.recent_sentences_count" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_handoff_messages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_ivr_messages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_virtual_agent_messages: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { # [doc = "Set the field `drop_handoff_messages`.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."] pub fn set_drop_handoff_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_handoff_messages = Some (v . into ()) ; self } # [doc = "Set the field `drop_ivr_messages`.\nIf set to true, all messages from ivr stage are dropped."] pub fn set_drop_ivr_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_ivr_messages = Some (v . into ()) ; self } # [doc = "Set the field `drop_virtual_agent_messages`.\nIf set to true, all messages from virtual agent are dropped."] pub fn set_drop_virtual_agent_messages (mut self , v : impl Into < PrimField < bool > >) -> Self { self . drop_virtual_agent_messages = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl { drop_handoff_messages : core :: default :: Default :: default () , drop_ivr_messages : core :: default :: Default :: default () , drop_virtual_agent_messages : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `drop_handoff_messages` after provisioning.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."] pub fn drop_handoff_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_handoff_messages" , self . base)) } # [doc = "Get a reference to the value of field `drop_ivr_messages` after provisioning.\nIf set to true, all messages from ivr stage are dropped."] pub fn drop_ivr_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_ivr_messages" , self . base)) } # [doc = "Get a reference to the value of field `drop_virtual_agent_messages` after provisioning.\nIf set to true, all messages from virtual agent are dropped."] pub fn drop_virtual_agent_messages (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.drop_virtual_agent_messages" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { # [doc = "Set the field `agent`.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn set_agent (mut self , v : impl Into < PrimField < String > >) -> Self { self . agent = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl { agent : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `agent` after provisioning.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn agent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.agent" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElDynamic { human_agent_side_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { agent : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] human_agent_side_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { # [doc = "Set the field `human_agent_side_config`.\n"] pub fn set_human_agent_side_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . human_agent_side_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . human_agent_side_config = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl
{
    #[doc = "he name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub agent: PrimField<String>,
}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl { agent : self . agent , human_agent_side_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `agent` after provisioning.\nhe name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."] pub fn agent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.agent" , self . base)) } # [doc = "Get a reference to the value of field `human_agent_side_config` after provisioning.\n"] pub fn human_agent_side_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElHumanAgentSideConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.human_agent_side_config" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    section_types: Option<ListField<PrimField<String>>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { # [doc = "Set the field `section_types`.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"] pub fn set_section_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . section_types = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl { section_types : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `section_types` after provisioning.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"] pub fn section_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.section_types" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDynamic { context_filter_settings : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl >> , dialogflow_query_source : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl >> , sections : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { # [serde (skip_serializing_if = "Option::is_none")] confidence_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_results : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] context_filter_settings : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dialogflow_query_source : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] sections : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { # [doc = "Set the field `confidence_threshold`.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."] pub fn set_confidence_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . confidence_threshold = Some (v . into ()) ; self } # [doc = "Set the field `max_results`.\nMaximum number of results to return."] pub fn set_max_results (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_results = Some (v . into ()) ; self } # [doc = "Set the field `context_filter_settings`.\n"] pub fn set_context_filter_settings (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . context_filter_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . context_filter_settings = Some (d) ; } } self } # [doc = "Set the field `dialogflow_query_source`.\n"] pub fn set_dialogflow_query_source (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dialogflow_query_source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dialogflow_query_source = Some (d) ; } } self } # [doc = "Set the field `sections`.\n"] pub fn set_sections (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . sections = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . sections = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl { confidence_threshold : core :: default :: Default :: default () , max_results : core :: default :: Default :: default () , context_filter_settings : core :: default :: Default :: default () , dialogflow_query_source : core :: default :: Default :: default () , sections : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."] pub fn confidence_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.confidence_threshold" , self . base)) } # [doc = "Get a reference to the value of field `max_results` after provisioning.\nMaximum number of results to return."] pub fn max_results (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_results" , self . base)) } # [doc = "Get a reference to the value of field `context_filter_settings` after provisioning.\n"] pub fn context_filter_settings (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElContextFilterSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.context_filter_settings" , self . base)) } # [doc = "Get a reference to the value of field `dialogflow_query_source` after provisioning.\n"] pub fn dialogflow_query_source (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElDialogflowQuerySourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dialogflow_query_source" , self . base)) } # [doc = "Get a reference to the value of field `sections` after provisioning.\n"] pub fn sections (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElSectionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.sections" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl
{
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { # [doc = "Set the field `type_`.\nType of Human Agent Assistant API feature to request."] pub fn set_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_ = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl { type_ : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_` after provisioning.\nType of Human Agent Assistant API feature to request."] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    no_small_talk: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_end_user: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { # [doc = "Set the field `no_small_talk`.\nDo not trigger if last utterance is small talk."] pub fn set_no_small_talk (mut self , v : impl Into < PrimField < bool > >) -> Self { self . no_small_talk = Some (v . into ()) ; self } # [doc = "Set the field `only_end_user`.\nOnly trigger suggestion if participant role of last utterance is END_USER."] pub fn set_only_end_user (mut self , v : impl Into < PrimField < bool > >) -> Self { self . only_end_user = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl { no_small_talk : core :: default :: Default :: default () , only_end_user : core :: default :: Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `no_small_talk` after provisioning.\nDo not trigger if last utterance is small talk."] pub fn no_small_talk (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.no_small_talk" , self . base)) } # [doc = "Get a reference to the value of field `only_end_user` after provisioning.\nOnly trigger suggestion if participant role of last utterance is END_USER."] pub fn only_end_user (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.only_end_user" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElDynamic { conversation_model_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl >> , conversation_process_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl >> , query_config : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl >> , suggestion_feature : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl >> , suggestion_trigger_settings : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { # [serde (skip_serializing_if = "Option::is_none")] disable_agent_query_logging : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_conversation_augmented_query : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_event_based_suggestion : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_query_suggestion_only : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enable_query_suggestion_when_no_answer : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_model_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_process_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] query_config : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] suggestion_feature : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl > > , # [serde (skip_serializing_if = "Option::is_none")] suggestion_trigger_settings : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { # [doc = "Set the field `disable_agent_query_logging`.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn set_disable_agent_query_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . disable_agent_query_logging = Some (v . into ()) ; self } # [doc = "Set the field `enable_conversation_augmented_query`.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn set_enable_conversation_augmented_query (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_conversation_augmented_query = Some (v . into ()) ; self } # [doc = "Set the field `enable_event_based_suggestion`.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."] pub fn set_enable_event_based_suggestion (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_event_based_suggestion = Some (v . into ()) ; self } # [doc = "Set the field `enable_query_suggestion_only`.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"] pub fn set_enable_query_suggestion_only (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_query_suggestion_only = Some (v . into ()) ; self } # [doc = "Set the field `enable_query_suggestion_when_no_answer`.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."] pub fn set_enable_query_suggestion_when_no_answer (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_query_suggestion_when_no_answer = Some (v . into ()) ; self } # [doc = "Set the field `conversation_model_config`.\n"] pub fn set_conversation_model_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . conversation_model_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . conversation_model_config = Some (d) ; } } self } # [doc = "Set the field `conversation_process_config`.\n"] pub fn set_conversation_process_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . conversation_process_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . conversation_process_config = Some (d) ; } } self } # [doc = "Set the field `query_config`.\n"] pub fn set_query_config (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . query_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . query_config = Some (d) ; } } self } # [doc = "Set the field `suggestion_feature`.\n"] pub fn set_suggestion_feature (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . suggestion_feature = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . suggestion_feature = Some (d) ; } } self } # [doc = "Set the field `suggestion_trigger_settings`.\n"] pub fn set_suggestion_trigger_settings (mut self , v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . suggestion_trigger_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . suggestion_trigger_settings = Some (d) ; } } self } }
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { type O = BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { pub fn build (self) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl { disable_agent_query_logging : core :: default :: Default :: default () , enable_conversation_augmented_query : core :: default :: Default :: default () , enable_event_based_suggestion : core :: default :: Default :: default () , enable_query_suggestion_only : core :: default :: Default :: default () , enable_query_suggestion_when_no_answer : core :: default :: Default :: default () , conversation_model_config : core :: default :: Default :: default () , conversation_process_config : core :: default :: Default :: default () , query_config : core :: default :: Default :: default () , suggestion_feature : core :: default :: Default :: default () , suggestion_trigger_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef { fn new (shared : StackShared , base : String) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef { DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `disable_agent_query_logging` after provisioning.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn disable_agent_query_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.disable_agent_query_logging" , self . base)) } # [doc = "Get a reference to the value of field `enable_conversation_augmented_query` after provisioning.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."] pub fn enable_conversation_augmented_query (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_conversation_augmented_query" , self . base)) } # [doc = "Get a reference to the value of field `enable_event_based_suggestion` after provisioning.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."] pub fn enable_event_based_suggestion (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_event_based_suggestion" , self . base)) } # [doc = "Get a reference to the value of field `enable_query_suggestion_only` after provisioning.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"] pub fn enable_query_suggestion_only (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_query_suggestion_only" , self . base)) } # [doc = "Get a reference to the value of field `enable_query_suggestion_when_no_answer` after provisioning.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."] pub fn enable_query_suggestion_when_no_answer (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_query_suggestion_when_no_answer" , self . base)) } # [doc = "Get a reference to the value of field `conversation_model_config` after provisioning.\n"] pub fn conversation_model_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationModelConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.conversation_model_config" , self . base)) } # [doc = "Get a reference to the value of field `conversation_process_config` after provisioning.\n"] pub fn conversation_process_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElConversationProcessConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.conversation_process_config" , self . base)) } # [doc = "Get a reference to the value of field `query_config` after provisioning.\n"] pub fn query_config (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElQueryConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.query_config" , self . base)) } # [doc = "Get a reference to the value of field `suggestion_feature` after provisioning.\n"] pub fn suggestion_feature (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionFeatureElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.suggestion_feature" , self . base)) } # [doc = "Get a reference to the value of field `suggestion_trigger_settings` after provisioning.\n"] pub fn suggestion_trigger_settings (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElSuggestionTriggerSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.suggestion_trigger_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElDynamic { feature_configs : Option < DynamicBlock < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl >> , }
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl { # [serde (skip_serializing_if = "Option::is_none")] disable_high_latency_features_sync_delivery : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] generators : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] group_suggestion_responses : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] feature_configs : Option < Vec < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl > > , dynamic : DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElDynamic , }
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl {
    #[doc = "Set the field `disable_high_latency_features_sync_delivery`.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn set_disable_high_latency_features_sync_delivery(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_high_latency_features_sync_delivery = Some(v.into());
        self
    }
    #[doc = "Set the field `generators`.\nList of various generator resource names used in the conversation profile."]
    pub fn set_generators(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.generators = Some(v.into());
        self
    }
    #[doc = "Set the field `group_suggestion_responses`.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn set_group_suggestion_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.group_suggestion_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_configs`.\n"]
    pub fn set_feature_configs(
        mut self,
        v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.feature_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl
{
    type O = BlockAssignable<
        DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl
{}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl {
    pub fn build(
        self,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl {
        DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl {
            disable_high_latency_features_sync_delivery: core::default::Default::default(),
            generators: core::default::Default::default(),
            group_suggestion_responses: core::default::Default::default(),
            feature_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef
    {
        DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_high_latency_features_sync_delivery` after provisioning.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn disable_high_latency_features_sync_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_latency_features_sync_delivery", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generators` after provisioning.\nList of various generator resource names used in the conversation profile."]
    pub fn generators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.generators", self.base))
    }
    #[doc = "Get a reference to the value of field `group_suggestion_responses` after provisioning.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn group_suggestion_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_suggestion_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_configs` after provisioning.\n"]    pub fn feature_configs (& self) -> ListRef < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElFeatureConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_entity_extraction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sentiment_analysis: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {
    #[doc = "Set the field `enable_entity_extraction`.\nEnable entity extraction in conversation messages on agent assist stage."]
    pub fn set_enable_entity_extraction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_entity_extraction = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_sentiment_analysis`.\nEnable sentiment analysis in conversation messages on agent assist stage. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral."]
    pub fn set_enable_sentiment_analysis(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl
{
    type O = BlockAssignable<
        DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {
    pub fn build(
        self,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {
        DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl {
            enable_entity_extraction: core::default::Default::default(),
            enable_sentiment_analysis: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef {
        DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_entity_extraction` after provisioning.\nEnable entity extraction in conversation messages on agent assist stage."]
    pub fn enable_entity_extraction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_entity_extraction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\nEnable sentiment analysis in conversation messages on agent assist stage. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral."]
    pub fn enable_sentiment_analysis(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sentiment_analysis", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {
    #[doc = "Set the field `message_format`.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl
{
    type O = BlockAssignable<
        DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {
    pub fn build(
        self,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {
        DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef {
        DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentAssistantConfigElDynamic {
    end_user_suggestion_config: Option<
        DynamicBlock<
            DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl,
        >,
    >,
    human_agent_suggestion_config: Option<
        DynamicBlock<
            DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl,
        >,
    >,
    message_analysis_config: Option<
        DynamicBlock<
            DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl,
        >,
    >,
    notification_config: Option<
        DynamicBlock<DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentAssistantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_user_suggestion_config: Option<
        Vec<DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_agent_suggestion_config: Option<
        Vec<DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_analysis_config: Option<
        Vec<DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config:
        Option<Vec<DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl>>,
    dynamic: DialogflowConversationProfileHumanAgentAssistantConfigElDynamic,
}
impl DialogflowConversationProfileHumanAgentAssistantConfigEl {
    #[doc = "Set the field `end_user_suggestion_config`.\n"]
    pub fn set_end_user_suggestion_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_user_suggestion_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_user_suggestion_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `human_agent_suggestion_config`.\n"]
    pub fn set_human_agent_suggestion_config(
        mut self,
        v : impl Into < BlockAssignable < DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.human_agent_suggestion_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.human_agent_suggestion_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `message_analysis_config`.\n"]
    pub fn set_message_analysis_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_analysis_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_analysis_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowConversationProfileHumanAgentAssistantConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileHumanAgentAssistantConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentAssistantConfigEl {}
impl BuildDialogflowConversationProfileHumanAgentAssistantConfigEl {
    pub fn build(self) -> DialogflowConversationProfileHumanAgentAssistantConfigEl {
        DialogflowConversationProfileHumanAgentAssistantConfigEl {
            end_user_suggestion_config: core::default::Default::default(),
            human_agent_suggestion_config: core::default::Default::default(),
            message_analysis_config: core::default::Default::default(),
            notification_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentAssistantConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentAssistantConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentAssistantConfigElRef {
        DialogflowConversationProfileHumanAgentAssistantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentAssistantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_user_suggestion_config` after provisioning.\n"]
    pub fn end_user_suggestion_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentAssistantConfigElEndUserSuggestionConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.end_user_suggestion_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `human_agent_suggestion_config` after provisioning.\n"]
    pub fn human_agent_suggestion_config(
        &self,
    ) -> ListRef<
        DialogflowConversationProfileHumanAgentAssistantConfigElHumanAgentSuggestionConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_agent_suggestion_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_analysis_config` after provisioning.\n"]
    pub fn message_analysis_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentAssistantConfigElMessageAnalysisConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_analysis_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentAssistantConfigElNotificationConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
    account_number: PrimField<String>,
}
impl DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {}
impl ToListMappable for DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
    type O =
        BlockAssignable<DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
    #[doc = "Account number of the LivePerson account to connect."]
    pub account_number: PrimField<String>,
}
impl BuildDialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
    pub fn build(self) -> DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
        DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl {
            account_number: self.account_number,
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef {
        DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_number` after provisioning.\nAccount number of the LivePerson account to connect."]
    pub fn account_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_number", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowConversationProfileHumanAgentHandoffConfigElDynamic {
    live_person_config: Option<
        DynamicBlock<DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileHumanAgentHandoffConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    live_person_config:
        Option<Vec<DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl>>,
    dynamic: DialogflowConversationProfileHumanAgentHandoffConfigElDynamic,
}
impl DialogflowConversationProfileHumanAgentHandoffConfigEl {
    #[doc = "Set the field `live_person_config`.\n"]
    pub fn set_live_person_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_person_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_person_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowConversationProfileHumanAgentHandoffConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileHumanAgentHandoffConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileHumanAgentHandoffConfigEl {}
impl BuildDialogflowConversationProfileHumanAgentHandoffConfigEl {
    pub fn build(self) -> DialogflowConversationProfileHumanAgentHandoffConfigEl {
        DialogflowConversationProfileHumanAgentHandoffConfigEl {
            live_person_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileHumanAgentHandoffConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileHumanAgentHandoffConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileHumanAgentHandoffConfigElRef {
        DialogflowConversationProfileHumanAgentHandoffConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileHumanAgentHandoffConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `live_person_config` after provisioning.\n"]
    pub fn live_person_config(
        &self,
    ) -> ListRef<DialogflowConversationProfileHumanAgentHandoffConfigElLivePersonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_person_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileLoggingConfigEl {
    #[doc = "Set the field `enable_stackdriver_logging`.\nWhether to log conversation events"]
    pub fn set_enable_stackdriver_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_stackdriver_logging = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileLoggingConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileLoggingConfigEl {}
impl BuildDialogflowConversationProfileLoggingConfigEl {
    pub fn build(self) -> DialogflowConversationProfileLoggingConfigEl {
        DialogflowConversationProfileLoggingConfigEl {
            enable_stackdriver_logging: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DialogflowConversationProfileLoggingConfigElRef {
        DialogflowConversationProfileLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nWhether to log conversation events"]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileNewMessageEventNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DialogflowConversationProfileNewMessageEventNotificationConfigEl {
    #[doc = "Set the field `message_format`.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileNewMessageEventNotificationConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileNewMessageEventNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileNewMessageEventNotificationConfigEl {}
impl BuildDialogflowConversationProfileNewMessageEventNotificationConfigEl {
    pub fn build(self) -> DialogflowConversationProfileNewMessageEventNotificationConfigEl {
        DialogflowConversationProfileNewMessageEventNotificationConfigEl {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileNewMessageEventNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileNewMessageEventNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileNewMessageEventNotificationConfigElRef {
        DialogflowConversationProfileNewMessageEventNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileNewMessageEventNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
    #[doc = "Set the field `message_format`.\nFormat of message. Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos.\nFor telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant service-<Conversation Project Number>@gcp-sa-dialogflow.iam.gserviceaccount.com the Dialogflow Service Agent role in the topic project.\nFor chat integration to receive notification, make sure API caller has been granted the Dialogflow Service Agent role for the topic.\nFormat: projects/<Project ID>/locations/<Location ID>/topics/<Topic ID>."]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileNewRecognitionResultNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileNewRecognitionResultNotificationConfigEl {}
impl BuildDialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
    pub fn build(self) -> DialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
        DialogflowConversationProfileNewRecognitionResultNotificationConfigEl {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef {
        DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileNewRecognitionResultNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\nFormat of message. Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos.\nFor telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant service-<Conversation Project Number>@gcp-sa-dialogflow.iam.gserviceaccount.com the Dialogflow Service Agent role in the topic project.\nFor chat integration to receive notification, make sure API caller has been granted the Dialogflow Service Agent role for the topic.\nFormat: projects/<Project ID>/locations/<Location ID>/topics/<Topic ID>."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DialogflowConversationProfileNotificationConfigEl {
    #[doc = "Set the field `message_format`.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileNotificationConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileNotificationConfigEl {}
impl BuildDialogflowConversationProfileNotificationConfigEl {
    pub fn build(self) -> DialogflowConversationProfileNotificationConfigEl {
        DialogflowConversationProfileNotificationConfigEl {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileNotificationConfigElRef {
        DialogflowConversationProfileNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileSttConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_word_info: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate_hertz: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_model_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_timeout_based_endpointing: Option<PrimField<bool>>,
}
impl DialogflowConversationProfileSttConfigEl {
    #[doc = "Set the field `audio_encoding`.\nAudio encoding of the audio content to process. Possible values: [\"AUDIO_ENCODING_UNSPECIFIED\", \"AUDIO_ENCODING_LINEAR_16\", \"AUDIO_ENCODING_FLAC\", \"AUDIO_ENCODING_MULAW\", \"AUDIO_ENCODING_AMR\", \"AUDIO_ENCODING_AMR_WB\", \"AUDIO_ENCODING_OGG_OPUS\", \"AUDIOENCODING_SPEEX_WITH_HEADER_BYTE\"]"]
    pub fn set_audio_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_word_info`.\nIf true, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words."]
    pub fn set_enable_word_info(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_word_info = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nThe language of the supplied audio."]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `model`.\nWhich Speech model to select.\nLeave this field unspecified to use Agent Speech settings for model selection."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate_hertz`.\nSample rate (in Hertz) of the audio content sent in the query."]
    pub fn set_sample_rate_hertz(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate_hertz = Some(v.into());
        self
    }
    #[doc = "Set the field `speech_model_variant`.\nThe speech model used in speech to text. Possible values: [\"SPEECH_MODEL_VARIANT_UNSPECIFIED\", \"USE_BEST_AVAILABLE\", \"USE_STANDARD\", \"USE_ENHANCED\"]"]
    pub fn set_speech_model_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.speech_model_variant = Some(v.into());
        self
    }
    #[doc = "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivy as seconds of timeout value."]
    pub fn set_use_timeout_based_endpointing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_timeout_based_endpointing = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileSttConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileSttConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileSttConfigEl {}
impl BuildDialogflowConversationProfileSttConfigEl {
    pub fn build(self) -> DialogflowConversationProfileSttConfigEl {
        DialogflowConversationProfileSttConfigEl {
            audio_encoding: core::default::Default::default(),
            enable_word_info: core::default::Default::default(),
            language_code: core::default::Default::default(),
            model: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
            speech_model_variant: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileSttConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileSttConfigElRef {
    fn new(shared: StackShared, base: String) -> DialogflowConversationProfileSttConfigElRef {
        DialogflowConversationProfileSttConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileSttConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_encoding` after provisioning.\nAudio encoding of the audio content to process. Possible values: [\"AUDIO_ENCODING_UNSPECIFIED\", \"AUDIO_ENCODING_LINEAR_16\", \"AUDIO_ENCODING_FLAC\", \"AUDIO_ENCODING_MULAW\", \"AUDIO_ENCODING_AMR\", \"AUDIO_ENCODING_AMR_WB\", \"AUDIO_ENCODING_OGG_OPUS\", \"AUDIOENCODING_SPEEX_WITH_HEADER_BYTE\"]"]
    pub fn audio_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_encoding", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_word_info` after provisioning.\nIf true, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words."]
    pub fn enable_word_info(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_word_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the supplied audio."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nWhich Speech model to select.\nLeave this field unspecified to use Agent Speech settings for model selection."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nSample rate (in Hertz) of the audio content sent in the query."]
    pub fn sample_rate_hertz(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sample_rate_hertz", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speech_model_variant` after provisioning.\nThe speech model used in speech to text. Possible values: [\"SPEECH_MODEL_VARIANT_UNSPECIFIED\", \"USE_BEST_AVAILABLE\", \"USE_STANDARD\", \"USE_ENHANCED\"]"]
    pub fn speech_model_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.speech_model_variant", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivy as seconds of timeout value."]
    pub fn use_timeout_based_endpointing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_timeout_based_endpointing", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowConversationProfileTimeoutsEl {
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
impl ToListMappable for DialogflowConversationProfileTimeoutsEl {
    type O = BlockAssignable<DialogflowConversationProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileTimeoutsEl {}
impl BuildDialogflowConversationProfileTimeoutsEl {
    pub fn build(self) -> DialogflowConversationProfileTimeoutsEl {
        DialogflowConversationProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowConversationProfileTimeoutsElRef {
        DialogflowConversationProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileTimeoutsElRef {
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
pub struct DialogflowConversationProfileTtsConfigElVoiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml_gender: Option<PrimField<String>>,
}
impl DialogflowConversationProfileTtsConfigElVoiceEl {
    #[doc = "Set the field `name`.\nThe name of the voice."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `ssml_gender`.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn set_ssml_gender(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml_gender = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowConversationProfileTtsConfigElVoiceEl {
    type O = BlockAssignable<DialogflowConversationProfileTtsConfigElVoiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileTtsConfigElVoiceEl {}
impl BuildDialogflowConversationProfileTtsConfigElVoiceEl {
    pub fn build(self) -> DialogflowConversationProfileTtsConfigElVoiceEl {
        DialogflowConversationProfileTtsConfigElVoiceEl {
            name: core::default::Default::default(),
            ssml_gender: core::default::Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileTtsConfigElVoiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileTtsConfigElVoiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowConversationProfileTtsConfigElVoiceElRef {
        DialogflowConversationProfileTtsConfigElVoiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileTtsConfigElVoiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the voice."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `ssml_gender` after provisioning.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn ssml_gender(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml_gender", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowConversationProfileTtsConfigElDynamic {
    voice: Option<DynamicBlock<DialogflowConversationProfileTtsConfigElVoiceEl>>,
}
#[derive(Serialize)]
pub struct DialogflowConversationProfileTtsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effects_profile_id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speaking_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_gain_db: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<Vec<DialogflowConversationProfileTtsConfigElVoiceEl>>,
    dynamic: DialogflowConversationProfileTtsConfigElDynamic,
}
impl DialogflowConversationProfileTtsConfigEl {
    #[doc = "Set the field `effects_profile_id`.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn set_effects_profile_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.effects_profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `pitch`.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn set_pitch(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pitch = Some(v.into());
        self
    }
    #[doc = "Set the field `speaking_rate`.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn set_speaking_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.speaking_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_gain_db`.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn set_volume_gain_db(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_gain_db = Some(v.into());
        self
    }
    #[doc = "Set the field `voice`.\n"]
    pub fn set_voice(
        mut self,
        v: impl Into<BlockAssignable<DialogflowConversationProfileTtsConfigElVoiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.voice = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.voice = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowConversationProfileTtsConfigEl {
    type O = BlockAssignable<DialogflowConversationProfileTtsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowConversationProfileTtsConfigEl {}
impl BuildDialogflowConversationProfileTtsConfigEl {
    pub fn build(self) -> DialogflowConversationProfileTtsConfigEl {
        DialogflowConversationProfileTtsConfigEl {
            effects_profile_id: core::default::Default::default(),
            pitch: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
            voice: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowConversationProfileTtsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowConversationProfileTtsConfigElRef {
    fn new(shared: StackShared, base: String) -> DialogflowConversationProfileTtsConfigElRef {
        DialogflowConversationProfileTtsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowConversationProfileTtsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effects_profile_id` after provisioning.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn effects_profile_id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effects_profile_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pitch` after provisioning.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn pitch(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pitch", self.base))
    }
    #[doc = "Get a reference to the value of field `speaking_rate` after provisioning.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn speaking_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.speaking_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_gain_db` after provisioning.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn volume_gain_db(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_gain_db", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `voice` after provisioning.\n"]
    pub fn voice(&self) -> ListRef<DialogflowConversationProfileTtsConfigElVoiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowConversationProfileDynamic {
    automated_agent_config:
        Option<DynamicBlock<DialogflowConversationProfileAutomatedAgentConfigEl>>,
    human_agent_assistant_config:
        Option<DynamicBlock<DialogflowConversationProfileHumanAgentAssistantConfigEl>>,
    human_agent_handoff_config:
        Option<DynamicBlock<DialogflowConversationProfileHumanAgentHandoffConfigEl>>,
    logging_config: Option<DynamicBlock<DialogflowConversationProfileLoggingConfigEl>>,
    new_message_event_notification_config:
        Option<DynamicBlock<DialogflowConversationProfileNewMessageEventNotificationConfigEl>>,
    new_recognition_result_notification_config:
        Option<DynamicBlock<DialogflowConversationProfileNewRecognitionResultNotificationConfigEl>>,
    notification_config: Option<DynamicBlock<DialogflowConversationProfileNotificationConfigEl>>,
    stt_config: Option<DynamicBlock<DialogflowConversationProfileSttConfigEl>>,
    tts_config: Option<DynamicBlock<DialogflowConversationProfileTtsConfigEl>>,
}
