use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockagentAgentCollaboratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    agent_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version: Option<PrimField<String>>,
    collaboration_instruction: PrimField<String>,
    collaborator_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relay_conversation_history: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_descriptor: Option<Vec<BedrockagentAgentCollaboratorAgentDescriptorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentAgentCollaboratorTimeoutsEl>,
    dynamic: BedrockagentAgentCollaboratorDynamic,
}
struct BedrockagentAgentCollaborator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentAgentCollaboratorData>,
}
#[derive(Clone)]
pub struct BedrockagentAgentCollaborator(Rc<BedrockagentAgentCollaborator_>);
impl BedrockagentAgentCollaborator {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_version = Some(v.into());
        self
    }
    #[doc = "Set the field `prepare_agent`.\n"]
    pub fn set_prepare_agent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prepare_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `relay_conversation_history`.\n"]
    pub fn set_relay_conversation_history(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().relay_conversation_history = Some(v.into());
        self
    }
    #[doc = "Set the field `agent_descriptor`.\n"]
    pub fn set_agent_descriptor(
        self,
        v: impl Into<BlockAssignable<BedrockagentAgentCollaboratorAgentDescriptorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_descriptor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_descriptor = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentAgentCollaboratorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaboration_instruction` after provisioning.\n"]
    pub fn collaboration_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaborator_id` after provisioning.\n"]
    pub fn collaborator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaborator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaborator_name` after provisioning.\n"]
    pub fn collaborator_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaborator_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepare_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `relay_conversation_history` after provisioning.\n"]
    pub fn relay_conversation_history(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.relay_conversation_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_descriptor` after provisioning.\n"]
    pub fn agent_descriptor(&self) -> ListRef<BedrockagentAgentCollaboratorAgentDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentCollaboratorTimeoutsElRef {
        BedrockagentAgentCollaboratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BedrockagentAgentCollaborator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockagentAgentCollaborator {}
impl ToListMappable for BedrockagentAgentCollaborator {
    type O = ListRef<BedrockagentAgentCollaboratorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockagentAgentCollaborator_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_agent_collaborator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockagentAgentCollaborator {
    pub tf_id: String,
    #[doc = ""]
    pub agent_id: PrimField<String>,
    #[doc = ""]
    pub collaboration_instruction: PrimField<String>,
    #[doc = ""]
    pub collaborator_name: PrimField<String>,
}
impl BuildBedrockagentAgentCollaborator {
    pub fn build(self, stack: &mut Stack) -> BedrockagentAgentCollaborator {
        let out = BedrockagentAgentCollaborator(Rc::new(BedrockagentAgentCollaborator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentAgentCollaboratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_id: self.agent_id,
                agent_version: core::default::Default::default(),
                collaboration_instruction: self.collaboration_instruction,
                collaborator_name: self.collaborator_name,
                prepare_agent: core::default::Default::default(),
                region: core::default::Default::default(),
                relay_conversation_history: core::default::Default::default(),
                agent_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockagentAgentCollaboratorRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentCollaboratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockagentAgentCollaboratorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaboration_instruction` after provisioning.\n"]
    pub fn collaboration_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaborator_id` after provisioning.\n"]
    pub fn collaborator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaborator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collaborator_name` after provisioning.\n"]
    pub fn collaborator_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaborator_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prepare_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `relay_conversation_history` after provisioning.\n"]
    pub fn relay_conversation_history(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.relay_conversation_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `agent_descriptor` after provisioning.\n"]
    pub fn agent_descriptor(&self) -> ListRef<BedrockagentAgentCollaboratorAgentDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentCollaboratorTimeoutsElRef {
        BedrockagentAgentCollaboratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentCollaboratorAgentDescriptorEl {
    alias_arn: PrimField<String>,
}
impl BedrockagentAgentCollaboratorAgentDescriptorEl {}
impl ToListMappable for BedrockagentAgentCollaboratorAgentDescriptorEl {
    type O = BlockAssignable<BedrockagentAgentCollaboratorAgentDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentCollaboratorAgentDescriptorEl {
    #[doc = ""]
    pub alias_arn: PrimField<String>,
}
impl BuildBedrockagentAgentCollaboratorAgentDescriptorEl {
    pub fn build(self) -> BedrockagentAgentCollaboratorAgentDescriptorEl {
        BedrockagentAgentCollaboratorAgentDescriptorEl {
            alias_arn: self.alias_arn,
        }
    }
}
pub struct BedrockagentAgentCollaboratorAgentDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentCollaboratorAgentDescriptorElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentCollaboratorAgentDescriptorElRef {
        BedrockagentAgentCollaboratorAgentDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentCollaboratorAgentDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias_arn` after provisioning.\n"]
    pub fn alias_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentAgentCollaboratorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BedrockagentAgentCollaboratorTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentAgentCollaboratorTimeoutsEl {
    type O = BlockAssignable<BedrockagentAgentCollaboratorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentAgentCollaboratorTimeoutsEl {}
impl BuildBedrockagentAgentCollaboratorTimeoutsEl {
    pub fn build(self) -> BedrockagentAgentCollaboratorTimeoutsEl {
        BedrockagentAgentCollaboratorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentAgentCollaboratorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentAgentCollaboratorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentCollaboratorTimeoutsElRef {
        BedrockagentAgentCollaboratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentAgentCollaboratorTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentAgentCollaboratorDynamic {
    agent_descriptor: Option<DynamicBlock<BedrockagentAgentCollaboratorAgentDescriptorEl>>,
}
