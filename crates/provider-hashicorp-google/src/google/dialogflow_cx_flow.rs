use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxFlowData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default_start_flow: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_route_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<Vec<DialogflowCxFlowAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_handlers: Option<Vec<DialogflowCxFlowEventHandlersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_connector_settings: Option<Vec<DialogflowCxFlowKnowledgeConnectorSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nlu_settings: Option<Vec<DialogflowCxFlowNluSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxFlowTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_routes: Option<Vec<DialogflowCxFlowTransitionRoutesEl>>,
    dynamic: DialogflowCxFlowDynamic,
}
struct DialogflowCxFlow_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxFlowData>,
}
#[derive(Clone)]
pub struct DialogflowCxFlow(Rc<DialogflowCxFlow_>);
impl DialogflowCxFlow {
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
    #[doc = "Set the field `description`.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_default_start_flow`.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn set_is_default_start_flow(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default_start_flow = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_route_groups`.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn set_transition_route_groups(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().transition_route_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxFlowAdvancedSettingsEl>>,
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
    #[doc = "Set the field `event_handlers`.\n"]
    pub fn set_event_handlers(
        self,
        v: impl Into<BlockAssignable<DialogflowCxFlowEventHandlersEl>>,
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
    #[doc = "Set the field `knowledge_connector_settings`.\n"]
    pub fn set_knowledge_connector_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsEl>>,
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
    #[doc = "Set the field `nlu_settings`.\n"]
    pub fn set_nlu_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxFlowNluSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nlu_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nlu_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxFlowTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_routes`.\n"]
    pub fn set_transition_routes(
        self,
        v: impl Into<BlockAssignable<DialogflowCxFlowTransitionRoutesEl>>,
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the flow."]
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
    #[doc = "Get a reference to the value of field `is_default_start_flow` after provisioning.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn is_default_start_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the flow.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
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
    #[doc = "Get a reference to the value of field `transition_route_groups` after provisioning.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_route_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxFlowAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> ListRef<DialogflowCxFlowEventHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> ListRef<DialogflowCxFlowNluSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nlu_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> ListRef<DialogflowCxFlowTransitionRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_routes", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxFlow {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxFlow {}
impl ToListMappable for DialogflowCxFlow {
    type O = ListRef<DialogflowCxFlowRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxFlow_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_flow".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxFlow {
    pub tf_id: String,
    #[doc = "The human-readable name of the flow."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxFlow {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxFlow {
        let out = DialogflowCxFlow(Rc::new(DialogflowCxFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxFlowData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                is_default_start_flow: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                transition_route_groups: core::default::Default::default(),
                advanced_settings: core::default::Default::default(),
                event_handlers: core::default::Default::default(),
                knowledge_connector_settings: core::default::Default::default(),
                nlu_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transition_routes: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxFlowRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxFlowRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the flow."]
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
    #[doc = "Get a reference to the value of field `is_default_start_flow` after provisioning.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn is_default_start_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default_start_flow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the flow.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
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
    #[doc = "Get a reference to the value of field `transition_route_groups` after provisioning.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_route_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxFlowAdvancedSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> ListRef<DialogflowCxFlowEventHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_connector_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> ListRef<DialogflowCxFlowNluSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nlu_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> ListRef<DialogflowCxFlowTransitionRoutesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transition_routes", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    #[doc = "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    type O = BlockAssignable<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {}
impl BuildDialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
        DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
        DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}
impl DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
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
impl ToListMappable for DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {}
impl BuildDialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
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
pub struct DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_consent_based_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_interaction_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
}
impl DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
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
impl ToListMappable for DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {}
impl BuildDialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
        DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl {
            enable_consent_based_redaction: core::default::Default::default(),
            enable_interaction_logging: core::default::Default::default(),
            enable_stackdriver_logging: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef {
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
pub struct DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpointer_sensitivity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    models: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_speech_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_timeout_based_endpointing: Option<PrimField<bool>>,
}
impl DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
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
impl ToListMappable for DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {}
impl BuildDialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
        DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl {
            endpointer_sensitivity: core::default::Default::default(),
            models: core::default::Default::default(),
            no_speech_timeout: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef {
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
struct DialogflowCxFlowAdvancedSettingsElDynamic {
    audio_export_gcs_destination:
        Option<DynamicBlock<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
    dtmf_settings: Option<DynamicBlock<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
    logging_settings: Option<DynamicBlock<DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl>>,
    speech_settings: Option<DynamicBlock<DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowAdvancedSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_export_gcs_destination:
        Option<Vec<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_settings: Option<Vec<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_settings: Option<Vec<DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_settings: Option<Vec<DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl>>,
    dynamic: DialogflowCxFlowAdvancedSettingsElDynamic,
}
impl DialogflowCxFlowAdvancedSettingsEl {
    #[doc = "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowAdvancedSettingsElLoggingSettingsEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowAdvancedSettingsElSpeechSettingsEl>>,
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
impl ToListMappable for DialogflowCxFlowAdvancedSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowAdvancedSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowAdvancedSettingsEl {}
impl BuildDialogflowCxFlowAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsEl {
        DialogflowCxFlowAdvancedSettingsEl {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxFlowAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowAdvancedSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowAdvancedSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> ListRef<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_export_gcs_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> ListRef<DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(
        &self,
    ) -> ListRef<DialogflowCxFlowAdvancedSettingsElLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]
    pub fn speech_settings(
        &self,
    ) -> ListRef<DialogflowCxFlowAdvancedSettingsElSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    type O = BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    type O = BlockAssignable<
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable<
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
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
    for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O =
        BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: PrimField<String>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {}
impl ToListMappable
    for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    type O = BlockAssignable<
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
    {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    type O = BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
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
struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: Option<
        DynamicBlock<
            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: Option<
        DynamicBlock<
            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: Option<
        DynamicBlock<
            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: Option<
        DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>,
    >,
    telephony_transfer_call: Option<
        DynamicBlock<
            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: Option<DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_success: Option<
        Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_agent_handoff: Option<
        Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_text:
        Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_audio:
        Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephony_transfer_call: Option<
        Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElDynamic,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
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
                DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
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
                DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
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
                DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
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
                DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
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
            BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>,
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
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    type O = BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
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
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> ListRef<
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
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
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    type O =
        BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
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
struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElDynamic {
    conditional_cases:
        Option<DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    messages: Option<DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: Option<
        DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_generative_fallback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases:
        Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions:
        Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxFlowEventHandlersElTriggerFulfillmentElDynamic,
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    #[doc = "Set the field `enable_generative_fallback`.\nIf the flag is true, the agent will utilize LLM to generate a text response.\nIf LLM generation fails, the defined responses in the fulfillment will be respected.\nThis flag is only useful for fulfillments associated with no-match event handlers."]
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
    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
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
                DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
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
impl ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentEl {}
impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
            enable_generative_fallback: core::default::Default::default(),
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_generative_fallback` after provisioning.\nIf the flag is true, the agent will utilize LLM to generate a text response.\nIf LLM generation fails, the defined responses in the fulfillment will be respected.\nThis flag is only useful for fulfillments associated with no-match event handlers."]
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
    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxFlowEventHandlersElDynamic {
    trigger_fulfillment: Option<DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment: Option<Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxFlowEventHandlersElDynamic,
}
impl DialogflowCxFlowEventHandlersEl {
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
        v: impl Into<BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
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
impl ToListMappable for DialogflowCxFlowEventHandlersEl {
    type O = BlockAssignable<DialogflowCxFlowEventHandlersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowEventHandlersEl {}
impl BuildDialogflowCxFlowEventHandlersEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersEl {
        DialogflowCxFlowEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxFlowEventHandlersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowEventHandlersElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowEventHandlersElRef {
        DialogflowCxFlowEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowEventHandlersElRef {
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
    ) -> ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_processing_mode: Option<PrimField<String>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    type O = BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
    pub fn build(self) -> DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef {
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
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
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl { enabled : core :: default :: Default :: default () , endpointing_timeout_duration : core :: default :: Default :: default () , finish_digit : core :: default :: Default :: default () , interdigit_timeout_duration : core :: default :: Default :: default () , max_digits : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequtectency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will de the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."] pub fn enabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enabled" , self . base)) } # [doc = "Get a reference to the value of field `endpointing_timeout_duration` after provisioning.\nEndpoint timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn endpointing_timeout_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.endpointing_timeout_duration" , self . base)) } # [doc = "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."] pub fn finish_digit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.finish_digit" , self . base)) } # [doc = "Get a reference to the value of field `interdigit_timeout_duration` after provisioning.\nInterdigit timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn interdigit_timeout_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interdigit_timeout_duration" , self . base)) } # [doc = "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."] pub fn max_digits (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_digits" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_consent_based_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_interaction_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { # [doc = "Set the field `enable_consent_based_redaction`.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."] pub fn set_enable_consent_based_redaction (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_consent_based_redaction = Some (v . into ()) ; self } # [doc = "Set the field `enable_interaction_logging`.\nEnables DF Interaction logging."] pub fn set_enable_interaction_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_interaction_logging = Some (v . into ()) ; self } # [doc = "Set the field `enable_stackdriver_logging`.\nEnables Google Cloud Logging."] pub fn set_enable_stackdriver_logging (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_stackdriver_logging = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl { enable_consent_based_redaction : core :: default :: Default :: default () , enable_interaction_logging : core :: default :: Default :: default () , enable_stackdriver_logging : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_consent_based_redaction` after provisioning.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."] pub fn enable_consent_based_redaction (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_consent_based_redaction" , self . base)) } # [doc = "Get a reference to the value of field `enable_interaction_logging` after provisioning.\nEnables DF Interaction logging."] pub fn enable_interaction_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_interaction_logging" , self . base)) } # [doc = "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nEnables Google Cloud Logging."] pub fn enable_stackdriver_logging (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_stackdriver_logging" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl
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
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { # [doc = "Set the field `endpointer_sensitivity`.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."] pub fn set_endpointer_sensitivity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . endpointer_sensitivity = Some (v . into ()) ; self } # [doc = "Set the field `models`.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."] pub fn set_models (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . models = Some (v . into ()) ; self } # [doc = "Set the field `no_speech_timeout`.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn set_no_speech_timeout (mut self , v : impl Into < PrimField < String > >) -> Self { self . no_speech_timeout = Some (v . into ()) ; self } # [doc = "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."] pub fn set_use_timeout_based_endpointing (mut self , v : impl Into < PrimField < bool > >) -> Self { self . use_timeout_based_endpointing = Some (v . into ()) ; self } }
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl { endpointer_sensitivity : core :: default :: Default :: default () , models : core :: default :: Default :: default () , no_speech_timeout : core :: default :: Default :: default () , use_timeout_based_endpointing : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `endpointer_sensitivity` after provisioning.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."] pub fn endpointer_sensitivity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.endpointer_sensitivity" , self . base)) } # [doc = "Get a reference to the value of field `models` after provisioning.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."] pub fn models (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.models" , self . base)) } # [doc = "Get a reference to the value of field `no_speech_timeout` after provisioning.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."] pub fn no_speech_timeout (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.no_speech_timeout" , self . base)) } # [doc = "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."] pub fn use_timeout_based_endpointing (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.use_timeout_based_endpointing" , self . base)) } }
#[derive(Serialize, Default)]
struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDynamic { dtmf_settings : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl >> , logging_settings : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl >> , speech_settings : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] dtmf_settings : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] logging_settings : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] speech_settings : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl > > , dynamic : DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDynamic , }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsEl >>,
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
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]    pub fn dtmf_settings (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElDtmfSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dtmf_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]    pub fn logging_settings (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElLoggingSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]    pub fn speech_settings (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElSpeechSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.speech_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl {}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{}
impl
    BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
{
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionEl {}
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_playback_interruption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl { allow_playback_interruption : core :: default :: Default :: default () , audio : core :: default :: Default :: default () , uri : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\n"] pub fn allow_playback_interruption (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_playback_interruption" , self . base)) } # [doc = "Get a reference to the value of field `audio` after provisioning.\n"] pub fn audio (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl { # [serde (skip_serializing_if = "Option::is_none")] segments : Option < ListField < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > > , }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        mut self,
        v : impl Into < ListField < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsEl > >,
    ) -> Self {
        self.segments = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioEl {
            segments: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]    pub fn segments (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElSegmentsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."] pub fn metadata (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metadata" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl
{}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl {}
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl { } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl { metadata : core :: default :: Default :: default () , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl {
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
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{}
impl
    BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl { ssml : core :: default :: Default :: default () , text : core :: default :: Default :: default () , }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef { shared : shared , base : base . to_string () , } } }
impl
    DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl
{
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef {
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    phone_number: PrimField<String>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { type O = BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { pub fn build (self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { phone_number : self . phone_number , } } }
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn new (shared : StackShared , base : String) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { shared : shared , base : base . to_string () , } } }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."] pub fn phone_number (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.phone_number" , self . base)) } }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text response variants. If multiple variants are defined, only one text response variant is returned at runtime.\nrequired: true"]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef {
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
struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElDynamic { conversation_success : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl >> , knowledge_info_card : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl >> , live_agent_handoff : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >> , output_audio_text : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl >> , play_audio : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl >> , telephony_transfer_call : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >> , text : Option < DynamicBlock < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl >> , }
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl { # [serde (skip_serializing_if = "Option::is_none")] channel : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] payload : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conversation_success : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl > > , # [serde (skip_serializing_if = "Option::is_none")] knowledge_info_card : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] live_agent_handoff : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl > > , # [serde (skip_serializing_if = "Option::is_none")] output_audio_text : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl > > , # [serde (skip_serializing_if = "Option::is_none")] play_audio : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl > > , # [serde (skip_serializing_if = "Option::is_none")] telephony_transfer_call : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl > > , # [serde (skip_serializing_if = "Option::is_none")] text : Option < Vec < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl > > , dynamic : DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElDynamic , }
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioEl >>,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
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
                DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextEl,
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
    type O =
        BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl {
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `end_interaction` after provisioning.\nThis type has no fields.\nIndicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]    pub fn end_interaction (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElEndInteractionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.end_interaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mixed_audio` after provisioning.\nRepresents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via playAudio. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn mixed_audio(
        &self,
    ) -> ListRef<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElMixedAudioElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.mixed_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\nReturns a response containing a custom, platform-specific payload.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]    pub fn conversation_success (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElConversationSuccessElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_success", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_info_card` after provisioning.\n"]    pub fn knowledge_info_card (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElKnowledgeInfoCardElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_info_card", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]    pub fn live_agent_handoff (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]    pub fn output_audio_text (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElOutputAudioTextElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElPlayAudioElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]    pub fn telephony_transfer_call (& self) -> ListRef < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElTextElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
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
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl
{
    type O = BlockAssignable<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl
{}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef
    {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef {
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
struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElDynamic {
    advanced_settings: Option<
        DynamicBlock<
            DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
        >,
    >,
    conditional_cases: Option<
        DynamicBlock<
            DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
        >,
    >,
    messages: Option<
        DynamicBlock<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>,
    >,
    set_parameter_actions: Option<
        DynamicBlock<
            DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
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
        Vec<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases: Option<
        Vec<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages:
        Option<Vec<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions: Option<
        Vec<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl>,
    >,
    dynamic: DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElDynamic,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
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
                DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsEl,
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
                DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesEl,
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
                DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesEl,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsEl >>,
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl {
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
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef {
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
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElAdvancedSettingsElRef,
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
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElConditionalCasesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElMessagesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<
        DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElSetParameterActionsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxFlowKnowledgeConnectorSettingsElDynamic {
    data_store_connections:
        Option<DynamicBlock<DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl>>,
    trigger_fulfillment:
        Option<DynamicBlock<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowKnowledgeConnectorSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_connections:
        Option<Vec<DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment:
        Option<Vec<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxFlowKnowledgeConnectorSettingsElDynamic,
}
impl DialogflowCxFlowKnowledgeConnectorSettingsEl {
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
            BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsEl>,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentEl>>,
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
impl ToListMappable for DialogflowCxFlowKnowledgeConnectorSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowKnowledgeConnectorSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowKnowledgeConnectorSettingsEl {}
impl BuildDialogflowCxFlowKnowledgeConnectorSettingsEl {
    pub fn build(self) -> DialogflowCxFlowKnowledgeConnectorSettingsEl {
        DialogflowCxFlowKnowledgeConnectorSettingsEl {
            enabled: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            data_store_connections: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxFlowKnowledgeConnectorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowKnowledgeConnectorSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowKnowledgeConnectorSettingsElRef {
        DialogflowCxFlowKnowledgeConnectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowKnowledgeConnectorSettingsElRef {
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
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElDataStoreConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> ListRef<DialogflowCxFlowKnowledgeConnectorSettingsElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowNluSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_training_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_type: Option<PrimField<String>>,
}
impl DialogflowCxFlowNluSettingsEl {
    #[doc = "Set the field `classification_threshold`.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold.\nIf the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
    pub fn set_classification_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.classification_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `model_training_mode`.\nIndicates NLU model training mode.\n* MODEL_TRAINING_MODE_AUTOMATIC: NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.\n* MODEL_TRAINING_MODE_MANUAL: User needs to manually trigger NLU model training. Best for large flows whose models take long time to train. Possible values: [\"MODEL_TRAINING_MODE_AUTOMATIC\", \"MODEL_TRAINING_MODE_MANUAL\"]"]
    pub fn set_model_training_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_training_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `model_type`.\nIndicates the type of NLU model.\n* MODEL_TYPE_STANDARD: Use standard NLU model.\n* MODEL_TYPE_ADVANCED: Use advanced NLU model. Possible values: [\"MODEL_TYPE_STANDARD\", \"MODEL_TYPE_ADVANCED\"]"]
    pub fn set_model_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_type = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowNluSettingsEl {
    type O = BlockAssignable<DialogflowCxFlowNluSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowNluSettingsEl {}
impl BuildDialogflowCxFlowNluSettingsEl {
    pub fn build(self) -> DialogflowCxFlowNluSettingsEl {
        DialogflowCxFlowNluSettingsEl {
            classification_threshold: core::default::Default::default(),
            model_training_mode: core::default::Default::default(),
            model_type: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowNluSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowNluSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowNluSettingsElRef {
        DialogflowCxFlowNluSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowNluSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `classification_threshold` after provisioning.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold.\nIf the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
    pub fn classification_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.classification_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_training_mode` after provisioning.\nIndicates NLU model training mode.\n* MODEL_TRAINING_MODE_AUTOMATIC: NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.\n* MODEL_TRAINING_MODE_MANUAL: User needs to manually trigger NLU model training. Best for large flows whose models take long time to train. Possible values: [\"MODEL_TRAINING_MODE_AUTOMATIC\", \"MODEL_TRAINING_MODE_MANUAL\"]"]
    pub fn model_training_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_training_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_type` after provisioning.\nIndicates the type of NLU model.\n* MODEL_TYPE_STANDARD: Use standard NLU model.\n* MODEL_TYPE_ADVANCED: Use advanced NLU model. Possible values: [\"MODEL_TYPE_STANDARD\", \"MODEL_TYPE_ADVANCED\"]"]
    pub fn model_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxFlowTimeoutsEl {
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
impl ToListMappable for DialogflowCxFlowTimeoutsEl {
    type O = BlockAssignable<DialogflowCxFlowTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTimeoutsEl {}
impl BuildDialogflowCxFlowTimeoutsEl {
    pub fn build(self) -> DialogflowCxFlowTimeoutsEl {
        DialogflowCxFlowTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTimeoutsElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cases: Option<PrimField<String>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[doc = "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cases = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    type O =
        BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl
{}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef
    {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl
{}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
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
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: PrimField<String>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {}
impl ToListMappable
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc = "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: PrimField<String>,
}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: PrimField<String>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {}
impl ToListMappable
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
{
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: PrimField<String>,
}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl
    {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef
    {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ListField<PrimField<String>>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    type O =
        BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
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
struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: Option<
        DynamicBlock<
            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: Option<
        DynamicBlock<
            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: Option<
        DynamicBlock<
            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: Option<
        DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>,
    >,
    telephony_transfer_call: Option<
        DynamicBlock<
            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: Option<
        DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_success: Option<
        Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_agent_handoff: Option<
        Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_text: Option<
        Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_audio:
        Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephony_transfer_call: Option<
        Vec<
            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElDynamic,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
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
        v : impl Into < BlockAssignable < DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl >>,
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
                DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
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
                DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
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
                DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl,
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
        v : impl Into < BlockAssignable < DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl >>,
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
            BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
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
impl ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    type O = BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
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
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef,
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
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.live_agent_handoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_audio_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }
    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> ListRef<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telephony_transfer_call", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
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
    for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl
{
    type O = BlockAssignable<
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
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
struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElDynamic {
    conditional_cases: Option<
        DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>,
    >,
    messages:
        Option<DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: Option<
        DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    return_partial_responses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_cases:
        Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_parameter_actions:
        Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElDynamic,
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
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
                DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl,
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
        v: impl Into<BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
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
                DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl,
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
impl ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    type O = BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {}
impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
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
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditional_cases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_parameter_actions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxFlowTransitionRoutesElDynamic {
    trigger_fulfillment:
        Option<DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_flow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_page: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_fulfillment: Option<Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElDynamic,
}
impl DialogflowCxFlowTransitionRoutesEl {
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
        v: impl Into<BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
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
impl ToListMappable for DialogflowCxFlowTransitionRoutesEl {
    type O = BlockAssignable<DialogflowCxFlowTransitionRoutesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxFlowTransitionRoutesEl {}
impl BuildDialogflowCxFlowTransitionRoutesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesEl {
        DialogflowCxFlowTransitionRoutesEl {
            condition: core::default::Default::default(),
            intent: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxFlowTransitionRoutesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxFlowTransitionRoutesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxFlowTransitionRoutesElRef {
        DialogflowCxFlowTransitionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxFlowTransitionRoutesElRef {
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
    ) -> ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_fulfillment", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxFlowDynamic {
    advanced_settings: Option<DynamicBlock<DialogflowCxFlowAdvancedSettingsEl>>,
    event_handlers: Option<DynamicBlock<DialogflowCxFlowEventHandlersEl>>,
    knowledge_connector_settings:
        Option<DynamicBlock<DialogflowCxFlowKnowledgeConnectorSettingsEl>>,
    nlu_settings: Option<DynamicBlock<DialogflowCxFlowNluSettingsEl>>,
    transition_routes: Option<DynamicBlock<DialogflowCxFlowTransitionRoutesEl>>,
}
