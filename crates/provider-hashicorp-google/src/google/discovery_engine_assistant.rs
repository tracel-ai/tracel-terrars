use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineAssistantData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    assistant_id: PrimField<String>,
    collection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_grounding_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_policy: Option<Vec<DiscoveryEngineAssistantCustomerPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<Vec<DiscoveryEngineAssistantGenerationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineAssistantTimeoutsEl>,
    dynamic: DiscoveryEngineAssistantDynamic,
}
struct DiscoveryEngineAssistant_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineAssistantData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineAssistant(Rc<DiscoveryEngineAssistant_>);
impl DiscoveryEngineAssistant {
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
    #[doc = "Set the field `description`.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `web_grounding_type`.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn set_web_grounding_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().web_grounding_type = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_policy`.\n"]
    pub fn set_customer_policy(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineAssistantCustomerPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generation_config`.\n"]
    pub fn set_generation_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineAssistantGenerationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineAssistantTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `assistant_id` after provisioning.\nThe unique id of the assistant."]
    pub fn assistant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assistant_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe unique id of the collection."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe unique id of the engine."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the assistant.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}'\n\nIt must be a UTF-8 encoded string with a length limit of 1024 characters."]
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
    #[doc = "Get a reference to the value of field `web_grounding_type` after provisioning.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn web_grounding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_grounding_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_policy` after provisioning.\n"]
    pub fn customer_policy(&self) -> ListRef<DiscoveryEngineAssistantCustomerPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation_config` after provisioning.\n"]
    pub fn generation_config(&self) -> ListRef<DiscoveryEngineAssistantGenerationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generation_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAssistantTimeoutsElRef {
        DiscoveryEngineAssistantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineAssistant {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineAssistant {}
impl ToListMappable for DiscoveryEngineAssistant {
    type O = ListRef<DiscoveryEngineAssistantRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineAssistant_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_assistant".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineAssistant {
    pub tf_id: String,
    #[doc = "The unique id of the assistant."]
    pub assistant_id: PrimField<String>,
    #[doc = "The unique id of the collection."]
    pub collection_id: PrimField<String>,
    #[doc = "The assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub display_name: PrimField<String>,
    #[doc = "The unique id of the engine."]
    pub engine_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineAssistant {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineAssistant {
        let out = DiscoveryEngineAssistant(Rc::new(DiscoveryEngineAssistant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineAssistantData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assistant_id: self.assistant_id,
                collection_id: self.collection_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                web_grounding_type: core::default::Default::default(),
                customer_policy: core::default::Default::default(),
                generation_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineAssistantRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineAssistantRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assistant_id` after provisioning.\nThe unique id of the assistant."]
    pub fn assistant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assistant_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe unique id of the collection."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for additional information. Expected to be shown on the\nconfiguration UI, not to the users of the assistant."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe assistant display name.\n\nIt must be a UTF-8 encoded string with a length limit of 128 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe unique id of the engine."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the assistant.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}'\n\nIt must be a UTF-8 encoded string with a length limit of 1024 characters."]
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
    #[doc = "Get a reference to the value of field `web_grounding_type` after provisioning.\nThe type of web grounding to use.\nThe supported values: 'WEB_GROUNDING_TYPE_DISABLED', 'WEB_GROUNDING_TYPE_GOOGLE_SEARCH', 'WEB_GROUNDING_TYPE_ENTERPRISE_WEB_SEARCH'."]
    pub fn web_grounding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_grounding_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_policy` after provisioning.\n"]
    pub fn customer_policy(&self) -> ListRef<DiscoveryEngineAssistantCustomerPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation_config` after provisioning.\n"]
    pub fn generation_config(&self) -> ListRef<DiscoveryEngineAssistantGenerationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generation_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAssistantTimeoutsElRef {
        DiscoveryEngineAssistantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_diacritics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_type: Option<PrimField<String>>,
    phrase: PrimField<String>,
}
impl DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
    #[doc = "Set the field `ignore_diacritics`.\nIf true, diacritical marks (e.g., accents, umlauts) are ignored when\nmatching banned phrases. For example, \"cafe\" would match \"café\"."]
    pub fn set_ignore_diacritics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_diacritics = Some(v.into());
        self
    }
    #[doc = "Set the field `match_type`.\nMatch type for the banned phrase.\nThe supported values: 'SIMPLE_STRING_MATCH', 'WORD_BOUNDARY_STRING_MATCH'."]
    pub fn set_match_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_type = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
    type O = BlockAssignable<DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
    #[doc = "The raw string content to be banned."]
    pub phrase: PrimField<String>,
}
impl BuildDiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
    pub fn build(self) -> DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
        DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl {
            ignore_diacritics: core::default::Default::default(),
            match_type: core::default::Default::default(),
            phrase: self.phrase,
        }
    }
}
pub struct DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef {
        DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ignore_diacritics` after provisioning.\nIf true, diacritical marks (e.g., accents, umlauts) are ignored when\nmatching banned phrases. For example, \"cafe\" would match \"café\"."]
    pub fn ignore_diacritics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_diacritics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_type` after provisioning.\nMatch type for the banned phrase.\nThe supported values: 'SIMPLE_STRING_MATCH', 'WORD_BOUNDARY_STRING_MATCH'."]
    pub fn match_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
    #[doc = "Get a reference to the value of field `phrase` after provisioning.\nThe raw string content to be banned."]
    pub fn phrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phrase", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_mode: Option<PrimField<String>>,
    response_template: PrimField<String>,
    user_prompt_template: PrimField<String>,
}
impl DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
    #[doc = "Set the field `failure_mode`.\nDefines the failure mode for Model Armor sanitization.\nThe supported values: 'FAIL_OPEN', 'FAIL_CLOSED'."]
    pub fn set_failure_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failure_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
    type O = BlockAssignable<DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
    #[doc = "The resource name of the Model Armor template for sanitizing assistant\nresponses. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the assistant\nresponse."]
    pub response_template: PrimField<String>,
    #[doc = "The resource name of the Model Armor template for sanitizing user\nprompts. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the user prompt."]
    pub user_prompt_template: PrimField<String>,
}
impl BuildDiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
    pub fn build(self) -> DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
        DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl {
            failure_mode: core::default::Default::default(),
            response_template: self.response_template,
            user_prompt_template: self.user_prompt_template,
        }
    }
}
pub struct DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef {
        DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_mode` after provisioning.\nDefines the failure mode for Model Armor sanitization.\nThe supported values: 'FAIL_OPEN', 'FAIL_CLOSED'."]
    pub fn failure_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `response_template` after provisioning.\nThe resource name of the Model Armor template for sanitizing assistant\nresponses. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the assistant\nresponse."]
    pub fn response_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_prompt_template` after provisioning.\nThe resource name of the Model Armor template for sanitizing user\nprompts. Format:\n'projects/{project}/locations/{location}/templates/{template_id}'\n\nIf not specified, no sanitization will be applied to the user prompt."]
    pub fn user_prompt_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_prompt_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineAssistantCustomerPolicyElDynamic {
    banned_phrases: Option<DynamicBlock<DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl>>,
    model_armor_config:
        Option<DynamicBlock<DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantCustomerPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_phrases: Option<Vec<DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_armor_config: Option<Vec<DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl>>,
    dynamic: DiscoveryEngineAssistantCustomerPolicyElDynamic,
}
impl DiscoveryEngineAssistantCustomerPolicyEl {
    #[doc = "Set the field `banned_phrases`.\n"]
    pub fn set_banned_phrases(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.banned_phrases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.banned_phrases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_armor_config`.\n"]
    pub fn set_model_armor_config(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_armor_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_armor_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineAssistantCustomerPolicyEl {
    type O = BlockAssignable<DiscoveryEngineAssistantCustomerPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantCustomerPolicyEl {}
impl BuildDiscoveryEngineAssistantCustomerPolicyEl {
    pub fn build(self) -> DiscoveryEngineAssistantCustomerPolicyEl {
        DiscoveryEngineAssistantCustomerPolicyEl {
            banned_phrases: core::default::Default::default(),
            model_armor_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineAssistantCustomerPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantCustomerPolicyElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineAssistantCustomerPolicyElRef {
        DiscoveryEngineAssistantCustomerPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantCustomerPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `banned_phrases` after provisioning.\n"]
    pub fn banned_phrases(
        &self,
    ) -> ListRef<DiscoveryEngineAssistantCustomerPolicyElBannedPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_phrases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_armor_config` after provisioning.\n"]
    pub fn model_armor_config(
        &self,
    ) -> ListRef<DiscoveryEngineAssistantCustomerPolicyElModelArmorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_armor_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_system_instruction: Option<PrimField<String>>,
}
impl DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
    #[doc = "Set the field `additional_system_instruction`.\nAdditional system instruction that will be added to the default system instruction."]
    pub fn set_additional_system_instruction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_system_instruction = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
    type O = BlockAssignable<DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {}
impl BuildDiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
    pub fn build(self) -> DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
        DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl {
            additional_system_instruction: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef {
        DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_system_instruction` after provisioning.\nAdditional system instruction that will be added to the default system instruction."]
    pub fn additional_system_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_system_instruction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineAssistantGenerationConfigElDynamic {
    system_instruction:
        Option<DynamicBlock<DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantGenerationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<Vec<DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl>>,
    dynamic: DiscoveryEngineAssistantGenerationConfigElDynamic,
}
impl DiscoveryEngineAssistantGenerationConfigEl {
    #[doc = "Set the field `default_language`.\nThe default language to use for the generation of the assistant response.\nUse an ISO 639-1 language code such as 'en'.\nIf not specified, the language will be automatically detected."]
    pub fn set_default_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_language = Some(v.into());
        self
    }
    #[doc = "Set the field `system_instruction`.\n"]
    pub fn set_system_instruction(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineAssistantGenerationConfigElSystemInstructionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.system_instruction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.system_instruction = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineAssistantGenerationConfigEl {
    type O = BlockAssignable<DiscoveryEngineAssistantGenerationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantGenerationConfigEl {}
impl BuildDiscoveryEngineAssistantGenerationConfigEl {
    pub fn build(self) -> DiscoveryEngineAssistantGenerationConfigEl {
        DiscoveryEngineAssistantGenerationConfigEl {
            default_language: core::default::Default::default(),
            system_instruction: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineAssistantGenerationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantGenerationConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineAssistantGenerationConfigElRef {
        DiscoveryEngineAssistantGenerationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantGenerationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_language` after provisioning.\nThe default language to use for the generation of the assistant response.\nUse an ISO 639-1 language code such as 'en'.\nIf not specified, the language will be automatically detected."]
    pub fn default_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `system_instruction` after provisioning.\n"]
    pub fn system_instruction(
        &self,
    ) -> ListRef<DiscoveryEngineAssistantGenerationConfigElSystemInstructionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.system_instruction", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAssistantTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineAssistantTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineAssistantTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineAssistantTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAssistantTimeoutsEl {}
impl BuildDiscoveryEngineAssistantTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineAssistantTimeoutsEl {
        DiscoveryEngineAssistantTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineAssistantTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAssistantTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineAssistantTimeoutsElRef {
        DiscoveryEngineAssistantTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAssistantTimeoutsElRef {
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
struct DiscoveryEngineAssistantDynamic {
    customer_policy: Option<DynamicBlock<DiscoveryEngineAssistantCustomerPolicyEl>>,
    generation_config: Option<DynamicBlock<DiscoveryEngineAssistantGenerationConfigEl>>,
}
