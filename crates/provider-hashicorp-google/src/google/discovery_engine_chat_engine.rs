use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineChatEngineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    collection_id: PrimField<String>,
    data_store_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    industry_vertical: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_engine_config: Option<Vec<DiscoveryEngineChatEngineChatEngineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_config: Option<Vec<DiscoveryEngineChatEngineCommonConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineChatEngineTimeoutsEl>,
    dynamic: DiscoveryEngineChatEngineDynamic,
}
struct DiscoveryEngineChatEngine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineChatEngineData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineChatEngine(Rc<DiscoveryEngineChatEngine_>);
impl DiscoveryEngineChatEngine {
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
    #[doc = "Set the field `industry_vertical`.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn set_industry_vertical(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `chat_engine_config`.\n"]
    pub fn set_chat_engine_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineChatEngineChatEngineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chat_engine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chat_engine_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `common_config`.\n"]
    pub fn set_common_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineChatEngineCommonConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().common_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.common_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineChatEngineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `chat_engine_metadata` after provisioning.\nAdditional information of the Chat Engine."]
    pub fn chat_engine_metadata(
        &self,
    ) -> ListRef<DiscoveryEngineChatEngineChatEngineMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chat_engine_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID to use for chat engine."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the chat engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `chat_engine_config` after provisioning.\n"]
    pub fn chat_engine_config(&self) -> ListRef<DiscoveryEngineChatEngineChatEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chat_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineChatEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineChatEngineTimeoutsElRef {
        DiscoveryEngineChatEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineChatEngine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineChatEngine {}
impl ToListMappable for DiscoveryEngineChatEngine {
    type O = ListRef<DiscoveryEngineChatEngineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineChatEngine_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_chat_engine".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineChatEngine {
    pub tf_id: String,
    #[doc = "The collection ID."]
    pub collection_id: PrimField<String>,
    #[doc = "The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub data_store_ids: ListField<PrimField<String>>,
    #[doc = "The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: PrimField<String>,
    #[doc = "The ID to use for chat engine."]
    pub engine_id: PrimField<String>,
    #[doc = "Location."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineChatEngine {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineChatEngine {
        let out = DiscoveryEngineChatEngine(Rc::new(DiscoveryEngineChatEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineChatEngineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collection_id: self.collection_id,
                data_store_ids: self.data_store_ids,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                industry_vertical: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                chat_engine_config: core::default::Default::default(),
                common_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineChatEngineRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineChatEngineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chat_engine_metadata` after provisioning.\nAdditional information of the Chat Engine."]
    pub fn chat_engine_metadata(
        &self,
    ) -> ListRef<DiscoveryEngineChatEngineChatEngineMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chat_engine_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be 'SOLUTION_TYPE_CHAT'."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID to use for chat engine."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the chat engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `chat_engine_config` after provisioning.\n"]
    pub fn chat_engine_config(&self) -> ListRef<DiscoveryEngineChatEngineChatEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chat_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineChatEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineChatEngineTimeoutsElRef {
        DiscoveryEngineChatEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineChatEngineChatEngineMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dialogflow_agent: Option<PrimField<String>>,
}
impl DiscoveryEngineChatEngineChatEngineMetadataEl {
    #[doc = "Set the field `dialogflow_agent`.\n"]
    pub fn set_dialogflow_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dialogflow_agent = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineChatEngineChatEngineMetadataEl {
    type O = BlockAssignable<DiscoveryEngineChatEngineChatEngineMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineChatEngineChatEngineMetadataEl {}
impl BuildDiscoveryEngineChatEngineChatEngineMetadataEl {
    pub fn build(self) -> DiscoveryEngineChatEngineChatEngineMetadataEl {
        DiscoveryEngineChatEngineChatEngineMetadataEl {
            dialogflow_agent: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineChatEngineChatEngineMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineChatEngineMetadataElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineChatEngineChatEngineMetadataElRef {
        DiscoveryEngineChatEngineChatEngineMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineChatEngineChatEngineMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dialogflow_agent` after provisioning.\n"]
    pub fn dialogflow_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dialogflow_agent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    business: Option<PrimField<String>>,
    default_language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    time_zone: PrimField<String>,
}
impl DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
    #[doc = "Set the field `business`.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn set_business(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.business = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nAgent location for Agent creation, currently supported values: global/us/eu, it needs to be the same region as the Chat Engine."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
    type O = BlockAssignable<DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
    #[doc = "The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub default_language_code: PrimField<String>,
    #[doc = "The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
    pub time_zone: PrimField<String>,
}
impl BuildDiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
    pub fn build(self) -> DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
        DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl {
            business: core::default::Default::default(),
            default_language_code: self.default_language_code,
            location: core::default::Default::default(),
            time_zone: self.time_zone,
        }
    }
}
pub struct DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef {
        DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `business` after provisioning.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn business(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business", self.base))
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nAgent location for Agent creation, currently supported values: global/us/eu, it needs to be the same region as the Chat Engine."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineChatEngineChatEngineConfigElDynamic {
    agent_creation_config:
        Option<DynamicBlock<DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineChatEngineChatEngineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_cross_region: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialogflow_agent_to_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_creation_config:
        Option<Vec<DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl>>,
    dynamic: DiscoveryEngineChatEngineChatEngineConfigElDynamic,
}
impl DiscoveryEngineChatEngineChatEngineConfigEl {
    #[doc = "Set the field `allow_cross_region`.\nIf the flag set to true, we allow the agent and engine are in\ndifferent locations, otherwise the agent and engine are required to be\nin the same location. The flag is set to false by default.\nNote that the 'allow_cross_region' are one-time consumed by and passed\nto EngineService.CreateEngine. It means they cannot be retrieved using\nEngineService.GetEngine or EngineService.ListEngines API after engine\ncreation."]
    pub fn set_allow_cross_region(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_cross_region = Some(v.into());
        self
    }
    #[doc = "Set the field `dialogflow_agent_to_link`.\nThe resource name of an existing Dialogflow agent to link to this Chat Engine. Format: 'projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>'.\nExactly one of 'agent_creation_config' or 'dialogflow_agent_to_link' must be set."]
    pub fn set_dialogflow_agent_to_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dialogflow_agent_to_link = Some(v.into());
        self
    }
    #[doc = "Set the field `agent_creation_config`.\n"]
    pub fn set_agent_creation_config(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.agent_creation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_creation_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineChatEngineChatEngineConfigEl {
    type O = BlockAssignable<DiscoveryEngineChatEngineChatEngineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineChatEngineChatEngineConfigEl {}
impl BuildDiscoveryEngineChatEngineChatEngineConfigEl {
    pub fn build(self) -> DiscoveryEngineChatEngineChatEngineConfigEl {
        DiscoveryEngineChatEngineChatEngineConfigEl {
            allow_cross_region: core::default::Default::default(),
            dialogflow_agent_to_link: core::default::Default::default(),
            agent_creation_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineChatEngineChatEngineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineChatEngineConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineChatEngineChatEngineConfigElRef {
        DiscoveryEngineChatEngineChatEngineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineChatEngineChatEngineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_cross_region` after provisioning.\nIf the flag set to true, we allow the agent and engine are in\ndifferent locations, otherwise the agent and engine are required to be\nin the same location. The flag is set to false by default.\nNote that the 'allow_cross_region' are one-time consumed by and passed\nto EngineService.CreateEngine. It means they cannot be retrieved using\nEngineService.GetEngine or EngineService.ListEngines API after engine\ncreation."]
    pub fn allow_cross_region(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_cross_region", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dialogflow_agent_to_link` after provisioning.\nThe resource name of an existing Dialogflow agent to link to this Chat Engine. Format: 'projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>'.\nExactly one of 'agent_creation_config' or 'dialogflow_agent_to_link' must be set."]
    pub fn dialogflow_agent_to_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dialogflow_agent_to_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `agent_creation_config` after provisioning.\n"]
    pub fn agent_creation_config(
        &self,
    ) -> ListRef<DiscoveryEngineChatEngineChatEngineConfigElAgentCreationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_creation_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineChatEngineCommonConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<PrimField<String>>,
}
impl DiscoveryEngineChatEngineCommonConfigEl {
    #[doc = "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features."]
    pub fn set_company_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.company_name = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineChatEngineCommonConfigEl {
    type O = BlockAssignable<DiscoveryEngineChatEngineCommonConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineChatEngineCommonConfigEl {}
impl BuildDiscoveryEngineChatEngineCommonConfigEl {
    pub fn build(self) -> DiscoveryEngineChatEngineCommonConfigEl {
        DiscoveryEngineChatEngineCommonConfigEl {
            company_name: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineChatEngineCommonConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineCommonConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineChatEngineCommonConfigElRef {
        DiscoveryEngineChatEngineCommonConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineChatEngineCommonConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `company_name` after provisioning.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features."]
    pub fn company_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineChatEngineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineChatEngineTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineChatEngineTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineChatEngineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineChatEngineTimeoutsEl {}
impl BuildDiscoveryEngineChatEngineTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineChatEngineTimeoutsEl {
        DiscoveryEngineChatEngineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineChatEngineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineChatEngineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineChatEngineTimeoutsElRef {
        DiscoveryEngineChatEngineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineChatEngineTimeoutsElRef {
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
struct DiscoveryEngineChatEngineDynamic {
    chat_engine_config: Option<DynamicBlock<DiscoveryEngineChatEngineChatEngineConfigEl>>,
    common_config: Option<DynamicBlock<DiscoveryEngineChatEngineCommonConfigEl>>,
}
