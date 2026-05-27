use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesAgentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<PrimField<String>>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_agents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrails: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instruction: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_agent_callbacks: Option<Vec<CesAgentAfterAgentCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_model_callbacks: Option<Vec<CesAgentAfterModelCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_tool_callbacks: Option<Vec<CesAgentAfterToolCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_agent_callbacks: Option<Vec<CesAgentBeforeAgentCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_model_callbacks: Option<Vec<CesAgentBeforeModelCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_tool_callbacks: Option<Vec<CesAgentBeforeToolCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_agent: Option<Vec<CesAgentLlmAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<Vec<CesAgentModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_dialogflow_agent: Option<Vec<CesAgentRemoteDialogflowAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesAgentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolsets: Option<Vec<CesAgentToolsetsEl>>,
    dynamic: CesAgentDynamic,
}
struct CesAgent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesAgentData>,
}
#[derive(Clone)]
pub struct CesAgent(Rc<CesAgent_>);
impl CesAgent {
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
    #[doc = "Set the field `agent_id`.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn set_agent_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_id = Some(v.into());
        self
    }
    #[doc = "Set the field `child_agents`.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_child_agents(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().child_agents = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nHuman-readable description of the agent."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrails`.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn set_guardrails(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().guardrails = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instruction`.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn set_instruction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `tools`.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tools(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tools = Some(v.into());
        self
    }
    #[doc = "Set the field `after_agent_callbacks`.\n"]
    pub fn set_after_agent_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentAfterAgentCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_agent_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_agent_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `after_model_callbacks`.\n"]
    pub fn set_after_model_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentAfterModelCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_model_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_model_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `after_tool_callbacks`.\n"]
    pub fn set_after_tool_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentAfterToolCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_tool_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_tool_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `before_agent_callbacks`.\n"]
    pub fn set_before_agent_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentBeforeAgentCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_agent_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_agent_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `before_model_callbacks`.\n"]
    pub fn set_before_model_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentBeforeModelCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_model_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_model_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `before_tool_callbacks`.\n"]
    pub fn set_before_tool_callbacks(
        self,
        v: impl Into<BlockAssignable<CesAgentBeforeToolCallbacksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_tool_callbacks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_tool_callbacks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `llm_agent`.\n"]
    pub fn set_llm_agent(self, v: impl Into<BlockAssignable<CesAgentLlmAgentEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_agent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_agent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        self,
        v: impl Into<BlockAssignable<CesAgentModelSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote_dialogflow_agent`.\n"]
    pub fn set_remote_dialogflow_agent(
        self,
        v: impl Into<BlockAssignable<CesAgentRemoteDialogflowAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_dialogflow_agent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_dialogflow_agent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesAgentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(self, v: impl Into<BlockAssignable<CesAgentToolsetsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().toolsets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.toolsets = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn child_agents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.child_agents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the agent was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the agent."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\nIf the agent is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
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
    #[doc = "Get a reference to the value of field `tools` after provisioning.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the agent was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(&self) -> ListRef<CesAgentAfterAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_agent_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(&self) -> ListRef<CesAgentAfterModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_model_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(&self) -> ListRef<CesAgentAfterToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_tool_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(&self) -> ListRef<CesAgentBeforeAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_agent_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(&self) -> ListRef<CesAgentBeforeModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_model_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(&self) -> ListRef<CesAgentBeforeToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_tool_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> ListRef<CesAgentLlmAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAgentModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(&self) -> ListRef<CesAgentRemoteDialogflowAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_dialogflow_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAgentTimeoutsElRef {
        CesAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> ListRef<CesAgentToolsetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.toolsets", self.extract_ref()),
        )
    }
}
impl Referable for CesAgent {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesAgent {}
impl ToListMappable for CesAgent {
    type O = ListRef<CesAgentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesAgent_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_agent".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesAgent {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "Display name of the agent."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesAgent {
    pub fn build(self, stack: &mut Stack) -> CesAgent {
        let out = CesAgent(Rc::new(CesAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesAgentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_id: core::default::Default::default(),
                app: self.app,
                child_agents: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                guardrails: core::default::Default::default(),
                id: core::default::Default::default(),
                instruction: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                tools: core::default::Default::default(),
                after_agent_callbacks: core::default::Default::default(),
                after_model_callbacks: core::default::Default::default(),
                after_tool_callbacks: core::default::Default::default(),
                before_agent_callbacks: core::default::Default::default(),
                before_model_callbacks: core::default::Default::default(),
                before_tool_callbacks: core::default::Default::default(),
                llm_agent: core::default::Default::default(),
                model_settings: core::default::Default::default(),
                remote_dialogflow_agent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                toolsets: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesAgentRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesAgentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn child_agents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.child_agents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the agent was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the agent."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\nIf the agent is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
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
    #[doc = "Get a reference to the value of field `tools` after provisioning.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the agent was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(&self) -> ListRef<CesAgentAfterAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_agent_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(&self) -> ListRef<CesAgentAfterModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_model_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(&self) -> ListRef<CesAgentAfterToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_tool_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(&self) -> ListRef<CesAgentBeforeAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_agent_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(&self) -> ListRef<CesAgentBeforeModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_model_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(&self) -> ListRef<CesAgentBeforeToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_tool_callbacks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> ListRef<CesAgentLlmAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAgentModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(&self) -> ListRef<CesAgentRemoteDialogflowAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_dialogflow_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAgentTimeoutsElRef {
        CesAgentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> ListRef<CesAgentToolsetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.toolsets", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesAgentAfterAgentCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentAfterAgentCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentAfterAgentCallbacksEl {
    type O = BlockAssignable<CesAgentAfterAgentCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentAfterAgentCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentAfterAgentCallbacksEl {
    pub fn build(self) -> CesAgentAfterAgentCallbacksEl {
        CesAgentAfterAgentCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentAfterAgentCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentAfterAgentCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentAfterAgentCallbacksElRef {
        CesAgentAfterAgentCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentAfterAgentCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentAfterModelCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentAfterModelCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentAfterModelCallbacksEl {
    type O = BlockAssignable<CesAgentAfterModelCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentAfterModelCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentAfterModelCallbacksEl {
    pub fn build(self) -> CesAgentAfterModelCallbacksEl {
        CesAgentAfterModelCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentAfterModelCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentAfterModelCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentAfterModelCallbacksElRef {
        CesAgentAfterModelCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentAfterModelCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentAfterToolCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentAfterToolCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentAfterToolCallbacksEl {
    type O = BlockAssignable<CesAgentAfterToolCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentAfterToolCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentAfterToolCallbacksEl {
    pub fn build(self) -> CesAgentAfterToolCallbacksEl {
        CesAgentAfterToolCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentAfterToolCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentAfterToolCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentAfterToolCallbacksElRef {
        CesAgentAfterToolCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentAfterToolCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentBeforeAgentCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentBeforeAgentCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentBeforeAgentCallbacksEl {
    type O = BlockAssignable<CesAgentBeforeAgentCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentBeforeAgentCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentBeforeAgentCallbacksEl {
    pub fn build(self) -> CesAgentBeforeAgentCallbacksEl {
        CesAgentBeforeAgentCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentBeforeAgentCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentBeforeAgentCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentBeforeAgentCallbacksElRef {
        CesAgentBeforeAgentCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentBeforeAgentCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentBeforeModelCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentBeforeModelCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentBeforeModelCallbacksEl {
    type O = BlockAssignable<CesAgentBeforeModelCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentBeforeModelCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentBeforeModelCallbacksEl {
    pub fn build(self) -> CesAgentBeforeModelCallbacksEl {
        CesAgentBeforeModelCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentBeforeModelCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentBeforeModelCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentBeforeModelCallbacksElRef {
        CesAgentBeforeModelCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentBeforeModelCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentBeforeToolCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    python_code: PrimField<String>,
}
impl CesAgentBeforeToolCallbacksEl {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentBeforeToolCallbacksEl {
    type O = BlockAssignable<CesAgentBeforeToolCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentBeforeToolCallbacksEl {
    #[doc = "The python code to execute for the callback."]
    pub python_code: PrimField<String>,
}
impl BuildCesAgentBeforeToolCallbacksEl {
    pub fn build(self) -> CesAgentBeforeToolCallbacksEl {
        CesAgentBeforeToolCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}
pub struct CesAgentBeforeToolCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentBeforeToolCallbacksElRef {
    fn new(shared: StackShared, base: String) -> CesAgentBeforeToolCallbacksElRef {
        CesAgentBeforeToolCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentBeforeToolCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentLlmAgentEl {}
impl CesAgentLlmAgentEl {}
impl ToListMappable for CesAgentLlmAgentEl {
    type O = BlockAssignable<CesAgentLlmAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentLlmAgentEl {}
impl BuildCesAgentLlmAgentEl {
    pub fn build(self) -> CesAgentLlmAgentEl {
        CesAgentLlmAgentEl {}
    }
}
pub struct CesAgentLlmAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentLlmAgentElRef {
    fn new(shared: StackShared, base: String) -> CesAgentLlmAgentElRef {
        CesAgentLlmAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentLlmAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct CesAgentModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAgentModelSettingsEl {
    #[doc = "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentModelSettingsEl {
    type O = BlockAssignable<CesAgentModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentModelSettingsEl {}
impl BuildCesAgentModelSettingsEl {
    pub fn build(self) -> CesAgentModelSettingsEl {
        CesAgentModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAgentModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAgentModelSettingsElRef {
        CesAgentModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAgentRemoteDialogflowAgentEl {
    agent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_id: Option<PrimField<String>>,
    flow_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_variable_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_response_interruption_settings: Option<PrimField<bool>>,
}
impl CesAgentRemoteDialogflowAgentEl {
    #[doc = "Set the field `environment_id`.\nThe environment ID of the Dialogflow agent be used for the agent\nexecution. If not specified, the draft environment will be used."]
    pub fn set_environment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_id = Some(v.into());
        self
    }
    #[doc = "Set the field `input_variable_mapping`.\nThe mapping of the app variables names to the Dialogflow session\nparameters names to be sent to the Dialogflow agent as input."]
    pub fn set_input_variable_mapping(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.input_variable_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `output_variable_mapping`.\nThe mapping of the Dialogflow session parameters names to the app\nvariables names to be sent back to the CES agent after the Dialogflow\nagent execution ends."]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.output_variable_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `respect_response_interruption_settings`.\nIndicates whether to respect the message-level interruption settings configured in the Dialogflow agent. * If false: all response messages from the Dialogflow agent follow the app-level barge-in settings. * If true: only response messages with ['allow_playback_interruption'](https://docs.cloud.google.com/dialogflow/cx/docs/reference/rpc/google.cloud.dialogflow.cx.v3#text) set to true will be interruptable, all other messages follow the app-level barge-in settings."]
    pub fn set_respect_response_interruption_settings(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.respect_response_interruption_settings = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentRemoteDialogflowAgentEl {
    type O = BlockAssignable<CesAgentRemoteDialogflowAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentRemoteDialogflowAgentEl {
    #[doc = "The\n[Dialogflow](https://cloud.google.com/dialogflow/cx/docs/concept/console-conversational-agents\nagent resource name.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub agent: PrimField<String>,
    #[doc = "The flow ID of the flow in the Dialogflow agent."]
    pub flow_id: PrimField<String>,
}
impl BuildCesAgentRemoteDialogflowAgentEl {
    pub fn build(self) -> CesAgentRemoteDialogflowAgentEl {
        CesAgentRemoteDialogflowAgentEl {
            agent: self.agent,
            environment_id: core::default::Default::default(),
            flow_id: self.flow_id,
            input_variable_mapping: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
            respect_response_interruption_settings: core::default::Default::default(),
        }
    }
}
pub struct CesAgentRemoteDialogflowAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentRemoteDialogflowAgentElRef {
    fn new(shared: StackShared, base: String) -> CesAgentRemoteDialogflowAgentElRef {
        CesAgentRemoteDialogflowAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentRemoteDialogflowAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\nThe\n[Dialogflow](https://cloud.google.com/dialogflow/cx/docs/concept/console-conversational-agents\nagent resource name.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\nThe environment ID of the Dialogflow agent be used for the agent\nexecution. If not specified, the draft environment will be used."]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flow_id` after provisioning.\nThe flow ID of the flow in the Dialogflow agent."]
    pub fn flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_id", self.base))
    }
    #[doc = "Get a reference to the value of field `input_variable_mapping` after provisioning.\nThe mapping of the app variables names to the Dialogflow session\nparameters names to be sent to the Dialogflow agent as input."]
    pub fn input_variable_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.input_variable_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_variable_mapping` after provisioning.\nThe mapping of the Dialogflow session parameters names to the app\nvariables names to be sent back to the CES agent after the Dialogflow\nagent execution ends."]
    pub fn output_variable_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.output_variable_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `respect_response_interruption_settings` after provisioning.\nIndicates whether to respect the message-level interruption settings configured in the Dialogflow agent. * If false: all response messages from the Dialogflow agent follow the app-level barge-in settings. * If true: only response messages with ['allow_playback_interruption'](https://docs.cloud.google.com/dialogflow/cx/docs/reference/rpc/google.cloud.dialogflow.cx.v3#text) set to true will be interruptable, all other messages follow the app-level barge-in settings."]
    pub fn respect_response_interruption_settings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.respect_response_interruption_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAgentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesAgentTimeoutsEl {
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
impl ToListMappable for CesAgentTimeoutsEl {
    type O = BlockAssignable<CesAgentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentTimeoutsEl {}
impl BuildCesAgentTimeoutsEl {
    pub fn build(self) -> CesAgentTimeoutsEl {
        CesAgentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesAgentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesAgentTimeoutsElRef {
        CesAgentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentTimeoutsElRef {
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
pub struct CesAgentToolsetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_ids: Option<ListField<PrimField<String>>>,
    toolset: PrimField<String>,
}
impl CesAgentToolsetsEl {
    #[doc = "Set the field `tool_ids`.\nThe tools IDs to filter the toolset."]
    pub fn set_tool_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tool_ids = Some(v.into());
        self
    }
}
impl ToListMappable for CesAgentToolsetsEl {
    type O = BlockAssignable<CesAgentToolsetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAgentToolsetsEl {
    #[doc = "The resource name of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub toolset: PrimField<String>,
}
impl BuildCesAgentToolsetsEl {
    pub fn build(self) -> CesAgentToolsetsEl {
        CesAgentToolsetsEl {
            tool_ids: core::default::Default::default(),
            toolset: self.toolset,
        }
    }
}
pub struct CesAgentToolsetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAgentToolsetsElRef {
    fn new(shared: StackShared, base: String) -> CesAgentToolsetsElRef {
        CesAgentToolsetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAgentToolsetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_ids` after provisioning.\nThe tools IDs to filter the toolset."]
    pub fn tool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tool_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\nThe resource name of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesAgentDynamic {
    after_agent_callbacks: Option<DynamicBlock<CesAgentAfterAgentCallbacksEl>>,
    after_model_callbacks: Option<DynamicBlock<CesAgentAfterModelCallbacksEl>>,
    after_tool_callbacks: Option<DynamicBlock<CesAgentAfterToolCallbacksEl>>,
    before_agent_callbacks: Option<DynamicBlock<CesAgentBeforeAgentCallbacksEl>>,
    before_model_callbacks: Option<DynamicBlock<CesAgentBeforeModelCallbacksEl>>,
    before_tool_callbacks: Option<DynamicBlock<CesAgentBeforeToolCallbacksEl>>,
    llm_agent: Option<DynamicBlock<CesAgentLlmAgentEl>>,
    model_settings: Option<DynamicBlock<CesAgentModelSettingsEl>>,
    remote_dialogflow_agent: Option<DynamicBlock<CesAgentRemoteDialogflowAgentEl>>,
    toolsets: Option<DynamicBlock<CesAgentToolsetsEl>>,
}
