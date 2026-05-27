use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxPlaybookData {
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
    goal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    playbook_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_tools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instruction: Option<Vec<DialogflowCxPlaybookInstructionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_model_settings: Option<Vec<DialogflowCxPlaybookLlmModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxPlaybookTimeoutsEl>,
    dynamic: DialogflowCxPlaybookDynamic,
}
struct DialogflowCxPlaybook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxPlaybookData>,
}
#[derive(Clone)]
pub struct DialogflowCxPlaybook(Rc<DialogflowCxPlaybook_>);
impl DialogflowCxPlaybook {
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
    #[doc = "Set the field `parent`.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `playbook_type`.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn set_playbook_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().playbook_type = Some(v.into());
        self
    }
    #[doc = "Set the field `referenced_tools`.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn set_referenced_tools(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().referenced_tools = Some(v.into());
        self
    }
    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPlaybookInstructionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instruction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instruction = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxPlaybookLlmModelSettingsEl>>,
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
    pub fn set_timeouts(self, v: impl Into<DialogflowCxPlaybookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of initial playbook creation.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the playbook, unique within an agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `goal` after provisioning.\nHigh level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub fn goal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.goal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Playbook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/playbooks/<Playbook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `playbook_type` after provisioning.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn playbook_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.playbook_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_flows` after provisioning.\nThe resource name of flows referenced by the current playbook in the instructions."]
    pub fn referenced_flows(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_flows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_playbooks` after provisioning.\nThe resource name of other playbooks referenced by the current playbook in the instructions."]
    pub fn referenced_playbooks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_playbooks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_tools` after provisioning.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn referenced_tools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_tools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token_count` after provisioning.\nEstimated number of tokes current playbook takes when sent to the LLM."]
    pub fn token_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast time the playbook version was updated.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> ListRef<DialogflowCxPlaybookInstructionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> ListRef<DialogflowCxPlaybookLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPlaybookTimeoutsElRef {
        DialogflowCxPlaybookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxPlaybook {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxPlaybook {}
impl ToListMappable for DialogflowCxPlaybook {
    type O = ListRef<DialogflowCxPlaybookRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxPlaybook_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_playbook".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxPlaybook {
    pub tf_id: String,
    #[doc = "The human-readable name of the playbook, unique within an agent."]
    pub display_name: PrimField<String>,
    #[doc = "High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub goal: PrimField<String>,
}
impl BuildDialogflowCxPlaybook {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxPlaybook {
        let out = DialogflowCxPlaybook(Rc::new(DialogflowCxPlaybook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxPlaybookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                goal: self.goal,
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                playbook_type: core::default::Default::default(),
                referenced_tools: core::default::Default::default(),
                instruction: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxPlaybookRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPlaybookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxPlaybookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of initial playbook creation.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the playbook, unique within an agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `goal` after provisioning.\nHigh level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub fn goal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.goal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Playbook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/playbooks/<Playbook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `playbook_type` after provisioning.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn playbook_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.playbook_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_flows` after provisioning.\nThe resource name of flows referenced by the current playbook in the instructions."]
    pub fn referenced_flows(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_flows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_playbooks` after provisioning.\nThe resource name of other playbooks referenced by the current playbook in the instructions."]
    pub fn referenced_playbooks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_playbooks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `referenced_tools` after provisioning.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn referenced_tools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_tools", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token_count` after provisioning.\nEstimated number of tokes current playbook takes when sent to the LLM."]
    pub fn token_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast time the playbook version was updated.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> ListRef<DialogflowCxPlaybookInstructionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> ListRef<DialogflowCxPlaybookLlmModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPlaybookTimeoutsElRef {
        DialogflowCxPlaybookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPlaybookInstructionElStepsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DialogflowCxPlaybookInstructionElStepsEl {
    #[doc = "Set the field `steps`.\nSub-processing needed to execute the current step.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Step](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.playbooks#step)."]
    pub fn set_steps(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.steps = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nStep instruction in text format."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxPlaybookInstructionElStepsEl {
    type O = BlockAssignable<DialogflowCxPlaybookInstructionElStepsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPlaybookInstructionElStepsEl {}
impl BuildDialogflowCxPlaybookInstructionElStepsEl {
    pub fn build(self) -> DialogflowCxPlaybookInstructionElStepsEl {
        DialogflowCxPlaybookInstructionElStepsEl {
            steps: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPlaybookInstructionElStepsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPlaybookInstructionElStepsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPlaybookInstructionElStepsElRef {
        DialogflowCxPlaybookInstructionElStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPlaybookInstructionElStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `steps` after provisioning.\nSub-processing needed to execute the current step.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Step](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.playbooks#step)."]
    pub fn steps(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.steps", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nStep instruction in text format."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxPlaybookInstructionElDynamic {
    steps: Option<DynamicBlock<DialogflowCxPlaybookInstructionElStepsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxPlaybookInstructionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guidelines: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<Vec<DialogflowCxPlaybookInstructionElStepsEl>>,
    dynamic: DialogflowCxPlaybookInstructionElDynamic,
}
impl DialogflowCxPlaybookInstructionEl {
    #[doc = "Set the field `guidelines`.\nGeneral guidelines for the playbook. These are unstructured instructions that are not directly part of the goal, e.g. \"Always be polite\". It's valid for this text to be long and used instead of steps altogether."]
    pub fn set_guidelines(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guidelines = Some(v.into());
        self
    }
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxPlaybookInstructionElStepsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.steps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxPlaybookInstructionEl {
    type O = BlockAssignable<DialogflowCxPlaybookInstructionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPlaybookInstructionEl {}
impl BuildDialogflowCxPlaybookInstructionEl {
    pub fn build(self) -> DialogflowCxPlaybookInstructionEl {
        DialogflowCxPlaybookInstructionEl {
            guidelines: core::default::Default::default(),
            steps: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxPlaybookInstructionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPlaybookInstructionElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPlaybookInstructionElRef {
        DialogflowCxPlaybookInstructionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPlaybookInstructionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guidelines` after provisioning.\nGeneral guidelines for the playbook. These are unstructured instructions that are not directly part of the goal, e.g. \"Always be polite\". It's valid for this text to be long and used instead of steps altogether."]
    pub fn guidelines(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guidelines", self.base))
    }
    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> ListRef<DialogflowCxPlaybookInstructionElStepsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxPlaybookLlmModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_text: Option<PrimField<String>>,
}
impl DialogflowCxPlaybookLlmModelSettingsEl {
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
impl ToListMappable for DialogflowCxPlaybookLlmModelSettingsEl {
    type O = BlockAssignable<DialogflowCxPlaybookLlmModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPlaybookLlmModelSettingsEl {}
impl BuildDialogflowCxPlaybookLlmModelSettingsEl {
    pub fn build(self) -> DialogflowCxPlaybookLlmModelSettingsEl {
        DialogflowCxPlaybookLlmModelSettingsEl {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPlaybookLlmModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPlaybookLlmModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPlaybookLlmModelSettingsElRef {
        DialogflowCxPlaybookLlmModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPlaybookLlmModelSettingsElRef {
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
pub struct DialogflowCxPlaybookTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxPlaybookTimeoutsEl {
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
impl ToListMappable for DialogflowCxPlaybookTimeoutsEl {
    type O = BlockAssignable<DialogflowCxPlaybookTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxPlaybookTimeoutsEl {}
impl BuildDialogflowCxPlaybookTimeoutsEl {
    pub fn build(self) -> DialogflowCxPlaybookTimeoutsEl {
        DialogflowCxPlaybookTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxPlaybookTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxPlaybookTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxPlaybookTimeoutsElRef {
        DialogflowCxPlaybookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxPlaybookTimeoutsElRef {
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
struct DialogflowCxPlaybookDynamic {
    instruction: Option<DynamicBlock<DialogflowCxPlaybookInstructionEl>>,
    llm_model_settings: Option<DynamicBlock<DialogflowCxPlaybookLlmModelSettingsEl>>,
}
