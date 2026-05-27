use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesExampleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_agent: Option<PrimField<String>>,
    example_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<CesExampleMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesExampleTimeoutsEl>,
    dynamic: CesExampleDynamic,
}
struct CesExample_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesExampleData>,
}
#[derive(Clone)]
pub struct CesExample(Rc<CesExample_>);
impl CesExample {
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
    #[doc = "Set the field `description`.\nHuman-readable description of the example."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_agent`.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_entry_agent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().entry_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(self, v: impl Into<BlockAssignable<CesExampleMessagesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().messages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.messages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesExampleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the example was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the example."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the example."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_agent` after provisioning.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn entry_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `example_id` after provisioning.\nThe ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub fn example_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.example_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `invalid` after provisioning.\nThe example may become invalid if referencing resources are deleted.\nInvalid examples will not be used as few-shot examples."]
    pub fn invalid(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invalid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the example.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/examples/{example}'"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the example was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> ListRef<CesExampleMessagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.messages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesExampleTimeoutsElRef {
        CesExampleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesExample {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesExample {}
impl ToListMappable for CesExample {
    type O = ListRef<CesExampleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesExample_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_example".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesExample {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "Display name of the example."]
    pub display_name: PrimField<String>,
    #[doc = "The ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub example_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesExample {
    pub fn build(self, stack: &mut Stack) -> CesExample {
        let out = CesExample(Rc::new(CesExample_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesExampleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                entry_agent: core::default::Default::default(),
                example_id: self.example_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                messages: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesExampleRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesExampleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the example was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the example."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the example."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_agent` after provisioning.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn entry_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `example_id` after provisioning.\nThe ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub fn example_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.example_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `invalid` after provisioning.\nThe example may become invalid if referencing resources are deleted.\nInvalid examples will not be used as few-shot examples."]
    pub fn invalid(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invalid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the example.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/examples/{example}'"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the example was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> ListRef<CesExampleMessagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.messages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesExampleTimeoutsElRef {
        CesExampleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElAgentTransferEl {
    target_agent: PrimField<String>,
}
impl CesExampleMessagesElChunksElAgentTransferEl {}
impl ToListMappable for CesExampleMessagesElChunksElAgentTransferEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElAgentTransferEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElAgentTransferEl {
    #[doc = "The agent to which the conversation is being transferred. The agent will\nhandle the conversation from this point forward.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub target_agent: PrimField<String>,
}
impl BuildCesExampleMessagesElChunksElAgentTransferEl {
    pub fn build(self) -> CesExampleMessagesElChunksElAgentTransferEl {
        CesExampleMessagesElChunksElAgentTransferEl {
            target_agent: self.target_agent,
        }
    }
}
pub struct CesExampleMessagesElChunksElAgentTransferElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElAgentTransferElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElChunksElAgentTransferElRef {
        CesExampleMessagesElChunksElAgentTransferElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElAgentTransferElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_agent` after provisioning.\nThe agent to which the conversation is being transferred. The agent will\nhandle the conversation from this point forward.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn target_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_agent", self.base))
    }
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElImageEl {
    data: PrimField<String>,
    mime_type: PrimField<String>,
}
impl CesExampleMessagesElChunksElImageEl {}
impl ToListMappable for CesExampleMessagesElChunksElImageEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElImageEl {
    #[doc = "Raw bytes of the image."]
    pub data: PrimField<String>,
    #[doc = "The IANA standard MIME type of the source data.\nSupported image types includes:\n* image/png\n* image/jpeg\n* image/webp"]
    pub mime_type: PrimField<String>,
}
impl BuildCesExampleMessagesElChunksElImageEl {
    pub fn build(self) -> CesExampleMessagesElChunksElImageEl {
        CesExampleMessagesElChunksElImageEl {
            data: self.data,
            mime_type: self.mime_type,
        }
    }
}
pub struct CesExampleMessagesElChunksElImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElImageElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElChunksElImageElRef {
        CesExampleMessagesElChunksElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\nRaw bytes of the image."]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\nThe IANA standard MIME type of the source data.\nSupported image types includes:\n* image/png\n* image/jpeg\n* image/webp"]
    pub fn mime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElToolCallElToolsetToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_id: Option<PrimField<String>>,
    toolset: PrimField<String>,
}
impl CesExampleMessagesElChunksElToolCallElToolsetToolEl {
    #[doc = "Set the field `tool_id`.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn set_tool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool_id = Some(v.into());
        self
    }
}
impl ToListMappable for CesExampleMessagesElChunksElToolCallElToolsetToolEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElToolCallElToolsetToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElToolCallElToolsetToolEl {
    #[doc = "The resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub toolset: PrimField<String>,
}
impl BuildCesExampleMessagesElChunksElToolCallElToolsetToolEl {
    pub fn build(self) -> CesExampleMessagesElChunksElToolCallElToolsetToolEl {
        CesExampleMessagesElChunksElToolCallElToolsetToolEl {
            tool_id: core::default::Default::default(),
            toolset: self.toolset,
        }
    }
}
pub struct CesExampleMessagesElChunksElToolCallElToolsetToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElToolCallElToolsetToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesExampleMessagesElChunksElToolCallElToolsetToolElRef {
        CesExampleMessagesElChunksElToolCallElToolsetToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElToolCallElToolsetToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\nThe resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesExampleMessagesElChunksElToolCallElDynamic {
    toolset_tool: Option<DynamicBlock<CesExampleMessagesElChunksElToolCallElToolsetToolEl>>,
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElToolCallEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset_tool: Option<Vec<CesExampleMessagesElChunksElToolCallElToolsetToolEl>>,
    dynamic: CesExampleMessagesElChunksElToolCallElDynamic,
}
impl CesExampleMessagesElChunksElToolCallEl {
    #[doc = "Set the field `args`.\nThe input parameters and values for the tool in JSON object format."]
    pub fn set_args(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\nThe unique identifier of the tool call. If populated, the client should\nreturn the execution result with the matching ID in\nToolResponse."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `tool`.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElToolCallElToolsetToolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.toolset_tool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.toolset_tool = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesExampleMessagesElChunksElToolCallEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElToolCallEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElToolCallEl {}
impl BuildCesExampleMessagesElChunksElToolCallEl {
    pub fn build(self) -> CesExampleMessagesElChunksElToolCallEl {
        CesExampleMessagesElChunksElToolCallEl {
            args: core::default::Default::default(),
            id: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesExampleMessagesElChunksElToolCallElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElToolCallElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElChunksElToolCallElRef {
        CesExampleMessagesElChunksElToolCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElToolCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe input parameters and values for the tool in JSON object format."]
    pub fn args(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the tool."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the tool call. If populated, the client should\nreturn the execution result with the matching ID in\nToolResponse."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(&self) -> ListRef<CesExampleMessagesElChunksElToolCallElToolsetToolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElToolResponseElToolsetToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_id: Option<PrimField<String>>,
    toolset: PrimField<String>,
}
impl CesExampleMessagesElChunksElToolResponseElToolsetToolEl {
    #[doc = "Set the field `tool_id`.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn set_tool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool_id = Some(v.into());
        self
    }
}
impl ToListMappable for CesExampleMessagesElChunksElToolResponseElToolsetToolEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElToolResponseElToolsetToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElToolResponseElToolsetToolEl {
    #[doc = "The resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub toolset: PrimField<String>,
}
impl BuildCesExampleMessagesElChunksElToolResponseElToolsetToolEl {
    pub fn build(self) -> CesExampleMessagesElChunksElToolResponseElToolsetToolEl {
        CesExampleMessagesElChunksElToolResponseElToolsetToolEl {
            tool_id: core::default::Default::default(),
            toolset: self.toolset,
        }
    }
}
pub struct CesExampleMessagesElChunksElToolResponseElToolsetToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElToolResponseElToolsetToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesExampleMessagesElChunksElToolResponseElToolsetToolElRef {
        CesExampleMessagesElChunksElToolResponseElToolsetToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElToolResponseElToolsetToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\nThe resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesExampleMessagesElChunksElToolResponseElDynamic {
    toolset_tool: Option<DynamicBlock<CesExampleMessagesElChunksElToolResponseElToolsetToolEl>>,
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksElToolResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    response: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset_tool: Option<Vec<CesExampleMessagesElChunksElToolResponseElToolsetToolEl>>,
    dynamic: CesExampleMessagesElChunksElToolResponseElDynamic,
}
impl CesExampleMessagesElChunksElToolResponseEl {
    #[doc = "Set the field `id`.\nThe matching ID of the tool call the response is for."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `tool`.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElToolResponseElToolsetToolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.toolset_tool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.toolset_tool = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesExampleMessagesElChunksElToolResponseEl {
    type O = BlockAssignable<CesExampleMessagesElChunksElToolResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksElToolResponseEl {
    #[doc = "The tool execution result in JSON object format.\nUse \"output\" key to specify tool response and \"error\" key to specify\nerror details (if any). If \"output\" and \"error\" keys are not specified,\nthen whole \"response\" is treated as tool execution result."]
    pub response: PrimField<String>,
}
impl BuildCesExampleMessagesElChunksElToolResponseEl {
    pub fn build(self) -> CesExampleMessagesElChunksElToolResponseEl {
        CesExampleMessagesElChunksElToolResponseEl {
            id: core::default::Default::default(),
            response: self.response,
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesExampleMessagesElChunksElToolResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElToolResponseElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElChunksElToolResponseElRef {
        CesExampleMessagesElChunksElToolResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElToolResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the tool."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe matching ID of the tool call the response is for."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe tool execution result in JSON object format.\nUse \"output\" key to specify tool response and \"error\" key to specify\nerror details (if any). If \"output\" and \"error\" keys are not specified,\nthen whole \"response\" is treated as tool execution result."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(
        &self,
    ) -> ListRef<CesExampleMessagesElChunksElToolResponseElToolsetToolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesExampleMessagesElChunksElDynamic {
    agent_transfer: Option<DynamicBlock<CesExampleMessagesElChunksElAgentTransferEl>>,
    image: Option<DynamicBlock<CesExampleMessagesElChunksElImageEl>>,
    tool_call: Option<DynamicBlock<CesExampleMessagesElChunksElToolCallEl>>,
    tool_response: Option<DynamicBlock<CesExampleMessagesElChunksElToolResponseEl>>,
}
#[derive(Serialize)]
pub struct CesExampleMessagesElChunksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_variables: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_transfer: Option<Vec<CesExampleMessagesElChunksElAgentTransferEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<CesExampleMessagesElChunksElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call: Option<Vec<CesExampleMessagesElChunksElToolCallEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_response: Option<Vec<CesExampleMessagesElChunksElToolResponseEl>>,
    dynamic: CesExampleMessagesElChunksElDynamic,
}
impl CesExampleMessagesElChunksEl {
    #[doc = "Set the field `text`.\nText data."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
    #[doc = "Set the field `updated_variables`.\nA struct represents variables that were updated in the conversation,\nkeyed by variable names."]
    pub fn set_updated_variables(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `agent_transfer`.\n"]
    pub fn set_agent_transfer(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElAgentTransferEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.agent_transfer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_transfer = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tool_call`.\n"]
    pub fn set_tool_call(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElToolCallEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_call = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tool_response`.\n"]
    pub fn set_tool_response(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksElToolResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_response = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesExampleMessagesElChunksEl {
    type O = BlockAssignable<CesExampleMessagesElChunksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesElChunksEl {}
impl BuildCesExampleMessagesElChunksEl {
    pub fn build(self) -> CesExampleMessagesElChunksEl {
        CesExampleMessagesElChunksEl {
            text: core::default::Default::default(),
            updated_variables: core::default::Default::default(),
            agent_transfer: core::default::Default::default(),
            image: core::default::Default::default(),
            tool_call: core::default::Default::default(),
            tool_response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesExampleMessagesElChunksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElChunksElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElChunksElRef {
        CesExampleMessagesElChunksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElChunksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nText data."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
    #[doc = "Get a reference to the value of field `updated_variables` after provisioning.\nA struct represents variables that were updated in the conversation,\nkeyed by variable names."]
    pub fn updated_variables(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `agent_transfer` after provisioning.\n"]
    pub fn agent_transfer(&self) -> ListRef<CesExampleMessagesElChunksElAgentTransferElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<CesExampleMessagesElChunksElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_call` after provisioning.\n"]
    pub fn tool_call(&self) -> ListRef<CesExampleMessagesElChunksElToolCallElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool_call", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_response` after provisioning.\n"]
    pub fn tool_response(&self) -> ListRef<CesExampleMessagesElChunksElToolResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tool_response", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesExampleMessagesElDynamic {
    chunks: Option<DynamicBlock<CesExampleMessagesElChunksEl>>,
}
#[derive(Serialize)]
pub struct CesExampleMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks: Option<Vec<CesExampleMessagesElChunksEl>>,
    dynamic: CesExampleMessagesElDynamic,
}
impl CesExampleMessagesEl {
    #[doc = "Set the field `role`.\nThe role within the conversation, e.g., user, agent."]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
    #[doc = "Set the field `chunks`.\n"]
    pub fn set_chunks(
        mut self,
        v: impl Into<BlockAssignable<CesExampleMessagesElChunksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chunks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chunks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesExampleMessagesEl {
    type O = BlockAssignable<CesExampleMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleMessagesEl {}
impl BuildCesExampleMessagesEl {
    pub fn build(self) -> CesExampleMessagesEl {
        CesExampleMessagesEl {
            role: core::default::Default::default(),
            chunks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesExampleMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleMessagesElRef {
    fn new(shared: StackShared, base: String) -> CesExampleMessagesElRef {
        CesExampleMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe role within the conversation, e.g., user, agent."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
    #[doc = "Get a reference to the value of field `chunks` after provisioning.\n"]
    pub fn chunks(&self) -> ListRef<CesExampleMessagesElChunksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.chunks", self.base))
    }
}
#[derive(Serialize)]
pub struct CesExampleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesExampleTimeoutsEl {
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
impl ToListMappable for CesExampleTimeoutsEl {
    type O = BlockAssignable<CesExampleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesExampleTimeoutsEl {}
impl BuildCesExampleTimeoutsEl {
    pub fn build(self) -> CesExampleTimeoutsEl {
        CesExampleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesExampleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesExampleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesExampleTimeoutsElRef {
        CesExampleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesExampleTimeoutsElRef {
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
struct CesExampleDynamic {
    messages: Option<DynamicBlock<CesExampleMessagesEl>>,
}
