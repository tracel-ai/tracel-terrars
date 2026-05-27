use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxPageData {
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
    transition_route_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<Vec<DialogflowCxPageAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_fulfillment: Option<Vec<DialogflowCxPageEntryFulfillmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_handlers: Option<Vec<DialogflowCxPageEventHandlersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    form: Option<Vec<DialogflowCxPageFormEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_connector_settings: Option<Vec<DialogflowCxPageKnowledgeConnectorSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxPageTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_routes: Option<Vec<DialogflowCxPageTransitionRoutesEl>>,
    dynamic: DialogflowCxPageDynamic,
}
struct DialogflowCxPage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxPageData>,
}
#[derive(Clone)]
pub struct DialogflowCxPage(Rc<DialogflowCxPage_>);
impl DialogflowCxPage {
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
    #[doc = "Set the field `language_code`.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_route_groups`.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn set_transition_route_groups(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().transition_route_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPageAdvancedSettingsEl>>,
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
    #[doc = "Set the field `entry_fulfillment`.\n"]
    pub fn set_entry_fulfillment(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_fulfillment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event_handlers`.\n"]
    pub fn set_event_handlers(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPageEventHandlersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_handlers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_handlers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `form`.\n"]
    pub fn set_form(self, v: impl Into<BlockAssignable<DialogflowCxPageFormEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().form = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.form = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `knowledge_connector_settings`.\n"]
    pub fn set_knowledge_connector_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxPageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_routes`.\n"]
    pub fn set_transition_routes(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPageTransitionRoutesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transition_routes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transition_routes = Some(d);
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_route_groups` after provisioning.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_route_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxPageAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_fulfillment` after provisioning.\n"]
    pub fn entry_fulfillment(&self) -> ListRef<DialogflowCxPageEntryFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> ListRef<DialogflowCxPageEventHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `form` after provisioning.\n"]
    pub fn form(&self) -> ListRef<DialogflowCxPageFormElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.form", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> ListRef<DialogflowCxPageTransitionRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_routes", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxPage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxPage {}
impl ToListMappable for DialogflowCxPage {
    type O = ListRef<DialogflowCxPageRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxPage_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_page".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxPage {
    pub tf_id: String,
    #[doc = "The human-readable name of the page, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxPage {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxPage {
        let out = DialogflowCxPage(Rc::new(DialogflowCxPage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxPageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                transition_route_groups: core::default::Default::default(),
                advanced_settings: core::default::Default::default(),
                entry_fulfillment: core::default::Default::default(),
                event_handlers: core::default::Default::default(),
                form: core::default::Default::default(),
                knowledge_connector_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transition_routes: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxPageRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxPageRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_route_groups` after provisioning.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_route_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxPageAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_fulfillment` after provisioning.\n"]
    pub fn entry_fulfillment(&self) -> ListRef<DialogflowCxPageEntryFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> ListRef<DialogflowCxPageEventHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `form` after provisioning.\n"]
    pub fn form(&self) -> ListRef<DialogflowCxPageFormElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.form", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> ListRef<DialogflowCxPageTransitionRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_routes", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}
impl DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
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
impl ToListMappable for DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    type O = BlockAssignable<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageAdvancedSettingsElDtmfSettingsEl {}
impl BuildDialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
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
#[derive(Serialize, Default)]
struct DialogflowCxPageAdvancedSettingsElDynamic {
    dtmf_settings: Option<DynamicBlock<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageAdvancedSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_settings: Option<Vec<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxPageAdvancedSettingsElDynamic,
}
impl DialogflowCxPageAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
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
}
impl ToListMappable for DialogflowCxPageAdvancedSettingsEl {
    type O = BlockAssignable<DialogflowCxPageAdvancedSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageAdvancedSettingsEl {}
impl BuildDialogflowCxPageAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxPageAdvancedSettingsEl {
        DialogflowCxPageAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageAdvancedSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageAdvancedSettingsElRef {
        DialogflowCxPageAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> ListRef<DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElConditionalCasesEl {}
impl BuildDialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
        DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
        DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: PrimField<String>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElTextEl {}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
        DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageEntryFulfillmentElMessagesElDynamic {
    conversation_success:
        Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>>,
    live_agent_handoff:
        Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    output_audio_text:
        Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    play_audio: Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call:
        Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>>,
    text: Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_success:
        Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_agent_handoff: Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_text: Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_audio: Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephony_transfer_call:
        Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageEntryFulfillmentElMessagesElDynamic,
}
impl DialogflowCxPageEntryFulfillmentElMessagesEl {
    #[doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_agent_handoff = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_audio_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.play_audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesEl {}
impl BuildDialogflowCxPageEntryFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesEl {
        DialogflowCxPageEntryFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElMessagesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageEntryFulfillmentElMessagesElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef> {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentElSetParameterActionsEl {}
impl BuildDialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
        DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageEntryFulfillmentElDynamic {
    conditional_cases: Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    messages: Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    set_parameter_actions:
        Option<DynamicBlock<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases: Option<Vec<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions: Option<Vec<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageEntryFulfillmentElDynamic,
}
impl DialogflowCxPageEntryFulfillmentEl {
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageEntryFulfillmentEl {
    type O = BlockAssignable<DialogflowCxPageEntryFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEntryFulfillmentEl {}
impl BuildDialogflowCxPageEntryFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentEl {
        DialogflowCxPageEntryFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageEntryFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEntryFulfillmentElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageEntryFulfillmentElRef {
        DialogflowCxPageEntryFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEntryFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElConditionalCasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> ListRef<DialogflowCxPageEntryFulfillmentElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    type O = BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    type O = BlockAssignable<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O =
        BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: PrimField<String>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {}
impl ToListMappable
    for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    type O = BlockAssignable<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
    {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    type O = BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: Option<
        DynamicBlock<
            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: Option<
        DynamicBlock<
            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: Option<
        DynamicBlock<
            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: Option<
        DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>,
    >,
    telephony_transfer_call: Option<
        DynamicBlock<
            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: Option<DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_success: Option<
        Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_agent_handoff: Option<
        Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_text:
        Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_audio:
        Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephony_transfer_call: Option<
        Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElDynamic,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    #[doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_agent_handoff = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_audio_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.play_audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    type O = BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> ListRef<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> ListRef<
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    type O =
        BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElTriggerFulfillmentElDynamic {
    conditional_cases:
        Option<DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    messages: Option<DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: Option<
        DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases:
        Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions:
        Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageEventHandlersElTriggerFulfillmentElDynamic,
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentEl {}
impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElDynamic {
    trigger_fulfillment: Option<DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment: Option<Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxPageEventHandlersElDynamic,
}
impl DialogflowCxPageEventHandlersEl {
    #[doc = "Set the field `event`.\nThe name of the event to handle."]
    pub fn set_event(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event = Some(v.into());
        self
    }
    #[doc = "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_flow = Some(v.into());
        self
    }
    #[doc = "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_page = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageEventHandlersEl {
    type O = BlockAssignable<DialogflowCxPageEventHandlersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageEventHandlersEl {}
impl BuildDialogflowCxPageEventHandlersEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersEl {
        DialogflowCxPageEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageEventHandlersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageEventHandlersElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageEventHandlersElRef {
        DialogflowCxPageEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageEventHandlersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\nThe name of the event to handle."]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this event handler."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }
    #[doc = "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}
impl DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
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
impl ToListMappable for DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    type O = BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {}
impl BuildDialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
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
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElAdvancedSettingsElDynamic {
    dtmf_settings:
        Option<DynamicBlock<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_settings: Option<Vec<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxPageFormElParametersElAdvancedSettingsElDynamic,
}
impl DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>,
        >,
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
}
impl ToListMappable for DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    type O = BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElAdvancedSettingsEl {}
impl BuildDialogflowCxPageFormElParametersElAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElAdvancedSettingsEl {
        DialogflowCxPageFormElParametersElAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
        DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(
        &self,
    ) -> ListRef<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl
{
    type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl { cases : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl { # [doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn set_metadata (mut self , v : impl Into < PrimField < String > >) -> Self { self . metadata = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl { # [doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn set_metadata (mut self , v : impl Into < PrimField < String > >) -> Self { self . metadata = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl { # [doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."] pub fn set_ssml (mut self , v : impl Into < PrimField < String > >) -> Self { self . ssml = Some (v . into ()) ; self } # [doc = "Set the field `text`.\nThe raw text to be synthesized."] pub fn set_text (mut self , v : impl Into < PrimField < String > >) -> Self { self . text = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl { ssml : core :: default :: Default :: default () , text : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."] pub fn ssml (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ssml" , self . base)) } # [doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl
{
    audio_uri: PrimField<String>,
}
impl
    DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl
{
}
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl
{
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl { audio_uri : self . audio_uri , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."] pub fn audio_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl
{
    phone_number: PrimField<String>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { phone_number : self . phone_number , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."] pub fn phone_number (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.phone_number" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl
{
    type O = BlockAssignable<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl
{}
impl
    BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl
{
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl
    {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElDynamic { conversation_success : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl >> , live_agent_handoff : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl >> , output_audio_text : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl >> , play_audio : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl >> , telephony_transfer_call : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl >> , text : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl { # [serde (skip_serializing_if = "Option::is_none")] channel : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] payload : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_success : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl > > , # [serde (skip_serializing_if = "Option::is_none")] live_agent_handoff : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl > > , # [serde (skip_serializing_if = "Option::is_none")] output_audio_text : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl > > , # [serde (skip_serializing_if = "Option::is_none")] play_audio : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl > > , # [serde (skip_serializing_if = "Option::is_none")] telephony_transfer_call : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl > > , # [serde (skip_serializing_if = "Option::is_none")] text : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl > > , dynamic : DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElDynamic , }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    #[doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_agent_handoff = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_audio_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.play_audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl
{
    type O = BlockAssignable<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef
    {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]    pub fn conversation_success (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]    pub fn live_agent_handoff (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]    pub fn output_audio_text (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]    pub fn play_audio (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef >{
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]    pub fn telephony_transfer_call (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]    pub fn text (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef >{
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl
    DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl
{
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl { parameter : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef { DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."] pub fn parameter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.parameter" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElDynamic { conditional_cases : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl >> , messages : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl >> , set_parameter_actions : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl { # [serde (skip_serializing_if = "Option::is_none")] return_partial_responses : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] tag : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] webhook : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conditional_cases : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl > > , # [serde (skip_serializing_if = "Option::is_none")] messages : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl > > , # [serde (skip_serializing_if = "Option::is_none")] set_parameter_actions : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl > > , dynamic : DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElDynamic , }
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    type O =
        BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]    pub fn conditional_cases (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]    pub fn set_parameter_actions (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl { # [doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."] pub fn set_cases (mut self , v : impl Into < PrimField < String > >) -> Self { self . cases = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl { cases : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."] pub fn cases (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cases" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl { # [doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn set_metadata (mut self , v : impl Into < PrimField < String > >) -> Self { self . metadata = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { # [doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn set_metadata (mut self , v : impl Into < PrimField < String > >) -> Self { self . metadata = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl { # [doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."] pub fn set_ssml (mut self , v : impl Into < PrimField < String > >) -> Self { self . ssml = Some (v . into ()) ; self } # [doc = "Set the field `text`.\nThe raw text to be synthesized."] pub fn set_text (mut self , v : impl Into < PrimField < String > >) -> Self { self . text = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl { ssml : core :: default :: Default :: default () , text : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."] pub fn ssml (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ssml" , self . base)) } # [doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl
{
    audio_uri: PrimField<String>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl
{
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { audio_uri : self . audio_uri , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."] pub fn audio_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    phone_number: PrimField<String>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { phone_number : self . phone_number , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."] pub fn phone_number (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.phone_number" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl { # [doc = "Set the field `text`.\nA collection of text responses."] pub fn set_text (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . text = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl { text : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."] pub fn text (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElDynamic { conversation_success : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl >> , live_agent_handoff : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >> , output_audio_text : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl >> , play_audio : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl >> , telephony_transfer_call : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >> , text : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { # [serde (skip_serializing_if = "Option::is_none")] channel : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] payload : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_success : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl > > , # [serde (skip_serializing_if = "Option::is_none")] live_agent_handoff : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > > , # [serde (skip_serializing_if = "Option::is_none")] output_audio_text : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl > > , # [serde (skip_serializing_if = "Option::is_none")] play_audio : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl > > , # [serde (skip_serializing_if = "Option::is_none")] telephony_transfer_call : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > > , # [serde (skip_serializing_if = "Option::is_none")] text : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl > > , dynamic : DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElDynamic , }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { # [doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."] pub fn set_channel (mut self , v : impl Into < PrimField < String > >) -> Self { self . channel = Some (v . into ()) ; self } # [doc = "Set the field `payload`.\nA custom, platform-specific payload."] pub fn set_payload (mut self , v : impl Into < PrimField < String > >) -> Self { self . payload = Some (v . into ()) ; self } # [doc = "Set the field `conversation_success`.\n"] pub fn set_conversation_success (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . conversation_success = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . conversation_success = Some (d) ; } } self } # [doc = "Set the field `live_agent_handoff`.\n"] pub fn set_live_agent_handoff (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . live_agent_handoff = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . live_agent_handoff = Some (d) ; } } self } # [doc = "Set the field `output_audio_text`.\n"] pub fn set_output_audio_text (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . output_audio_text = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . output_audio_text = Some (d) ; } } self } # [doc = "Set the field `play_audio`.\n"] pub fn set_play_audio (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . play_audio = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . play_audio = Some (d) ; } } self } # [doc = "Set the field `telephony_transfer_call`.\n"] pub fn set_telephony_transfer_call (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . telephony_transfer_call = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . telephony_transfer_call = Some (d) ; } } self } # [doc = "Set the field `text`.\n"] pub fn set_text (mut self , v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . text = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . text = Some (d) ; } } self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl { channel : core :: default :: Default :: default () , payload : core :: default :: Default :: default () , conversation_success : core :: default :: Default :: default () , live_agent_handoff : core :: default :: Default :: default () , output_audio_text : core :: default :: Default :: default () , play_audio : core :: default :: Default :: default () , telephony_transfer_call : core :: default :: Default :: default () , text : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."] pub fn channel (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.channel" , self . base)) } # [doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."] pub fn payload (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.payload" , self . base)) } # [doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"] pub fn conversation_success (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.conversation_success" , self . base)) } # [doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"] pub fn live_agent_handoff (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.live_agent_handoff" , self . base)) } # [doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"] pub fn output_audio_text (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.output_audio_text" , self . base)) } # [doc = "Get a reference to the value of field `play_audio` after provisioning.\n"] pub fn play_audio (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.play_audio" , self . base)) } # [doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"] pub fn telephony_transfer_call (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.telephony_transfer_call" , self . base)) } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl { # [doc = "Set the field `parameter`.\nDisplay name of the parameter."] pub fn set_parameter (mut self , v : impl Into < PrimField < String > >) -> Self { self . parameter = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl { type O = BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl
{}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl { pub fn build (self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl { parameter : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."] pub fn parameter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.parameter" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElDynamic { conditional_cases : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl >> , messages : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl >> , set_parameter_actions : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl { # [serde (skip_serializing_if = "Option::is_none")] return_partial_responses : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] tag : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] webhook : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conditional_cases : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl > > , # [serde (skip_serializing_if = "Option::is_none")] messages : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl > > , # [serde (skip_serializing_if = "Option::is_none")] set_parameter_actions : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl > > , dynamic : DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElDynamic , }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl
{
    type O = BlockAssignable<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl
{}
impl
    BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl
{
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl
    {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl { return_partial_responses : core :: default :: Default :: default () , tag : core :: default :: Default :: default () , webhook : core :: default :: Default :: default () , conditional_cases : core :: default :: Default :: default () , messages : core :: default :: Default :: default () , set_parameter_actions : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef { DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]    pub fn conditional_cases (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]    pub fn messages (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]    pub fn set_parameter_actions (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElDynamic { trigger_fulfillment : Option < DynamicBlock < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl { # [serde (skip_serializing_if = "Option::is_none")] event : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target_flow : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target_page : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] trigger_fulfillment : Option < Vec < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl > > , dynamic : DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElDynamic , }
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    #[doc = "Set the field `event`.\nThe name of the event to handle."]
    pub fn set_event(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event = Some(v.into());
        self
    }
    #[doc = "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_flow = Some(v.into());
        self
    }
    #[doc = "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_page = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    type O =
        BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\nThe name of the event to handle."]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this event handler."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }
    #[doc = "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]    pub fn trigger_fulfillment (& self) -> ListRef < DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElDynamic {
    initial_prompt_fulfillment: Option<
        DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>,
    >,
    reprompt_event_handlers: Option<
        DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_prompt_fulfillment:
        Option<Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reprompt_event_handlers:
        Option<Vec<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>>,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElDynamic,
}
impl DialogflowCxPageFormElParametersElFillBehaviorEl {
    #[doc = "Set the field `initial_prompt_fulfillment`.\n"]
    pub fn set_initial_prompt_fulfillment(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initial_prompt_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_prompt_fulfillment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reprompt_event_handlers`.\n"]
    pub fn set_reprompt_event_handlers(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reprompt_event_handlers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reprompt_event_handlers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorEl {
    type O = BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorEl {}
impl BuildDialogflowCxPageFormElParametersElFillBehaviorEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorEl {
        DialogflowCxPageFormElParametersElFillBehaviorEl {
            initial_prompt_fulfillment: core::default::Default::default(),
            reprompt_event_handlers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElFillBehaviorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElFillBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `initial_prompt_fulfillment` after provisioning.\n"]
    pub fn initial_prompt_fulfillment(
        &self,
    ) -> ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_prompt_fulfillment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reprompt_event_handlers` after provisioning.\n"]
    pub fn reprompt_event_handlers(
        &self,
    ) -> ListRef<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reprompt_event_handlers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElDynamic {
    advanced_settings: Option<DynamicBlock<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    fill_behavior: Option<DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_list: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redact: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<Vec<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_behavior: Option<Vec<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
    dynamic: DialogflowCxPageFormElParametersElDynamic,
}
impl DialogflowCxPageFormElParametersEl {
    #[doc = "Set the field `default_value`.\nThe default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub fn set_default_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_value = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe human-readable name of the parameter, unique within the form."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `entity_type`.\nThe entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub fn set_entity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `is_list`.\nIndicates whether the parameter represents a list of values."]
    pub fn set_is_list(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_list = Some(v.into());
        self
    }
    #[doc = "Set the field `redact`.\nIndicates whether the parameter content should be redacted in log.\nIf redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn set_redact(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.redact = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\nIndicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.\nRequired parameters must be filled before form filling concludes."]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fill_behavior`.\n"]
    pub fn set_fill_behavior(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fill_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fill_behavior = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageFormElParametersEl {
    type O = BlockAssignable<DialogflowCxPageFormElParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormElParametersEl {}
impl BuildDialogflowCxPageFormElParametersEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersEl {
        DialogflowCxPageFormElParametersEl {
            default_value: core::default::Default::default(),
            display_name: core::default::Default::default(),
            entity_type: core::default::Default::default(),
            is_list: core::default::Default::default(),
            redact: core::default::Default::default(),
            required: core::default::Default::default(),
            advanced_settings: core::default::Default::default(),
            fill_behavior: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElParametersElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageFormElParametersElRef {
        DialogflowCxPageFormElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\nThe default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the parameter, unique within the form."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\nThe entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub fn entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
    #[doc = "Get a reference to the value of field `is_list` after provisioning.\nIndicates whether the parameter represents a list of values."]
    pub fn is_list(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_list", self.base))
    }
    #[doc = "Get a reference to the value of field `redact` after provisioning.\nIndicates whether the parameter content should be redacted in log.\nIf redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn redact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redact", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nIndicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.\nRequired parameters must be filled before form filling concludes."]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(
        &self,
    ) -> ListRef<DialogflowCxPageFormElParametersElAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fill_behavior` after provisioning.\n"]
    pub fn fill_behavior(&self) -> ListRef<DialogflowCxPageFormElParametersElFillBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fill_behavior", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageFormElDynamic {
    parameters: Option<DynamicBlock<DialogflowCxPageFormElParametersEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageFormEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<DialogflowCxPageFormElParametersEl>>,
    dynamic: DialogflowCxPageFormElDynamic,
}
impl DialogflowCxPageFormEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageFormElParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageFormEl {
    type O = BlockAssignable<DialogflowCxPageFormEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageFormEl {}
impl BuildDialogflowCxPageFormEl {
    pub fn build(self) -> DialogflowCxPageFormEl {
        DialogflowCxPageFormEl {
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageFormElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageFormElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageFormElRef {
        DialogflowCxPageFormElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageFormElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DialogflowCxPageFormElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_processing_mode: Option<PrimField<String>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    #[doc = "Set the field `data_store`.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn set_data_store(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_store = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_type`.\nThe type of the connected data store.\n* PUBLIC_WEB: A data store that contains public web content.\n* UNSTRUCTURED: A data store that contains unstructured private data.\n* STRUCTURED: A data store that contains structured data (for example FAQ). Possible values: [\"PUBLIC_WEB\", \"UNSTRUCTURED\", \"STRUCTURED\"]"]
    pub fn set_data_store_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_store_type = Some(v.into());
        self
    }
    #[doc = "Set the field `document_processing_mode`.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\n* DOCUMENTS: Documents are processed as documents.\n* CHUNKS: Documents are converted to chunks. Possible values: [\"DOCUMENTS\", \"CHUNKS\"]"]
    pub fn set_document_processing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_processing_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    type O = BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    pub fn build(self) -> DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
        DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_type` after provisioning.\nThe type of the connected data store.\n* PUBLIC_WEB: A data store that contains public web content.\n* UNSTRUCTURED: A data store that contains unstructured private data.\n* STRUCTURED: A data store that contains structured data (for example FAQ). Possible values: [\"PUBLIC_WEB\", \"UNSTRUCTURED\", \"STRUCTURED\"]"]
    pub fn data_store_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\n* DOCUMENTS: Documents are processed as documents.\n* CHUNKS: Documents are converted to chunks. Possible values: [\"DOCUMENTS\", \"CHUNKS\"]"]
    pub fn document_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_processing_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpointing_timeout_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interdigit_timeout_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
{
    #[doc = "Set the field `enabled`.\nIf true, incoming audio is processed for DTMF (dual tone multi frequtectency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will de the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `endpointing_timeout_duration`.\nEndpoint timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn set_endpointing_timeout_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpointing_timeout_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_digit`.\nThe digit that terminates a DTMF digit sequence."]
    pub fn set_finish_digit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_digit = Some(v.into());
        self
    }
    #[doc = "Set the field `interdigit_timeout_duration`.\nInterdigit timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn set_interdigit_timeout_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interdigit_timeout_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `max_digits`.\nMax length of DTMF digits."]
    pub fn set_max_digits(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_digits = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { enabled : core :: default :: Default :: default () , endpointing_timeout_duration : core :: default :: Default :: default () , finish_digit : core :: default :: Default :: default () , interdigit_timeout_duration : core :: default :: Default :: default () , max_digits : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequtectency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will de the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."] pub fn enabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enabled" , self . base)) } # [doc = "Get a reference to the value of field `endpointing_timeout_duration` after provisioning.\nEndpoint timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn endpointing_timeout_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.endpointing_timeout_duration" , self . base)) } # [doc = "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."] pub fn finish_digit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.finish_digit" , self . base)) } # [doc = "Get a reference to the value of field `interdigit_timeout_duration` after provisioning.\nInterdigit timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn interdigit_timeout_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interdigit_timeout_duration" , self . base)) } # [doc = "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."] pub fn max_digits (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_digits" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_consent_based_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_interaction_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { # [doc = "Set the field `enable_consent_based_redaction`.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."] pub fn set_enable_consent_based_redaction (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_consent_based_redaction = Some (v . into ()) ; self } # [doc = "Set the field `enable_interaction_logging`.\nEnables DF Interaction logging."] pub fn set_enable_interaction_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_interaction_logging = Some (v . into ()) ; self } # [doc = "Set the field `enable_stackdriver_logging`.\nEnables Google Cloud Logging."] pub fn set_enable_stackdriver_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_stackdriver_logging = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { enable_consent_based_redaction : core :: default :: Default :: default () , enable_interaction_logging : core :: default :: Default :: default () , enable_stackdriver_logging : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_consent_based_redaction` after provisioning.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."] pub fn enable_consent_based_redaction (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_consent_based_redaction" , self . base)) } # [doc = "Get a reference to the value of field `enable_interaction_logging` after provisioning.\nEnables DF Interaction logging."] pub fn enable_interaction_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_interaction_logging" , self . base)) } # [doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nEnables Google Cloud Logging."] pub fn enable_stackdriver_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_stackdriver_logging" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    endpointer_sensitivity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    models: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_speech_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_timeout_based_endpointing: Option<PrimField<bool>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { # [doc = "Set the field `endpointer_sensitivity`.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."] pub fn set_endpointer_sensitivity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . endpointer_sensitivity = Some (v . into ()) ; self } # [doc = "Set the field `models`.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."] pub fn set_models (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . models = Some (v . into ()) ; self } # [doc = "Set the field `no_speech_timeout`.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn set_no_speech_timeout (mut self , v : impl Into < PrimField < String > >) -> Self { self . no_speech_timeout = Some (v . into ()) ; self } # [doc = "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."] pub fn set_use_timeout_based_endpointing (mut self , v : impl Into < PrimField < bool > >) -> Self { self . use_timeout_based_endpointing = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { endpointer_sensitivity : core :: default :: Default :: default () , models : core :: default :: Default :: default () , no_speech_timeout : core :: default :: Default :: default () , use_timeout_based_endpointing : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `endpointer_sensitivity` after provisioning.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."] pub fn endpointer_sensitivity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.endpointer_sensitivity" , self . base)) } # [doc = "Get a reference to the value of field `models` after provisioning.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."] pub fn models (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.models" , self . base)) } # [doc = "Get a reference to the value of field `no_speech_timeout` after provisioning.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn no_speech_timeout (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.no_speech_timeout" , self . base)) } # [doc = "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."] pub fn use_timeout_based_endpointing (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.use_timeout_based_endpointing" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDynamic { dtmf_settings : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl >> , logging_settings : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl >> , speech_settings : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] dtmf_settings : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] logging_settings : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] speech_settings : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl > > , dynamic : DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDynamic , }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl >>,
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
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]    pub fn dtmf_settings (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]    pub fn logging_settings (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]    pub fn speech_settings (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl {}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{}
impl
    BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl {}
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_playback_interruption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
{
    #[doc = "Set the field `allow_playback_interruption`.\n"]
    pub fn set_allow_playback_interruption(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_playback_interruption = Some(v.into());
        self
    }
    #[doc = "Set the field `audio`.\n"]
    pub fn set_audio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { allow_playback_interruption : core :: default :: Default :: default () , audio : core :: default :: Default :: default () , uri : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\n"] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `audio` after provisioning.\n"] pub fn audio (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl { # [serde (skip_serializing_if = "Option::is_none")] segments : Option < ListField < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > > , }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        mut self,
        v : impl Into < ListField < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > >,
    ) -> Self {
        self.segments = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
            segments: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]    pub fn segments (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl
{}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl {}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn set_ssml(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nThe raw text to be synthesized.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{}
impl
    BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl { ssml : core :: default :: Default :: default () , text : core :: default :: Default :: default () , }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn ssml(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl
{
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    phone_number: PrimField<String>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { type O = BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { pub fn build (self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { phone_number : self . phone_number , } } }
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn new (shared : StackShared , base : String) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."] pub fn phone_number (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.phone_number" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text response variants. If multiple variants are defined, only one text response variant is returned at runtime.\nrequired: true"]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text response variants. If multiple variants are defined, only one text response variant is returned at runtime.\nrequired: true"]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElDynamic { conversation_success : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl >> , knowledge_info_card : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl >> , live_agent_handoff : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >> , output_audio_text : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl >> , play_audio : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl >> , telephony_transfer_call : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >> , text : Option < DynamicBlock < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl { # [serde (skip_serializing_if = "Option::is_none")] channel : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] payload : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_success : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl > > , # [serde (skip_serializing_if = "Option::is_none")] knowledge_info_card : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] live_agent_handoff : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > > , # [serde (skip_serializing_if = "Option::is_none")] output_audio_text : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl > > , # [serde (skip_serializing_if = "Option::is_none")] play_audio : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl > > , # [serde (skip_serializing_if = "Option::is_none")] telephony_transfer_call : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > > , # [serde (skip_serializing_if = "Option::is_none")] text : Option < Vec < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl > > , dynamic : DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElDynamic , }
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
    #[doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `payload`.\nReturns a response containing a custom, platform-specific payload.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `knowledge_info_card`.\n"]
    pub fn set_knowledge_info_card(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.knowledge_info_card = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_info_card = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_agent_handoff = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_audio_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.play_audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
    type O =
        BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            knowledge_info_card: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `end_interaction` after provisioning.\nThis type has no fields.\nIndicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]    pub fn end_interaction (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.end_interaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mixed_audio` after provisioning.\nRepresents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via playAudio. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn mixed_audio(
        &self,
    ) -> ListRef<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.mixed_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nReturns a response containing a custom, platform-specific payload.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]    pub fn conversation_success (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_info_card` after provisioning.\n"]    pub fn knowledge_info_card (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_info_card", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]    pub fn live_agent_handoff (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]    pub fn output_audio_text (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]    pub fn telephony_transfer_call (& self) -> ListRef < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl
{
    type O = BlockAssignable<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl
{}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
    {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElDynamic {
    advanced_settings: Option<
        DynamicBlock<
            DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
        >,
    >,
    conditional_cases: Option<
        DynamicBlock<
            DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
        >,
    >,
    messages: Option<
        DynamicBlock<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>,
    >,
    set_parameter_actions: Option<
        DynamicBlock<
            DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_generative_fallback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<
        Vec<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases: Option<
        Vec<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages:
        Option<Vec<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions: Option<
        Vec<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl>,
    >,
    dynamic: DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElDynamic,
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    #[doc = "Set the field `enable_generative_fallback`.\nIf the flag is true, the agent will utilize LLM to generate a text response. If LLM generation fails, the defined responses in the fulfillment will be respected. This flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn set_enable_generative_fallback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_generative_fallback = Some(v.into());
        self
    }
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl {
            enable_generative_fallback: core::default::Default::default(),
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            advanced_settings: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_generative_fallback` after provisioning.\nIf the flag is true, the agent will utilize LLM to generate a text response. If LLM generation fails, the defined responses in the fulfillment will be respected. This flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn enable_generative_fallback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_generative_fallback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(
        &self,
    ) -> ListRef<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> ListRef<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<
        DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageKnowledgeConnectorSettingsElDynamic {
    data_store_connections:
        Option<DynamicBlock<DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl>>,
    trigger_fulfillment:
        Option<DynamicBlock<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageKnowledgeConnectorSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_connections:
        Option<Vec<DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment:
        Option<Vec<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxPageKnowledgeConnectorSettingsElDynamic,
}
impl DialogflowCxPageKnowledgeConnectorSettingsEl {
    #[doc = "Set the field `enabled`.\nWhether Knowledge Connector is enabled or not."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `target_flow`.\nThe target flow to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>.\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn set_target_flow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_flow = Some(v.into());
        self
    }
    #[doc = "Set the field `target_page`.\nThe target page to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>/pages/<PageID>.\nThe page must be in the same host flow (the flow that owns this 'KnowledgeConnectorSettings').\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn set_target_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_page = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_connections`.\n"]
    pub fn set_data_store_connections(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_connections = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageKnowledgeConnectorSettingsEl {
    type O = BlockAssignable<DialogflowCxPageKnowledgeConnectorSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageKnowledgeConnectorSettingsEl {}
impl BuildDialogflowCxPageKnowledgeConnectorSettingsEl {
    pub fn build(self) -> DialogflowCxPageKnowledgeConnectorSettingsEl {
        DialogflowCxPageKnowledgeConnectorSettingsEl {
            enabled: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            data_store_connections: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageKnowledgeConnectorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageKnowledgeConnectorSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageKnowledgeConnectorSettingsElRef {
        DialogflowCxPageKnowledgeConnectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageKnowledgeConnectorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether Knowledge Connector is enabled or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>.\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn target_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }
    #[doc = "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>/pages/<PageID>.\nThe page must be in the same host flow (the flow that owns this 'KnowledgeConnectorSettings').\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn target_page(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_connections` after provisioning.\n"]
    pub fn data_store_connections(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElDataStoreConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> ListRef<DialogflowCxPageKnowledgeConnectorSettingsElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxPageTimeoutsEl {
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
impl ToListMappable for DialogflowCxPageTimeoutsEl {
    type O = BlockAssignable<DialogflowCxPageTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTimeoutsEl {}
impl BuildDialogflowCxPageTimeoutsEl {
    pub fn build(self) -> DialogflowCxPageTimeoutsEl {
        DialogflowCxPageTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTimeoutsElRef {
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
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    type O =
        BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
    {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: PrimField<String>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
    {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
    {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    type O =
        BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_playback_interruption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: Option<
        DynamicBlock<
            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: Option<
        DynamicBlock<
            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: Option<
        DynamicBlock<
            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: Option<
        DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>,
    >,
    telephony_transfer_call: Option<
        DynamicBlock<
            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: Option<
        DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_success: Option<
        Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_agent_handoff: Option<
        Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_text: Option<
        Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_audio:
        Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephony_transfer_call: Option<
        Vec<
            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElDynamic,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[doc = "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.live_agent_handoff = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_audio_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.play_audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    type O = BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> ListRef<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> ListRef<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> ListRef<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl
{
    type O = BlockAssignable<
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElDynamic {
    conditional_cases: Option<
        DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>,
    >,
    messages:
        Option<DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: Option<
        DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases:
        Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions:
        Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElTriggerFulfillmentElDynamic,
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    #[doc = "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_partial_responses = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_cases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_parameter_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {}
impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_partial_responses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElDynamic {
    trigger_fulfillment:
        Option<DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment: Option<Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElDynamic,
}
impl DialogflowCxPageTransitionRoutesEl {
    #[doc = "Set the field `condition`.\nThe condition to evaluate against form parameters or session parameters.\nAt least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }
    #[doc = "Set the field `intent`.\nThe unique identifier of an Intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>. Indicates that the transition can only happen when the given intent is matched. At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn set_intent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.intent = Some(v.into());
        self
    }
    #[doc = "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_flow = Some(v.into());
        self
    }
    #[doc = "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_page = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPageTransitionRoutesEl {
    type O = BlockAssignable<DialogflowCxPageTransitionRoutesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPageTransitionRoutesEl {}
impl BuildDialogflowCxPageTransitionRoutesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesEl {
        DialogflowCxPageTransitionRoutesEl {
            condition: core::default::Default::default(),
            intent: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPageTransitionRoutesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPageTransitionRoutesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPageTransitionRoutesElRef {
        DialogflowCxPageTransitionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPageTransitionRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nThe condition to evaluate against form parameters or session parameters.\nAt least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `intent` after provisioning.\nThe unique identifier of an Intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>. Indicates that the transition can only happen when the given intent is matched. At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn intent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intent", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this transition route."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }
    #[doc = "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPageDynamic {
    advanced_settings: Option<DynamicBlock<DialogflowCxPageAdvancedSettingsEl>>,
    entry_fulfillment: Option<DynamicBlock<DialogflowCxPageEntryFulfillmentEl>>,
    event_handlers: Option<DynamicBlock<DialogflowCxPageEventHandlersEl>>,
    form: Option<DynamicBlock<DialogflowCxPageFormEl>>,
    knowledge_connector_settings:
        Option<DynamicBlock<DialogflowCxPageKnowledgeConnectorSettingsEl>>,
    transition_routes: Option<DynamicBlock<DialogflowCxPageTransitionRoutesEl>>,
}
