use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowIntentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_response_platforms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_context_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_fallback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ml_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_followup_intent_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset_contexts: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowIntentTimeoutsEl>,
}
struct DialogflowIntent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowIntentData>,
}
#[derive(Clone)]
pub struct DialogflowIntent(Rc<DialogflowIntent_>);
impl DialogflowIntent {
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
    #[doc = "Set the field `action`.\nThe name of the action associated with the intent.\nNote: The action name must not contain whitespaces."]
    pub fn set_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action = Some(v.into());
        self
    }
    #[doc = "Set the field `default_response_platforms`.\nThe list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED\n(i.e. default platform). Possible values: [\"FACEBOOK\", \"SLACK\", \"TELEGRAM\", \"KIK\", \"SKYPE\", \"LINE\", \"VIBER\", \"ACTIONS_ON_GOOGLE\", \"GOOGLE_HANGOUTS\"]"]
    pub fn set_default_response_platforms(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().default_response_platforms = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `events`.\nThe collection of event names that trigger the intent. If the collection of input contexts is not empty, all of\nthe contexts must be present in the active user session for an event to trigger this intent. See the\n[events reference](https://cloud.google.com/dialogflow/docs/events-overview) for more details."]
    pub fn set_events(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().events = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `input_context_names`.\nThe list of context names required for this intent to be triggered.\nFormat: projects/<Project ID>/agent/sessions/-/contexts/<Context ID>."]
    pub fn set_input_context_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().input_context_names = Some(v.into());
        self
    }
    #[doc = "Set the field `is_fallback`.\nIndicates whether this is a fallback intent."]
    pub fn set_is_fallback(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_fallback = Some(v.into());
        self
    }
    #[doc = "Set the field `ml_disabled`.\nIndicates whether Machine Learning is disabled for the intent.\nNote: If mlDisabled setting is set to true, then this intent is not taken into account during inference in ML\nONLY match mode. Also, auto-markup in the UI is turned off."]
    pub fn set_ml_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ml_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_followup_intent_name`.\nThe unique identifier of the parent intent in the chain of followup intents.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn set_parent_followup_intent_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_followup_intent_name = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority of this intent. Higher numbers represent higher priorities.\n  - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds\n  to the Normal priority in the console.\n  - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `reset_contexts`.\nIndicates whether to delete all contexts in the current session when this intent is matched."]
    pub fn set_reset_contexts(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reset_contexts = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook_state`.\nIndicates whether webhooks are enabled for the intent.\n* WEBHOOK_STATE_ENABLED: Webhook is enabled in the agent and in the intent.\n* WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING: Webhook is enabled in the agent and in the intent. Also, each slot\nfilling prompt is forwarded to the webhook. Possible values: [\"WEBHOOK_STATE_ENABLED\", \"WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING\"]"]
    pub fn set_webhook_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().webhook_state = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowIntentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe name of the action associated with the intent.\nNote: The action name must not contain whitespaces."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_response_platforms` after provisioning.\nThe list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED\n(i.e. default platform). Possible values: [\"FACEBOOK\", \"SLACK\", \"TELEGRAM\", \"KIK\", \"SKYPE\", \"LINE\", \"VIBER\", \"ACTIONS_ON_GOOGLE\", \"GOOGLE_HANGOUTS\"]"]
    pub fn default_response_platforms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_response_platforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this intent to be displayed on the console."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\nThe collection of event names that trigger the intent. If the collection of input contexts is not empty, all of\nthe contexts must be present in the active user session for an event to trigger this intent. See the\n[events reference](https://cloud.google.com/dialogflow/docs/events-overview) for more details."]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `followup_intent_info` after provisioning.\nInformation about all followup intents that have this intent as a direct or indirect parent. We populate this field\nonly in the output."]
    pub fn followup_intent_info(&self) -> ListRef<DialogflowIntentFollowupIntentInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.followup_intent_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_context_names` after provisioning.\nThe list of context names required for this intent to be triggered.\nFormat: projects/<Project ID>/agent/sessions/-/contexts/<Context ID>."]
    pub fn input_context_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_context_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fallback` after provisioning.\nIndicates whether this is a fallback intent."]
    pub fn is_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fallback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ml_disabled` after provisioning.\nIndicates whether Machine Learning is disabled for the intent.\nNote: If mlDisabled setting is set to true, then this intent is not taken into account during inference in ML\nONLY match mode. Also, auto-markup in the UI is turned off."]
    pub fn ml_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ml_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this intent.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_followup_intent_name` after provisioning.\nThe unique identifier of the parent intent in the chain of followup intents.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn parent_followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_followup_intent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this intent. Higher numbers represent higher priorities.\n  - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds\n  to the Normal priority in the console.\n  - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reset_contexts` after provisioning.\nIndicates whether to delete all contexts in the current session when this intent is matched."]
    pub fn reset_contexts(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reset_contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_followup_intent_name` after provisioning.\nThe unique identifier of the root intent in the chain of followup intents. It identifies the correct followup\nintents chain for this intent.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn root_followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_followup_intent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_state` after provisioning.\nIndicates whether webhooks are enabled for the intent.\n* WEBHOOK_STATE_ENABLED: Webhook is enabled in the agent and in the intent.\n* WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING: Webhook is enabled in the agent and in the intent. Also, each slot\nfilling prompt is forwarded to the webhook. Possible values: [\"WEBHOOK_STATE_ENABLED\", \"WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING\"]"]
    pub fn webhook_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowIntentTimeoutsElRef {
        DialogflowIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowIntent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowIntent {}
impl ToListMappable for DialogflowIntent {
    type O = ListRef<DialogflowIntentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowIntent_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_intent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowIntent {
    pub tf_id: String,
    #[doc = "The name of this intent to be displayed on the console."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowIntent {
    pub fn build(self, stack: &mut Stack) -> DialogflowIntent {
        let out = DialogflowIntent(Rc::new(DialogflowIntent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowIntentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: core::default::Default::default(),
                default_response_platforms: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                events: core::default::Default::default(),
                id: core::default::Default::default(),
                input_context_names: core::default::Default::default(),
                is_fallback: core::default::Default::default(),
                ml_disabled: core::default::Default::default(),
                parent_followup_intent_name: core::default::Default::default(),
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                reset_contexts: core::default::Default::default(),
                webhook_state: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowIntentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowIntentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowIntentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe name of the action associated with the intent.\nNote: The action name must not contain whitespaces."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_response_platforms` after provisioning.\nThe list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED\n(i.e. default platform). Possible values: [\"FACEBOOK\", \"SLACK\", \"TELEGRAM\", \"KIK\", \"SKYPE\", \"LINE\", \"VIBER\", \"ACTIONS_ON_GOOGLE\", \"GOOGLE_HANGOUTS\"]"]
    pub fn default_response_platforms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_response_platforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this intent to be displayed on the console."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\nThe collection of event names that trigger the intent. If the collection of input contexts is not empty, all of\nthe contexts must be present in the active user session for an event to trigger this intent. See the\n[events reference](https://cloud.google.com/dialogflow/docs/events-overview) for more details."]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `followup_intent_info` after provisioning.\nInformation about all followup intents that have this intent as a direct or indirect parent. We populate this field\nonly in the output."]
    pub fn followup_intent_info(&self) -> ListRef<DialogflowIntentFollowupIntentInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.followup_intent_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_context_names` after provisioning.\nThe list of context names required for this intent to be triggered.\nFormat: projects/<Project ID>/agent/sessions/-/contexts/<Context ID>."]
    pub fn input_context_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_context_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fallback` after provisioning.\nIndicates whether this is a fallback intent."]
    pub fn is_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fallback", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ml_disabled` after provisioning.\nIndicates whether Machine Learning is disabled for the intent.\nNote: If mlDisabled setting is set to true, then this intent is not taken into account during inference in ML\nONLY match mode. Also, auto-markup in the UI is turned off."]
    pub fn ml_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ml_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this intent.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_followup_intent_name` after provisioning.\nThe unique identifier of the parent intent in the chain of followup intents.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn parent_followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_followup_intent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this intent. Higher numbers represent higher priorities.\n  - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds\n  to the Normal priority in the console.\n  - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reset_contexts` after provisioning.\nIndicates whether to delete all contexts in the current session when this intent is matched."]
    pub fn reset_contexts(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reset_contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_followup_intent_name` after provisioning.\nThe unique identifier of the root intent in the chain of followup intents. It identifies the correct followup\nintents chain for this intent.\nFormat: projects/<Project ID>/agent/intents/<Intent ID>."]
    pub fn root_followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_followup_intent_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_state` after provisioning.\nIndicates whether webhooks are enabled for the intent.\n* WEBHOOK_STATE_ENABLED: Webhook is enabled in the agent and in the intent.\n* WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING: Webhook is enabled in the agent and in the intent. Also, each slot\nfilling prompt is forwarded to the webhook. Possible values: [\"WEBHOOK_STATE_ENABLED\", \"WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING\"]"]
    pub fn webhook_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowIntentTimeoutsElRef {
        DialogflowIntentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowIntentFollowupIntentInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    followup_intent_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_followup_intent_name: Option<PrimField<String>>,
}
impl DialogflowIntentFollowupIntentInfoEl {
    #[doc = "Set the field `followup_intent_name`.\n"]
    pub fn set_followup_intent_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.followup_intent_name = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_followup_intent_name`.\n"]
    pub fn set_parent_followup_intent_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent_followup_intent_name = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowIntentFollowupIntentInfoEl {
    type O = BlockAssignable<DialogflowIntentFollowupIntentInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowIntentFollowupIntentInfoEl {}
impl BuildDialogflowIntentFollowupIntentInfoEl {
    pub fn build(self) -> DialogflowIntentFollowupIntentInfoEl {
        DialogflowIntentFollowupIntentInfoEl {
            followup_intent_name: core::default::Default::default(),
            parent_followup_intent_name: core::default::Default::default(),
        }
    }
}
pub struct DialogflowIntentFollowupIntentInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowIntentFollowupIntentInfoElRef {
    fn new(shared: StackShared, base: String) -> DialogflowIntentFollowupIntentInfoElRef {
        DialogflowIntentFollowupIntentInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowIntentFollowupIntentInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `followup_intent_name` after provisioning.\n"]
    pub fn followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.followup_intent_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parent_followup_intent_name` after provisioning.\n"]
    pub fn parent_followup_intent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_followup_intent_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowIntentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowIntentTimeoutsEl {
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
impl ToListMappable for DialogflowIntentTimeoutsEl {
    type O = BlockAssignable<DialogflowIntentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowIntentTimeoutsEl {}
impl BuildDialogflowIntentTimeoutsEl {
    pub fn build(self) -> DialogflowIntentTimeoutsEl {
        DialogflowIntentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowIntentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowIntentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowIntentTimeoutsElRef {
        DialogflowIntentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowIntentTimeoutsElRef {
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
