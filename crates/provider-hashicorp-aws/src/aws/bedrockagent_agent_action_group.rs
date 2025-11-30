use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BedrockagentAgentActionGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_group_state: Option<PrimField<String>>,
    agent_id: PrimField<String>,
    agent_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_action_group_signature: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_resource_in_use_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_group_executor: Option<Vec<BedrockagentAgentActionGroupActionGroupExecutorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_schema: Option<Vec<BedrockagentAgentActionGroupApiSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_schema: Option<Vec<BedrockagentAgentActionGroupFunctionSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentAgentActionGroupTimeoutsEl>,
    dynamic: BedrockagentAgentActionGroupDynamic,
}

struct BedrockagentAgentActionGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentAgentActionGroupData>,
}

#[derive(Clone)]
pub struct BedrockagentAgentActionGroup(Rc<BedrockagentAgentActionGroup_>);

impl BedrockagentAgentActionGroup {
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

    #[doc = "Set the field `action_group_state`.\n"]
    pub fn set_action_group_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action_group_state = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `parent_action_group_signature`.\n"]
    pub fn set_parent_action_group_signature(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_action_group_signature = Some(v.into());
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

    #[doc = "Set the field `skip_resource_in_use_check`.\n"]
    pub fn set_skip_resource_in_use_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_resource_in_use_check = Some(v.into());
        self
    }

    #[doc = "Set the field `action_group_executor`.\n"]
    pub fn set_action_group_executor(
        self,
        v: impl Into<BlockAssignable<BedrockagentAgentActionGroupActionGroupExecutorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_group_executor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_group_executor = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `api_schema`.\n"]
    pub fn set_api_schema(
        self,
        v: impl Into<BlockAssignable<BedrockagentAgentActionGroupApiSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_schema = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `function_schema`.\n"]
    pub fn set_function_schema(
        self,
        v: impl Into<BlockAssignable<BedrockagentAgentActionGroupFunctionSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().function_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.function_schema = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentAgentActionGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action_group_id` after provisioning.\n"]
    pub fn action_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_name` after provisioning.\n"]
    pub fn action_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_state` after provisioning.\n"]
    pub fn action_group_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_state", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_action_group_signature` after provisioning.\n"]
    pub fn parent_action_group_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_action_group_signature", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_resource_in_use_check", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_executor` after provisioning.\n"]
    pub fn action_group_executor(
        &self,
    ) -> ListRef<BedrockagentAgentActionGroupActionGroupExecutorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_group_executor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `api_schema` after provisioning.\n"]
    pub fn api_schema(&self) -> ListRef<BedrockagentAgentActionGroupApiSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_schema` after provisioning.\n"]
    pub fn function_schema(&self) -> ListRef<BedrockagentAgentActionGroupFunctionSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.function_schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentActionGroupTimeoutsElRef {
        BedrockagentAgentActionGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentAgentActionGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BedrockagentAgentActionGroup {}

impl ToListMappable for BedrockagentAgentActionGroup {
    type O = ListRef<BedrockagentAgentActionGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentAgentActionGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_agent_action_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentAgentActionGroup {
    pub tf_id: String,
    #[doc = ""]
    pub action_group_name: PrimField<String>,
    #[doc = ""]
    pub agent_id: PrimField<String>,
    #[doc = ""]
    pub agent_version: PrimField<String>,
}

impl BuildBedrockagentAgentActionGroup {
    pub fn build(self, stack: &mut Stack) -> BedrockagentAgentActionGroup {
        let out = BedrockagentAgentActionGroup(Rc::new(BedrockagentAgentActionGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentAgentActionGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action_group_name: self.action_group_name,
                action_group_state: core::default::Default::default(),
                agent_id: self.agent_id,
                agent_version: self.agent_version,
                description: core::default::Default::default(),
                parent_action_group_signature: core::default::Default::default(),
                prepare_agent: core::default::Default::default(),
                region: core::default::Default::default(),
                skip_resource_in_use_check: core::default::Default::default(),
                action_group_executor: core::default::Default::default(),
                api_schema: core::default::Default::default(),
                function_schema: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentAgentActionGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BedrockagentAgentActionGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_group_id` after provisioning.\n"]
    pub fn action_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_name` after provisioning.\n"]
    pub fn action_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_state` after provisioning.\n"]
    pub fn action_group_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_group_state", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_action_group_signature` after provisioning.\n"]
    pub fn parent_action_group_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_action_group_signature", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_resource_in_use_check", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_group_executor` after provisioning.\n"]
    pub fn action_group_executor(
        &self,
    ) -> ListRef<BedrockagentAgentActionGroupActionGroupExecutorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_group_executor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `api_schema` after provisioning.\n"]
    pub fn api_schema(&self) -> ListRef<BedrockagentAgentActionGroupApiSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_schema` after provisioning.\n"]
    pub fn function_schema(&self) -> ListRef<BedrockagentAgentActionGroupFunctionSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.function_schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentActionGroupTimeoutsElRef {
        BedrockagentAgentActionGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupActionGroupExecutorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<PrimField<String>>,
}

impl BedrockagentAgentActionGroupActionGroupExecutorEl {
    #[doc = "Set the field `custom_control`.\n"]
    pub fn set_custom_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_control = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupActionGroupExecutorEl {
    type O = BlockAssignable<BedrockagentAgentActionGroupActionGroupExecutorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupActionGroupExecutorEl {}

impl BuildBedrockagentAgentActionGroupActionGroupExecutorEl {
    pub fn build(self) -> BedrockagentAgentActionGroupActionGroupExecutorEl {
        BedrockagentAgentActionGroupActionGroupExecutorEl {
            custom_control: core::default::Default::default(),
            lambda: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupActionGroupExecutorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupActionGroupExecutorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentAgentActionGroupActionGroupExecutorElRef {
        BedrockagentAgentActionGroupActionGroupExecutorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupActionGroupExecutorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_control` after provisioning.\n"]
    pub fn custom_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_control", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupApiSchemaElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_object_key: Option<PrimField<String>>,
}

impl BedrockagentAgentActionGroupApiSchemaElS3El {
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_object_key`.\n"]
    pub fn set_s3_object_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_object_key = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupApiSchemaElS3El {
    type O = BlockAssignable<BedrockagentAgentActionGroupApiSchemaElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupApiSchemaElS3El {}

impl BuildBedrockagentAgentActionGroupApiSchemaElS3El {
    pub fn build(self) -> BedrockagentAgentActionGroupApiSchemaElS3El {
        BedrockagentAgentActionGroupApiSchemaElS3El {
            s3_bucket_name: core::default::Default::default(),
            s3_object_key: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupApiSchemaElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupApiSchemaElS3ElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentActionGroupApiSchemaElS3ElRef {
        BedrockagentAgentActionGroupApiSchemaElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupApiSchemaElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_object_key` after provisioning.\n"]
    pub fn s3_object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_key", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupApiSchemaElDynamic {
    s3: Option<DynamicBlock<BedrockagentAgentActionGroupApiSchemaElS3El>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupApiSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<BedrockagentAgentActionGroupApiSchemaElS3El>>,
    dynamic: BedrockagentAgentActionGroupApiSchemaElDynamic,
}

impl BedrockagentAgentActionGroupApiSchemaEl {
    #[doc = "Set the field `payload`.\n"]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentAgentActionGroupApiSchemaElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupApiSchemaEl {
    type O = BlockAssignable<BedrockagentAgentActionGroupApiSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupApiSchemaEl {}

impl BuildBedrockagentAgentActionGroupApiSchemaEl {
    pub fn build(self) -> BedrockagentAgentActionGroupApiSchemaEl {
        BedrockagentAgentActionGroupApiSchemaEl {
            payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupApiSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupApiSchemaElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentActionGroupApiSchemaElRef {
        BedrockagentAgentActionGroupApiSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupApiSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<BedrockagentAgentActionGroupApiSchemaElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    map_block_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }
}

impl ToListMappable
    for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl
{
    type O = BlockAssignable<
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl
{
    #[doc = ""]
    pub map_block_key: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl {
    pub fn build(
        self,
    ) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl {
            description: core::default::Default::default(),
            map_block_key: self.map_block_key,
            required: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersElRef
    {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_block_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElDynamic {
    parameters: Option<
        DynamicBlock<
            BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl>,
    >,
    dynamic: BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElDynamic,
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElParametersEl,
                        >,
                    >,
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

impl ToListMappable for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
    type O =
        BlockAssignable<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
    pub fn build(self) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl {
            description: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElDynamic {
    functions: Option<
        DynamicBlock<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    functions:
        Option<Vec<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl>>,
    dynamic: BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElDynamic,
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.functions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.functions = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
    type O = BlockAssignable<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {}

impl BuildBedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
    pub fn build(self) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl {
            functions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef {
        BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(
        &self,
    ) -> ListRef<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElFunctionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.functions", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupFunctionSchemaElDynamic {
    member_functions:
        Option<DynamicBlock<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupFunctionSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    member_functions: Option<Vec<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl>>,
    dynamic: BedrockagentAgentActionGroupFunctionSchemaElDynamic,
}

impl BedrockagentAgentActionGroupFunctionSchemaEl {
    #[doc = "Set the field `member_functions`.\n"]
    pub fn set_member_functions(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.member_functions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.member_functions = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupFunctionSchemaEl {
    type O = BlockAssignable<BedrockagentAgentActionGroupFunctionSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupFunctionSchemaEl {}

impl BuildBedrockagentAgentActionGroupFunctionSchemaEl {
    pub fn build(self) -> BedrockagentAgentActionGroupFunctionSchemaEl {
        BedrockagentAgentActionGroupFunctionSchemaEl {
            member_functions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupFunctionSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupFunctionSchemaElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentActionGroupFunctionSchemaElRef {
        BedrockagentAgentActionGroupFunctionSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupFunctionSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `member_functions` after provisioning.\n"]
    pub fn member_functions(
        &self,
    ) -> ListRef<BedrockagentAgentActionGroupFunctionSchemaElMemberFunctionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.member_functions", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockagentAgentActionGroupTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentAgentActionGroupTimeoutsEl {
    type O = BlockAssignable<BedrockagentAgentActionGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroupTimeoutsEl {}

impl BuildBedrockagentAgentActionGroupTimeoutsEl {
    pub fn build(self) -> BedrockagentAgentActionGroupTimeoutsEl {
        BedrockagentAgentActionGroupTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentAgentActionGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentActionGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentActionGroupTimeoutsElRef {
        BedrockagentAgentActionGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroupTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupDynamic {
    action_group_executor: Option<DynamicBlock<BedrockagentAgentActionGroupActionGroupExecutorEl>>,
    api_schema: Option<DynamicBlock<BedrockagentAgentActionGroupApiSchemaEl>>,
    function_schema: Option<DynamicBlock<BedrockagentAgentActionGroupFunctionSchemaEl>>,
}
