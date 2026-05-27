use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    environmentid: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfillment: Option<Vec<DialogflowEnvironmentFulfillmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_to_speech_settings: Option<Vec<DialogflowEnvironmentTextToSpeechSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowEnvironmentTimeoutsEl>,
    dynamic: DialogflowEnvironmentDynamic,
}
struct DialogflowEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowEnvironmentData>,
}
#[derive(Clone)]
pub struct DialogflowEnvironment(Rc<DialogflowEnvironment_>);
impl DialogflowEnvironment {
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
    #[doc = "Set the field `agent_version`.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn set_agent_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_version = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe developer-provided description for this environment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `fulfillment`.\n"]
    pub fn set_fulfillment(
        self,
        v: impl Into<BlockAssignable<DialogflowEnvironmentFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fulfillment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fulfillment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_to_speech_settings`.\n"]
    pub fn set_text_to_speech_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowEnvironmentTextToSpeechSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().text_to_speech_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.text_to_speech_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe developer-provided description for this environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environmentid` after provisioning.\n"]
    pub fn environmentid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environmentid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this agent environment."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this environment."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfillment` after provisioning.\n"]
    pub fn fulfillment(&self) -> ListRef<DialogflowEnvironmentFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(
        &self,
    ) -> ListRef<DialogflowEnvironmentTextToSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_to_speech_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEnvironmentTimeoutsElRef {
        DialogflowEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowEnvironment {}
impl ToListMappable for DialogflowEnvironment {
    type O = ListRef<DialogflowEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub environmentid: PrimField<String>,
}
impl BuildDialogflowEnvironment {
    pub fn build(self, stack: &mut Stack) -> DialogflowEnvironment {
        let out = DialogflowEnvironment(Rc::new(DialogflowEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                environmentid: self.environmentid,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                fulfillment: core::default::Default::default(),
                text_to_speech_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe developer-provided description for this environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environmentid` after provisioning.\n"]
    pub fn environmentid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environmentid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this agent environment."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this environment."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfillment` after provisioning.\n"]
    pub fn fulfillment(&self) -> ListRef<DialogflowEnvironmentFulfillmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(
        &self,
    ) -> ListRef<DialogflowEnvironmentTextToSpeechSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_to_speech_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEnvironmentTimeoutsElRef {
        DialogflowEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentFulfillmentElFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DialogflowEnvironmentFulfillmentElFeaturesEl {}
impl ToListMappable for DialogflowEnvironmentFulfillmentElFeaturesEl {
    type O = BlockAssignable<DialogflowEnvironmentFulfillmentElFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentFulfillmentElFeaturesEl {
    #[doc = "The type of the feature that enabled for fulfillment. Possible values: [\"TYPE_UNSPECIFIED\", \"SMALLTALK\"]"]
    pub type_: PrimField<String>,
}
impl BuildDialogflowEnvironmentFulfillmentElFeaturesEl {
    pub fn build(self) -> DialogflowEnvironmentFulfillmentElFeaturesEl {
        DialogflowEnvironmentFulfillmentElFeaturesEl { type_: self.type_ }
    }
}
pub struct DialogflowEnvironmentFulfillmentElFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentFulfillmentElFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEnvironmentFulfillmentElFeaturesElRef {
        DialogflowEnvironmentFulfillmentElFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentFulfillmentElFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the feature that enabled for fulfillment. Possible values: [\"TYPE_UNSPECIFIED\", \"SMALLTALK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentFulfillmentElGenericWebServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers: Option<RecField<PrimField<String>>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl DialogflowEnvironmentFulfillmentElGenericWebServiceEl {
    #[doc = "Set the field `password`.\nThe password for HTTP Basic authentication."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers`.\nThe HTTP request headers to send together with fulfillment requests"]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nThe user name for HTTP Basic authentication."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowEnvironmentFulfillmentElGenericWebServiceEl {
    type O = BlockAssignable<DialogflowEnvironmentFulfillmentElGenericWebServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentFulfillmentElGenericWebServiceEl {
    #[doc = "The fulfillment URI for receiving POST requests. It must use https protocol."]
    pub uri: PrimField<String>,
}
impl BuildDialogflowEnvironmentFulfillmentElGenericWebServiceEl {
    pub fn build(self) -> DialogflowEnvironmentFulfillmentElGenericWebServiceEl {
        DialogflowEnvironmentFulfillmentElGenericWebServiceEl {
            password: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentFulfillmentElGenericWebServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentFulfillmentElGenericWebServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowEnvironmentFulfillmentElGenericWebServiceElRef {
        DialogflowEnvironmentFulfillmentElGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentFulfillmentElGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password for HTTP Basic authentication."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with fulfillment requests"]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe fulfillment URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe user name for HTTP Basic authentication."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowEnvironmentFulfillmentElDynamic {
    features: Option<DynamicBlock<DialogflowEnvironmentFulfillmentElFeaturesEl>>,
    generic_web_service:
        Option<DynamicBlock<DialogflowEnvironmentFulfillmentElGenericWebServiceEl>>,
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentFulfillmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<Vec<DialogflowEnvironmentFulfillmentElFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowEnvironmentFulfillmentElGenericWebServiceEl>>,
    dynamic: DialogflowEnvironmentFulfillmentElDynamic,
}
impl DialogflowEnvironmentFulfillmentEl {
    #[doc = "Set the field `display_name`.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe unique identifier of the fulfillment. Supports the following formats:\n- projects/<Project ID>/agent/fulfillment\n- projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `features`.\n"]
    pub fn set_features(
        mut self,
        v: impl Into<BlockAssignable<DialogflowEnvironmentFulfillmentElFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v: impl Into<BlockAssignable<DialogflowEnvironmentFulfillmentElGenericWebServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generic_web_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_web_service = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowEnvironmentFulfillmentEl {
    type O = BlockAssignable<DialogflowEnvironmentFulfillmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentFulfillmentEl {}
impl BuildDialogflowEnvironmentFulfillmentEl {
    pub fn build(self) -> DialogflowEnvironmentFulfillmentEl {
        DialogflowEnvironmentFulfillmentEl {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
            features: core::default::Default::default(),
            generic_web_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentFulfillmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentFulfillmentElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEnvironmentFulfillmentElRef {
        DialogflowEnvironmentFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentFulfillmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the fulfillment. Supports the following formats:\n- projects/<Project ID>/agent/fulfillment\n- projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ListRef<DialogflowEnvironmentFulfillmentElFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features", self.base))
    }
    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(
        &self,
    ) -> ListRef<DialogflowEnvironmentFulfillmentElGenericWebServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generic_web_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml_gender: Option<PrimField<String>>,
}
impl DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {
    #[doc = "Set the field `name`.\nThe name of the voice."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `ssml_gender`.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn set_ssml_gender(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssml_gender = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl
{
    type O = BlockAssignable<
        DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {}
impl BuildDialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {
    pub fn build(
        self,
    ) -> DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {
        DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl {
            name: core::default::Default::default(),
            ssml_gender: core::default::Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef {
        DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the voice."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `ssml_gender` after provisioning.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn ssml_gender(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssml_gender", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElDynamic {
    voice: Option<
        DynamicBlock<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl>,
    >,
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effects_profile_id: Option<ListField<PrimField<String>>>,
    language: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speaking_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_gain_db: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<Vec<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl>>,
    dynamic: DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElDynamic,
}
impl DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
    #[doc = "Set the field `effects_profile_id`.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn set_effects_profile_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.effects_profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `pitch`.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn set_pitch(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pitch = Some(v.into());
        self
    }
    #[doc = "Set the field `speaking_rate`.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn set_speaking_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.speaking_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_gain_db`.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn set_volume_gain_db(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_gain_db = Some(v.into());
        self
    }
    #[doc = "Set the field `voice`.\n"]
    pub fn set_voice(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.voice = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.voice = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
    type O = BlockAssignable<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
    #[doc = ""]
    pub language: PrimField<String>,
}
impl BuildDialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
    pub fn build(self) -> DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
        DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl {
            effects_profile_id: core::default::Default::default(),
            language: self.language,
            pitch: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
            voice: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElRef {
        DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effects_profile_id` after provisioning.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn effects_profile_id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effects_profile_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }
    #[doc = "Get a reference to the value of field `pitch` after provisioning.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn pitch(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pitch", self.base))
    }
    #[doc = "Get a reference to the value of field `speaking_rate` after provisioning.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn speaking_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.speaking_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_gain_db` after provisioning.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn volume_gain_db(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_gain_db", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `voice` after provisioning.\n"]
    pub fn voice(
        &self,
    ) -> ListRef<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsElVoiceElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowEnvironmentTextToSpeechSettingsElDynamic {
    synthesize_speech_configs:
        Option<DynamicBlock<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentTextToSpeechSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_text_to_speech: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_audio_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate_hertz: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesize_speech_configs:
        Option<Vec<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl>>,
    dynamic: DialogflowEnvironmentTextToSpeechSettingsElDynamic,
}
impl DialogflowEnvironmentTextToSpeechSettingsEl {
    #[doc = "Set the field `enable_text_to_speech`.\nIndicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained."]
    pub fn set_enable_text_to_speech(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_text_to_speech = Some(v.into());
        self
    }
    #[doc = "Set the field `output_audio_encoding`.\nAudio encoding of the synthesized audio content. Possible values: [\"OUTPUT_AUDIO_ENCODING_UNSPECIFIED\", \"OUTPUT_AUDIO_ENCODING_LINEAR_16\", \"OUTPUT_AUDIO_ENCODING_MP3\", \"OUTPUT_AUDIO_ENCODING_MP3_64_KBPS\", \"OUTPUT_AUDIO_ENCODING_OGG_OPUS\", \"OUTPUT_AUDIO_ENCODING_MULAW\", \"OUTPUT_AUDIO_ENCODING_ALAW\"]"]
    pub fn set_output_audio_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_audio_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate_hertz`.\nThe synthesis sample rate (in hertz) for this audio."]
    pub fn set_sample_rate_hertz(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate_hertz = Some(v.into());
        self
    }
    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowEnvironmentTextToSpeechSettingsElSynthesizeSpeechConfigsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.synthesize_speech_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.synthesize_speech_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowEnvironmentTextToSpeechSettingsEl {
    type O = BlockAssignable<DialogflowEnvironmentTextToSpeechSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentTextToSpeechSettingsEl {}
impl BuildDialogflowEnvironmentTextToSpeechSettingsEl {
    pub fn build(self) -> DialogflowEnvironmentTextToSpeechSettingsEl {
        DialogflowEnvironmentTextToSpeechSettingsEl {
            enable_text_to_speech: core::default::Default::default(),
            output_audio_encoding: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentTextToSpeechSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentTextToSpeechSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEnvironmentTextToSpeechSettingsElRef {
        DialogflowEnvironmentTextToSpeechSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentTextToSpeechSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_text_to_speech` after provisioning.\nIndicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained."]
    pub fn enable_text_to_speech(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_text_to_speech", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_audio_encoding` after provisioning.\nAudio encoding of the synthesized audio content. Possible values: [\"OUTPUT_AUDIO_ENCODING_UNSPECIFIED\", \"OUTPUT_AUDIO_ENCODING_LINEAR_16\", \"OUTPUT_AUDIO_ENCODING_MP3\", \"OUTPUT_AUDIO_ENCODING_MP3_64_KBPS\", \"OUTPUT_AUDIO_ENCODING_OGG_OPUS\", \"OUTPUT_AUDIO_ENCODING_MULAW\", \"OUTPUT_AUDIO_ENCODING_ALAW\"]"]
    pub fn output_audio_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_audio_encoding", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nThe synthesis sample rate (in hertz) for this audio."]
    pub fn sample_rate_hertz(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sample_rate_hertz", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowEnvironmentTimeoutsEl {
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
impl ToListMappable for DialogflowEnvironmentTimeoutsEl {
    type O = BlockAssignable<DialogflowEnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEnvironmentTimeoutsEl {}
impl BuildDialogflowEnvironmentTimeoutsEl {
    pub fn build(self) -> DialogflowEnvironmentTimeoutsEl {
        DialogflowEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEnvironmentTimeoutsElRef {
        DialogflowEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEnvironmentTimeoutsElRef {
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
struct DialogflowEnvironmentDynamic {
    fulfillment: Option<DynamicBlock<DialogflowEnvironmentFulfillmentEl>>,
    text_to_speech_settings: Option<DynamicBlock<DialogflowEnvironmentTextToSpeechSettingsEl>>,
}
