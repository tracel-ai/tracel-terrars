use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_instruction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrails: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_execution_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_processing_config: Option<Vec<CesAppAudioProcessingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_settings: Option<Vec<CesAppClientCertificateSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_settings: Option<Vec<CesAppDataStoreSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_channel_profile: Option<Vec<CesAppDefaultChannelProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_metrics_thresholds: Option<Vec<CesAppEvaluationMetricsThresholdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_settings: Option<Vec<CesAppLanguageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_settings: Option<Vec<CesAppLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<Vec<CesAppModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone_settings: Option<Vec<CesAppTimeZoneSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesAppTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_declarations: Option<Vec<CesAppVariableDeclarationsEl>>,
    dynamic: CesAppDynamic,
}
struct CesApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesAppData>,
}
#[derive(Clone)]
pub struct CesApp(Rc<CesApp_>);
impl CesApp {
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
    #[doc = "Set the field `description`.\nHuman-readable description of the app."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `global_instruction`.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn set_global_instruction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().global_instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrails`.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn set_guardrails(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().guardrails = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `pinned`.\nWhether the app is pinned in the app list."]
    pub fn set_pinned(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pinned = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `root_agent`.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_root_agent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `tool_execution_mode`.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn set_tool_execution_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tool_execution_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_processing_config`.\n"]
    pub fn set_audio_processing_config(
        self,
        v: impl Into<BlockAssignable<CesAppAudioProcessingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_processing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_processing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        self,
        v: impl Into<BlockAssignable<CesAppClientCertificateSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_store_settings`.\n"]
    pub fn set_data_store_settings(
        self,
        v: impl Into<BlockAssignable<CesAppDataStoreSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_channel_profile`.\n"]
    pub fn set_default_channel_profile(
        self,
        v: impl Into<BlockAssignable<CesAppDefaultChannelProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_channel_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_channel_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `evaluation_metrics_thresholds`.\n"]
    pub fn set_evaluation_metrics_thresholds(
        self,
        v: impl Into<BlockAssignable<CesAppEvaluationMetricsThresholdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_metrics_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .evaluation_metrics_thresholds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `language_settings`.\n"]
    pub fn set_language_settings(
        self,
        v: impl Into<BlockAssignable<CesAppLanguageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().language_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.language_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(self, v: impl Into<BlockAssignable<CesAppModelSettingsEl>>) -> Self {
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
    #[doc = "Set the field `time_zone_settings`.\n"]
    pub fn set_time_zone_settings(
        self,
        v: impl Into<BlockAssignable<CesAppTimeZoneSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_zone_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_zone_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesAppTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `variable_declarations`.\n"]
    pub fn set_variable_declarations(
        self,
        v: impl Into<BlockAssignable<CesAppVariableDeclarationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variable_declarations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variable_declarations = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app was created."]
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
    #[doc = "Get a reference to the value of field `deployment_count` after provisioning.\nNumber of deployments in the app."]
    pub fn deployment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the app."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the app."]
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
    #[doc = "Get a reference to the value of field `global_instruction` after provisioning.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn global_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pinned` after provisioning.\nWhether the app is pinned in the app list."]
    pub fn pinned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pinned", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_agent` after provisioning.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn root_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_execution_mode` after provisioning.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn tool_execution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_execution_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the app was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(&self) -> ListRef<CesAppAudioProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_processing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> ListRef<CesAppClientCertificateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(&self) -> ListRef<CesAppDataStoreSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(&self) -> ListRef<CesAppDefaultChannelProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_channel_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(&self) -> ListRef<CesAppEvaluationMetricsThresholdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.evaluation_metrics_thresholds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> ListRef<CesAppLanguageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.language_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> ListRef<CesAppLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAppModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> ListRef<CesAppTimeZoneSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_zone_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppTimeoutsElRef {
        CesAppTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(&self) -> ListRef<CesAppVariableDeclarationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.variable_declarations", self.extract_ref()),
        )
    }
}
impl Referable for CesApp {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesApp {}
impl ToListMappable for CesApp {
    type O = ListRef<CesAppRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesApp_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_app".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesApp {
    pub tf_id: String,
    #[doc = "The ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub app_id: PrimField<String>,
    #[doc = "Display name of the app."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesApp {
    pub fn build(self, stack: &mut Stack) -> CesApp {
        let out = CesApp(Rc::new(CesApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_id: self.app_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                global_instruction: core::default::Default::default(),
                guardrails: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                metadata: core::default::Default::default(),
                pinned: core::default::Default::default(),
                project: core::default::Default::default(),
                root_agent: core::default::Default::default(),
                tool_execution_mode: core::default::Default::default(),
                audio_processing_config: core::default::Default::default(),
                client_certificate_settings: core::default::Default::default(),
                data_store_settings: core::default::Default::default(),
                default_channel_profile: core::default::Default::default(),
                evaluation_metrics_thresholds: core::default::Default::default(),
                language_settings: core::default::Default::default(),
                logging_settings: core::default::Default::default(),
                model_settings: core::default::Default::default(),
                time_zone_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                variable_declarations: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesAppRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesAppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app was created."]
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
    #[doc = "Get a reference to the value of field `deployment_count` after provisioning.\nNumber of deployments in the app."]
    pub fn deployment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the app."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the app."]
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
    #[doc = "Get a reference to the value of field `global_instruction` after provisioning.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn global_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_instruction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pinned` after provisioning.\nWhether the app is pinned in the app list."]
    pub fn pinned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pinned", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_agent` after provisioning.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn root_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_execution_mode` after provisioning.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn tool_execution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_execution_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the app was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(&self) -> ListRef<CesAppAudioProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_processing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> ListRef<CesAppClientCertificateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(&self) -> ListRef<CesAppDataStoreSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(&self) -> ListRef<CesAppDefaultChannelProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_channel_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(&self) -> ListRef<CesAppEvaluationMetricsThresholdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.evaluation_metrics_thresholds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> ListRef<CesAppLanguageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.language_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> ListRef<CesAppLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAppModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> ListRef<CesAppTimeZoneSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_zone_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppTimeoutsElRef {
        CesAppTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(&self) -> ListRef<CesAppVariableDeclarationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.variable_declarations", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppAudioProcessingConfigElAmbientSoundConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prebuilt_ambient_sound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_gain_db: Option<PrimField<f64>>,
}
impl CesAppAudioProcessingConfigElAmbientSoundConfigEl {
    #[doc = "Set the field `gcs_uri`.\nAmbient noise as a mono-channel, 16kHz WAV file stored in [Cloud\nStorage](https://cloud.google.com/storage).\nNote: Please make sure the CES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com' has\n'storage.objects.get' permission to the Cloud Storage object."]
    pub fn set_gcs_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `prebuilt_ambient_sound`.\nName of the prebuilt ambient sound.\nValid values are: - \"coffee_shop\" - \"keyboard\" - \"keypad\" - \"hum\"\n-\"office_1\" - \"office_2\" - \"office_3\"\n-\"room_1\" - \"room_2\" - \"room_3\"\n-\"room_4\" - \"room_5\" - \"air_conditioner\""]
    pub fn set_prebuilt_ambient_sound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prebuilt_ambient_sound = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_gain_db`.\nVolume gain (in dB) of the normal native volume supported by\nambient noise, in the range [-96.0, 16.0]. If unset, or set to a value of\n0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)\nwill play at approximately half the amplitude of the normal native signal\namplitude. A value of +6.0 (dB) will play at approximately twice the\namplitude of the normal native signal amplitude. We strongly recommend not\nto exceed +10 (dB) as there's usually no effective increase in loudness for\nany value greater than that."]
    pub fn set_volume_gain_db(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_gain_db = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppAudioProcessingConfigElAmbientSoundConfigEl {
    type O = BlockAssignable<CesAppAudioProcessingConfigElAmbientSoundConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppAudioProcessingConfigElAmbientSoundConfigEl {}
impl BuildCesAppAudioProcessingConfigElAmbientSoundConfigEl {
    pub fn build(self) -> CesAppAudioProcessingConfigElAmbientSoundConfigEl {
        CesAppAudioProcessingConfigElAmbientSoundConfigEl {
            gcs_uri: core::default::Default::default(),
            prebuilt_ambient_sound: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
        }
    }
}
pub struct CesAppAudioProcessingConfigElAmbientSoundConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppAudioProcessingConfigElAmbientSoundConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppAudioProcessingConfigElAmbientSoundConfigElRef {
        CesAppAudioProcessingConfigElAmbientSoundConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppAudioProcessingConfigElAmbientSoundConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_uri` after provisioning.\nAmbient noise as a mono-channel, 16kHz WAV file stored in [Cloud\nStorage](https://cloud.google.com/storage).\nNote: Please make sure the CES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com' has\n'storage.objects.get' permission to the Cloud Storage object."]
    pub fn gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `prebuilt_ambient_sound` after provisioning.\nName of the prebuilt ambient sound.\nValid values are: - \"coffee_shop\" - \"keyboard\" - \"keypad\" - \"hum\"\n-\"office_1\" - \"office_2\" - \"office_3\"\n-\"room_1\" - \"room_2\" - \"room_3\"\n-\"room_4\" - \"room_5\" - \"air_conditioner\""]
    pub fn prebuilt_ambient_sound(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prebuilt_ambient_sound", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_gain_db` after provisioning.\nVolume gain (in dB) of the normal native volume supported by\nambient noise, in the range [-96.0, 16.0]. If unset, or set to a value of\n0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)\nwill play at approximately half the amplitude of the normal native signal\namplitude. A value of +6.0 (dB) will play at approximately twice the\namplitude of the normal native signal amplitude. We strongly recommend not\nto exceed +10 (dB) as there's usually no effective increase in loudness for\nany value greater than that."]
    pub fn volume_gain_db(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_gain_db", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppAudioProcessingConfigElBargeInConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    barge_in_awareness: Option<PrimField<bool>>,
}
impl CesAppAudioProcessingConfigElBargeInConfigEl {
    #[doc = "Set the field `barge_in_awareness`.\nIf enabled, the agent will adapt its next response based on the assumption\nthat the user hasn't heard the full preceding agent message.\nThis should not be used in scenarios where agent responses are displayed\nvisually."]
    pub fn set_barge_in_awareness(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.barge_in_awareness = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppAudioProcessingConfigElBargeInConfigEl {
    type O = BlockAssignable<CesAppAudioProcessingConfigElBargeInConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppAudioProcessingConfigElBargeInConfigEl {}
impl BuildCesAppAudioProcessingConfigElBargeInConfigEl {
    pub fn build(self) -> CesAppAudioProcessingConfigElBargeInConfigEl {
        CesAppAudioProcessingConfigElBargeInConfigEl {
            barge_in_awareness: core::default::Default::default(),
        }
    }
}
pub struct CesAppAudioProcessingConfigElBargeInConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppAudioProcessingConfigElBargeInConfigElRef {
    fn new(shared: StackShared, base: String) -> CesAppAudioProcessingConfigElBargeInConfigElRef {
        CesAppAudioProcessingConfigElBargeInConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppAudioProcessingConfigElBargeInConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `barge_in_awareness` after provisioning.\nIf enabled, the agent will adapt its next response based on the assumption\nthat the user hasn't heard the full preceding agent message.\nThis should not be used in scenarios where agent responses are displayed\nvisually."]
    pub fn barge_in_awareness(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.barge_in_awareness", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speaking_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<PrimField<String>>,
}
impl CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    #[doc = "Set the field `speaking_rate`.\nThe speaking rate/speed in the range [0.25, 2.0]. 1.0 is the normal native\nspeed supported by the specific voice. 2.0 is twice as fast, and 0.5 is\nhalf as fast. Values outside of the range [0.25, 2.0] will return an error."]
    pub fn set_speaking_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.speaking_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `voice`.\nThe name of the voice. If not set, the service will choose a\nvoice based on the other parameters such as language_code.\nFor the list of available voices, please refer to Supported voices and\nlanguages from Cloud Text-to-Speech."]
    pub fn set_voice(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.voice = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    type O = BlockAssignable<CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    #[doc = ""]
    pub language_code: PrimField<String>,
}
impl BuildCesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    pub fn build(self) -> CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
        CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl {
            language_code: self.language_code,
            speaking_rate: core::default::Default::default(),
            voice: core::default::Default::default(),
        }
    }
}
pub struct CesAppAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
        CesAppAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speaking_rate` after provisioning.\nThe speaking rate/speed in the range [0.25, 2.0]. 1.0 is the normal native\nspeed supported by the specific voice. 2.0 is twice as fast, and 0.5 is\nhalf as fast. Values outside of the range [0.25, 2.0] will return an error."]
    pub fn speaking_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.speaking_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `voice` after provisioning.\nThe name of the voice. If not set, the service will choose a\nvoice based on the other parameters such as language_code.\nFor the list of available voices, please refer to Supported voices and\nlanguages from Cloud Text-to-Speech."]
    pub fn voice(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesAppAudioProcessingConfigElDynamic {
    ambient_sound_config: Option<DynamicBlock<CesAppAudioProcessingConfigElAmbientSoundConfigEl>>,
    barge_in_config: Option<DynamicBlock<CesAppAudioProcessingConfigElBargeInConfigEl>>,
    synthesize_speech_configs:
        Option<DynamicBlock<CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl>>,
}
#[derive(Serialize)]
pub struct CesAppAudioProcessingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inactivity_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient_sound_config: Option<Vec<CesAppAudioProcessingConfigElAmbientSoundConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    barge_in_config: Option<Vec<CesAppAudioProcessingConfigElBargeInConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesize_speech_configs: Option<Vec<CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl>>,
    dynamic: CesAppAudioProcessingConfigElDynamic,
}
impl CesAppAudioProcessingConfigEl {
    #[doc = "Set the field `inactivity_timeout`.\nThe duration of user inactivity (no speech or interaction) before the agent\nprompts the user for reengagement. If not set, the agent will not prompt\nthe user for reengagement."]
    pub fn set_inactivity_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inactivity_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `ambient_sound_config`.\n"]
    pub fn set_ambient_sound_config(
        mut self,
        v: impl Into<BlockAssignable<CesAppAudioProcessingConfigElAmbientSoundConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ambient_sound_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ambient_sound_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `barge_in_config`.\n"]
    pub fn set_barge_in_config(
        mut self,
        v: impl Into<BlockAssignable<CesAppAudioProcessingConfigElBargeInConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.barge_in_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.barge_in_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v: impl Into<BlockAssignable<CesAppAudioProcessingConfigElSynthesizeSpeechConfigsEl>>,
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
impl ToListMappable for CesAppAudioProcessingConfigEl {
    type O = BlockAssignable<CesAppAudioProcessingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppAudioProcessingConfigEl {}
impl BuildCesAppAudioProcessingConfigEl {
    pub fn build(self) -> CesAppAudioProcessingConfigEl {
        CesAppAudioProcessingConfigEl {
            inactivity_timeout: core::default::Default::default(),
            ambient_sound_config: core::default::Default::default(),
            barge_in_config: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppAudioProcessingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppAudioProcessingConfigElRef {
    fn new(shared: StackShared, base: String) -> CesAppAudioProcessingConfigElRef {
        CesAppAudioProcessingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppAudioProcessingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inactivity_timeout` after provisioning.\nThe duration of user inactivity (no speech or interaction) before the agent\nprompts the user for reengagement. If not set, the agent will not prompt\nthe user for reengagement."]
    pub fn inactivity_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactivity_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ambient_sound_config` after provisioning.\n"]
    pub fn ambient_sound_config(
        &self,
    ) -> ListRef<CesAppAudioProcessingConfigElAmbientSoundConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ambient_sound_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `barge_in_config` after provisioning.\n"]
    pub fn barge_in_config(&self) -> ListRef<CesAppAudioProcessingConfigElBargeInConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.barge_in_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppClientCertificateSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase: Option<PrimField<String>>,
    private_key: PrimField<String>,
    tls_certificate: PrimField<String>,
}
impl CesAppClientCertificateSettingsEl {
    #[doc = "Set the field `passphrase`.\nThe passphrase to decrypt the private key.\nShould be left unset if the private key is not encrypted."]
    pub fn set_passphrase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passphrase = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppClientCertificateSettingsEl {
    type O = BlockAssignable<CesAppClientCertificateSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppClientCertificateSettingsEl {
    #[doc = "The name of the SecretManager secret version resource\nstoring the private key encoded in PEM format.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub private_key: PrimField<String>,
    #[doc = "The TLS certificate encoded in PEM format.\nThis string must include the begin header and end footer lines."]
    pub tls_certificate: PrimField<String>,
}
impl BuildCesAppClientCertificateSettingsEl {
    pub fn build(self) -> CesAppClientCertificateSettingsEl {
        CesAppClientCertificateSettingsEl {
            passphrase: core::default::Default::default(),
            private_key: self.private_key,
            tls_certificate: self.tls_certificate,
        }
    }
}
pub struct CesAppClientCertificateSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppClientCertificateSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppClientCertificateSettingsElRef {
        CesAppClientCertificateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppClientCertificateSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\nThe passphrase to decrypt the private key.\nShould be left unset if the private key is not encrypted."]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nThe name of the SecretManager secret version resource\nstoring the private key encoded in PEM format.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
    #[doc = "Get a reference to the value of field `tls_certificate` after provisioning.\nThe TLS certificate encoded in PEM format.\nThis string must include the begin header and end footer lines."]
    pub fn tls_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppDataStoreSettingsElEnginesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CesAppDataStoreSettingsElEnginesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppDataStoreSettingsElEnginesEl {
    type O = BlockAssignable<CesAppDataStoreSettingsElEnginesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppDataStoreSettingsElEnginesEl {}
impl BuildCesAppDataStoreSettingsElEnginesEl {
    pub fn build(self) -> CesAppDataStoreSettingsElEnginesEl {
        CesAppDataStoreSettingsElEnginesEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CesAppDataStoreSettingsElEnginesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppDataStoreSettingsElEnginesElRef {
    fn new(shared: StackShared, base: String) -> CesAppDataStoreSettingsElEnginesElRef {
        CesAppDataStoreSettingsElEnginesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppDataStoreSettingsElEnginesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppDataStoreSettingsEl {}
impl CesAppDataStoreSettingsEl {}
impl ToListMappable for CesAppDataStoreSettingsEl {
    type O = BlockAssignable<CesAppDataStoreSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppDataStoreSettingsEl {}
impl BuildCesAppDataStoreSettingsEl {
    pub fn build(self) -> CesAppDataStoreSettingsEl {
        CesAppDataStoreSettingsEl {}
    }
}
pub struct CesAppDataStoreSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppDataStoreSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppDataStoreSettingsElRef {
        CesAppDataStoreSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppDataStoreSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engines` after provisioning.\nThe engines for the app."]
    pub fn engines(&self) -> ListRef<CesAppDataStoreSettingsElEnginesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.engines", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppDefaultChannelProfileElPersonaPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persona: Option<PrimField<String>>,
}
impl CesAppDefaultChannelProfileElPersonaPropertyEl {
    #[doc = "Set the field `persona`.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn set_persona(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persona = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppDefaultChannelProfileElPersonaPropertyEl {
    type O = BlockAssignable<CesAppDefaultChannelProfileElPersonaPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppDefaultChannelProfileElPersonaPropertyEl {}
impl BuildCesAppDefaultChannelProfileElPersonaPropertyEl {
    pub fn build(self) -> CesAppDefaultChannelProfileElPersonaPropertyEl {
        CesAppDefaultChannelProfileElPersonaPropertyEl {
            persona: core::default::Default::default(),
        }
    }
}
pub struct CesAppDefaultChannelProfileElPersonaPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppDefaultChannelProfileElPersonaPropertyElRef {
    fn new(shared: StackShared, base: String) -> CesAppDefaultChannelProfileElPersonaPropertyElRef {
        CesAppDefaultChannelProfileElPersonaPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppDefaultChannelProfileElPersonaPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persona` after provisioning.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn persona(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppDefaultChannelProfileElWebWidgetConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    modality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_title: Option<PrimField<String>>,
}
impl CesAppDefaultChannelProfileElWebWidgetConfigEl {
    #[doc = "Set the field `modality`.\nThe modality of the web widget.\nPossible values:\nUNKNOWN_MODALITY\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY"]
    pub fn set_modality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.modality = Some(v.into());
        self
    }
    #[doc = "Set the field `theme`.\nThe theme of the web widget.\nPossible values:\nUNKNOWN_THEME\nLIGHT\nDARK"]
    pub fn set_theme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.theme = Some(v.into());
        self
    }
    #[doc = "Set the field `web_widget_title`.\nThe title of the web widget."]
    pub fn set_web_widget_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_widget_title = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppDefaultChannelProfileElWebWidgetConfigEl {
    type O = BlockAssignable<CesAppDefaultChannelProfileElWebWidgetConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppDefaultChannelProfileElWebWidgetConfigEl {}
impl BuildCesAppDefaultChannelProfileElWebWidgetConfigEl {
    pub fn build(self) -> CesAppDefaultChannelProfileElWebWidgetConfigEl {
        CesAppDefaultChannelProfileElWebWidgetConfigEl {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
        }
    }
}
pub struct CesAppDefaultChannelProfileElWebWidgetConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppDefaultChannelProfileElWebWidgetConfigElRef {
    fn new(shared: StackShared, base: String) -> CesAppDefaultChannelProfileElWebWidgetConfigElRef {
        CesAppDefaultChannelProfileElWebWidgetConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppDefaultChannelProfileElWebWidgetConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `modality` after provisioning.\nThe modality of the web widget.\nPossible values:\nUNKNOWN_MODALITY\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY"]
    pub fn modality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }
    #[doc = "Get a reference to the value of field `theme` after provisioning.\nThe theme of the web widget.\nPossible values:\nUNKNOWN_THEME\nLIGHT\nDARK"]
    pub fn theme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.theme", self.base))
    }
    #[doc = "Get a reference to the value of field `web_widget_title` after provisioning.\nThe title of the web widget."]
    pub fn web_widget_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_widget_title", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesAppDefaultChannelProfileElDynamic {
    persona_property: Option<DynamicBlock<CesAppDefaultChannelProfileElPersonaPropertyEl>>,
    web_widget_config: Option<DynamicBlock<CesAppDefaultChannelProfileElWebWidgetConfigEl>>,
}
#[derive(Serialize)]
pub struct CesAppDefaultChannelProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_barge_in_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_dtmf: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persona_property: Option<Vec<CesAppDefaultChannelProfileElPersonaPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_config: Option<Vec<CesAppDefaultChannelProfileElWebWidgetConfigEl>>,
    dynamic: CesAppDefaultChannelProfileElDynamic,
}
impl CesAppDefaultChannelProfileEl {
    #[doc = "Set the field `channel_type`.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE"]
    pub fn set_channel_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel_type = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_barge_in_control`.\nWhether to disable user barge-in in the conversation.\n- true: User interruptions are disabled while the agent is speaking.\n- false: The agent retains automatic control over when the user can interrupt."]
    pub fn set_disable_barge_in_control(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_barge_in_control = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_dtmf`.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn set_disable_dtmf(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_dtmf = Some(v.into());
        self
    }
    #[doc = "Set the field `profile_id`.\nThe unique identifier of the channel profile."]
    pub fn set_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `persona_property`.\n"]
    pub fn set_persona_property(
        mut self,
        v: impl Into<BlockAssignable<CesAppDefaultChannelProfileElPersonaPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.persona_property = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.persona_property = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_widget_config`.\n"]
    pub fn set_web_widget_config(
        mut self,
        v: impl Into<BlockAssignable<CesAppDefaultChannelProfileElWebWidgetConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_widget_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_widget_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesAppDefaultChannelProfileEl {
    type O = BlockAssignable<CesAppDefaultChannelProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppDefaultChannelProfileEl {}
impl BuildCesAppDefaultChannelProfileEl {
    pub fn build(self) -> CesAppDefaultChannelProfileEl {
        CesAppDefaultChannelProfileEl {
            channel_type: core::default::Default::default(),
            disable_barge_in_control: core::default::Default::default(),
            disable_dtmf: core::default::Default::default(),
            profile_id: core::default::Default::default(),
            persona_property: core::default::Default::default(),
            web_widget_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppDefaultChannelProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppDefaultChannelProfileElRef {
    fn new(shared: StackShared, base: String) -> CesAppDefaultChannelProfileElRef {
        CesAppDefaultChannelProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppDefaultChannelProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_type` after provisioning.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE"]
    pub fn channel_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_barge_in_control` after provisioning.\nWhether to disable user barge-in in the conversation.\n- true: User interruptions are disabled while the agent is speaking.\n- false: The agent retains automatic control over when the user can interrupt."]
    pub fn disable_barge_in_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_barge_in_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_dtmf` after provisioning.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn disable_dtmf(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_dtmf", self.base))
    }
    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\nThe unique identifier of the channel profile."]
    pub fn profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }
    #[doc = "Get a reference to the value of field `persona_property` after provisioning.\n"]
    pub fn persona_property(&self) -> ListRef<CesAppDefaultChannelProfileElPersonaPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persona_property", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(&self) -> ListRef<CesAppDefaultChannelProfileElWebWidgetConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_widget_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_invocation_parameter_correctness_threshold: Option<PrimField<f64>>,
}
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { # [doc = "Set the field `tool_invocation_parameter_correctness_threshold`.\nThe success threshold for individual tool invocation parameter\ncorrectness. Must be a float between 0 and 1. Default is 1.0."] pub fn set_tool_invocation_parameter_correctness_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . tool_invocation_parameter_correctness_threshold = Some (v . into ()) ; self } }
impl ToListMappable for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { type O = BlockAssignable < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl
{}
impl BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { pub fn build (self) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { tool_invocation_parameter_correctness_threshold : core :: default :: Default :: default () , } } }
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { fn new (shared : StackShared , base : String) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `tool_invocation_parameter_correctness_threshold` after provisioning.\nThe success threshold for individual tool invocation parameter\ncorrectness. Must be a float between 0 and 1. Default is 1.0."] pub fn tool_invocation_parameter_correctness_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.tool_invocation_parameter_correctness_threshold" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    overall_tool_invocation_correctness_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_similarity_success_threshold: Option<PrimField<f64>>,
}
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { # [doc = "Set the field `overall_tool_invocation_correctness_threshold`.\nThe success threshold for overall tool invocation correctness. Must be\na float between 0 and 1. Default is 1.0."] pub fn set_overall_tool_invocation_correctness_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . overall_tool_invocation_correctness_threshold = Some (v . into ()) ; self } # [doc = "Set the field `semantic_similarity_success_threshold`.\nThe success threshold for semantic similarity. Must be an integer\nbetween 0 and 4. Default is >= 3."] pub fn set_semantic_similarity_success_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . semantic_similarity_success_threshold = Some (v . into ()) ; self } }
impl ToListMappable for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { type O = BlockAssignable < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl
{}
impl BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { pub fn build (self) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { overall_tool_invocation_correctness_threshold : core :: default :: Default :: default () , semantic_similarity_success_threshold : core :: default :: Default :: default () , } } }
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { fn new (shared : StackShared , base : String) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `overall_tool_invocation_correctness_threshold` after provisioning.\nThe success threshold for overall tool invocation correctness. Must be\na float between 0 and 1. Default is 1.0."] pub fn overall_tool_invocation_correctness_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.overall_tool_invocation_correctness_threshold" , self . base)) } # [doc = "Get a reference to the value of field `semantic_similarity_success_threshold` after provisioning.\nThe success threshold for semantic similarity. Must be an integer\nbetween 0 and 4. Default is >= 3."] pub fn semantic_similarity_success_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.semantic_similarity_success_threshold" , self . base)) } }
#[derive(Serialize, Default)]
struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElDynamic { expectation_level_metrics_thresholds : Option < DynamicBlock < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl >> , turn_level_metrics_thresholds : Option < DynamicBlock < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl >> , }
#[derive(Serialize)]
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl { # [serde (skip_serializing_if = "Option::is_none")] expectation_level_metrics_thresholds : Option < Vec < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl > > , # [serde (skip_serializing_if = "Option::is_none")] turn_level_metrics_thresholds : Option < Vec < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl > > , dynamic : CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElDynamic , }
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
    #[doc = "Set the field `expectation_level_metrics_thresholds`.\n"]
    pub fn set_expectation_level_metrics_thresholds(
        mut self,
        v : impl Into < BlockAssignable < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expectation_level_metrics_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expectation_level_metrics_thresholds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `turn_level_metrics_thresholds`.\n"]
    pub fn set_turn_level_metrics_thresholds(
        mut self,
        v : impl Into < BlockAssignable < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.turn_level_metrics_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.turn_level_metrics_thresholds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
    type O =
        BlockAssignable<CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {}
impl BuildCesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
    pub fn build(self) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
        CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
            expectation_level_metrics_thresholds: core::default::Default::default(),
            turn_level_metrics_thresholds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef {
        CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expectation_level_metrics_thresholds` after provisioning.\n"]    pub fn expectation_level_metrics_thresholds (& self) -> ListRef < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.expectation_level_metrics_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `turn_level_metrics_thresholds` after provisioning.\n"]    pub fn turn_level_metrics_thresholds (& self) -> ListRef < CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.turn_level_metrics_thresholds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesAppEvaluationMetricsThresholdsElDynamic {
    golden_evaluation_metrics_thresholds: Option<
        DynamicBlock<CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl>,
    >,
}
#[derive(Serialize)]
pub struct CesAppEvaluationMetricsThresholdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    golden_evaluation_metrics_thresholds:
        Option<Vec<CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl>>,
    dynamic: CesAppEvaluationMetricsThresholdsElDynamic,
}
impl CesAppEvaluationMetricsThresholdsEl {
    #[doc = "Set the field `golden_evaluation_metrics_thresholds`.\n"]
    pub fn set_golden_evaluation_metrics_thresholds(
        mut self,
        v: impl Into<
            BlockAssignable<CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.golden_evaluation_metrics_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.golden_evaluation_metrics_thresholds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesAppEvaluationMetricsThresholdsEl {
    type O = BlockAssignable<CesAppEvaluationMetricsThresholdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppEvaluationMetricsThresholdsEl {}
impl BuildCesAppEvaluationMetricsThresholdsEl {
    pub fn build(self) -> CesAppEvaluationMetricsThresholdsEl {
        CesAppEvaluationMetricsThresholdsEl {
            golden_evaluation_metrics_thresholds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppEvaluationMetricsThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppEvaluationMetricsThresholdsElRef {
    fn new(shared: StackShared, base: String) -> CesAppEvaluationMetricsThresholdsElRef {
        CesAppEvaluationMetricsThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppEvaluationMetricsThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `golden_evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn golden_evaluation_metrics_thresholds(
        &self,
    ) -> ListRef<CesAppEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.golden_evaluation_metrics_thresholds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppLanguageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_multilingual_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_language_codes: Option<ListField<PrimField<String>>>,
}
impl CesAppLanguageSettingsEl {
    #[doc = "Set the field `default_language_code`.\nThe default language code of the app."]
    pub fn set_default_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_multilingual_support`.\nEnables multilingual support. If true, agents in the app will use pre-built\ninstructions to improve handling of multilingual input."]
    pub fn set_enable_multilingual_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_multilingual_support = Some(v.into());
        self
    }
    #[doc = "Set the field `fallback_action`.\nThe action to perform when an agent receives input in an unsupported\nlanguage.\nThis can be a predefined action or a custom tool call.\nValid values are:\n- A tool's full resource name, which triggers a specific tool execution.\n- A predefined system action, such as \"escalate\" or \"exit\", which triggers\nan EndSession signal with corresponding metadata\nto terminate the conversation."]
    pub fn set_fallback_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fallback_action = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_language_codes`.\nList of languages codes supported by the app, in addition to the\n'default_language_code'."]
    pub fn set_supported_language_codes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_language_codes = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLanguageSettingsEl {
    type O = BlockAssignable<CesAppLanguageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLanguageSettingsEl {}
impl BuildCesAppLanguageSettingsEl {
    pub fn build(self) -> CesAppLanguageSettingsEl {
        CesAppLanguageSettingsEl {
            default_language_code: core::default::Default::default(),
            enable_multilingual_support: core::default::Default::default(),
            fallback_action: core::default::Default::default(),
            supported_language_codes: core::default::Default::default(),
        }
    }
}
pub struct CesAppLanguageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLanguageSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppLanguageSettingsElRef {
        CesAppLanguageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLanguageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language code of the app."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multilingual_support` after provisioning.\nEnables multilingual support. If true, agents in the app will use pre-built\ninstructions to improve handling of multilingual input."]
    pub fn enable_multilingual_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multilingual_support", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_action` after provisioning.\nThe action to perform when an agent receives input in an unsupported\nlanguage.\nThis can be a predefined action or a custom tool call.\nValid values are:\n- A tool's full resource name, which triggers a specific tool execution.\n- A predefined system action, such as \"escalate\" or \"exit\", which triggers\nan EndSession signal with corresponding metadata\nto terminate the conversation."]
    pub fn fallback_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fallback_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\nList of languages codes supported by the app, in addition to the\n'default_language_code'."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsElAudioRecordingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_path_prefix: Option<PrimField<String>>,
}
impl CesAppLoggingSettingsElAudioRecordingConfigEl {
    #[doc = "Set the field `gcs_bucket`.\nThe [Cloud Storage](https://cloud.google.com/storage) bucket to store the\nsession audio recordings. The URI must start with \"gs://\".\nNote: If the Cloud Storage bucket is in a different project from the app,\nyou should grant 'storage.objects.create' permission to the CES service\nagent 'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn set_gcs_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_path_prefix`.\nThe Cloud Storage path prefix for audio recordings.\nThis prefix can include the following placeholders, which will be\ndynamically substituted at serving time:\n- $project:   project ID\n- $location:  app location\n- $app:       app ID\n- $date:      session date in YYYY-MM-DD format\n- $session:   session ID\nIf the path prefix is not specified, the default prefix\n'$project/$location/$app/$date/$session/' will be used."]
    pub fn set_gcs_path_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_path_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsElAudioRecordingConfigEl {
    type O = BlockAssignable<CesAppLoggingSettingsElAudioRecordingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsElAudioRecordingConfigEl {}
impl BuildCesAppLoggingSettingsElAudioRecordingConfigEl {
    pub fn build(self) -> CesAppLoggingSettingsElAudioRecordingConfigEl {
        CesAppLoggingSettingsElAudioRecordingConfigEl {
            gcs_bucket: core::default::Default::default(),
            gcs_path_prefix: core::default::Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElAudioRecordingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElAudioRecordingConfigElRef {
    fn new(shared: StackShared, base: String) -> CesAppLoggingSettingsElAudioRecordingConfigElRef {
        CesAppLoggingSettingsElAudioRecordingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElAudioRecordingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe [Cloud Storage](https://cloud.google.com/storage) bucket to store the\nsession audio recordings. The URI must start with \"gs://\".\nNote: If the Cloud Storage bucket is in a different project from the app,\nyou should grant 'storage.objects.create' permission to the CES service\nagent 'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_path_prefix` after provisioning.\nThe Cloud Storage path prefix for audio recordings.\nThis prefix can include the following placeholders, which will be\ndynamically substituted at serving time:\n- $project:   project ID\n- $location:  app location\n- $app:       app ID\n- $date:      session date in YYYY-MM-DD format\n- $session:   session ID\nIf the path prefix is not specified, the default prefix\n'$project/$location/$app/$date/$session/' will be used."]
    pub fn gcs_path_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_path_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsElBigqueryExportSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl CesAppLoggingSettingsElBigqueryExportSettingsEl {
    #[doc = "Set the field `dataset`.\nThe BigQuery dataset to export the data to."]
    pub fn set_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nIndicates whether the BigQuery export is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project ID of the BigQuery dataset to export the data to.\nNote: If the BigQuery dataset is in a different project from the app, you should grant\nroles/bigquery.admin role to the CES service agent service-<PROJECT-\nNUMBER>@gcp-sa-ces.iam.gserviceaccount.com."]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsElBigqueryExportSettingsEl {
    type O = BlockAssignable<CesAppLoggingSettingsElBigqueryExportSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsElBigqueryExportSettingsEl {}
impl BuildCesAppLoggingSettingsElBigqueryExportSettingsEl {
    pub fn build(self) -> CesAppLoggingSettingsElBigqueryExportSettingsEl {
        CesAppLoggingSettingsElBigqueryExportSettingsEl {
            dataset: core::default::Default::default(),
            enabled: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElBigqueryExportSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElBigqueryExportSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppLoggingSettingsElBigqueryExportSettingsElRef {
        CesAppLoggingSettingsElBigqueryExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElBigqueryExportSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nThe BigQuery dataset to export the data to."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether the BigQuery export is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID of the BigQuery dataset to export the data to.\nNote: If the BigQuery dataset is in a different project from the app, you should grant\nroles/bigquery.admin role to the CES service agent service-<PROJECT-\nNUMBER>@gcp-sa-ces.iam.gserviceaccount.com."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsElCloudLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cloud_logging: Option<PrimField<bool>>,
}
impl CesAppLoggingSettingsElCloudLoggingSettingsEl {
    #[doc = "Set the field `enable_cloud_logging`.\nWhether to enable Cloud Logging for the sessions."]
    pub fn set_enable_cloud_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_cloud_logging = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsElCloudLoggingSettingsEl {
    type O = BlockAssignable<CesAppLoggingSettingsElCloudLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsElCloudLoggingSettingsEl {}
impl BuildCesAppLoggingSettingsElCloudLoggingSettingsEl {
    pub fn build(self) -> CesAppLoggingSettingsElCloudLoggingSettingsEl {
        CesAppLoggingSettingsElCloudLoggingSettingsEl {
            enable_cloud_logging: core::default::Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElCloudLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElCloudLoggingSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppLoggingSettingsElCloudLoggingSettingsElRef {
        CesAppLoggingSettingsElCloudLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElCloudLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nWhether to enable Cloud Logging for the sessions."]
    pub fn enable_cloud_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cloud_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsElConversationLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_conversation_logging: Option<PrimField<bool>>,
}
impl CesAppLoggingSettingsElConversationLoggingSettingsEl {
    #[doc = "Set the field `disable_conversation_logging`.\nWhether to disable conversation logging for the sessions."]
    pub fn set_disable_conversation_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_conversation_logging = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsElConversationLoggingSettingsEl {
    type O = BlockAssignable<CesAppLoggingSettingsElConversationLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsElConversationLoggingSettingsEl {}
impl BuildCesAppLoggingSettingsElConversationLoggingSettingsEl {
    pub fn build(self) -> CesAppLoggingSettingsElConversationLoggingSettingsEl {
        CesAppLoggingSettingsElConversationLoggingSettingsEl {
            disable_conversation_logging: core::default::Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElConversationLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElConversationLoggingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppLoggingSettingsElConversationLoggingSettingsElRef {
        CesAppLoggingSettingsElConversationLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElConversationLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_conversation_logging` after provisioning.\nWhether to disable conversation logging for the sessions."]
    pub fn disable_conversation_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_conversation_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsElRedactionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_template: Option<PrimField<String>>,
}
impl CesAppLoggingSettingsElRedactionConfigEl {
    #[doc = "Set the field `deidentify_template`.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name to\ninstruct on how to de-identify content.\nFormat:\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deidentify_template = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_redaction`.\nIf true, redaction will be applied in various logging scenarios, including\nconversation history, Cloud Logging and audio recording."]
    pub fn set_enable_redaction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_redaction = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template`.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name to configure\ndetection of sensitive data types.\nFormat:\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inspect_template = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsElRedactionConfigEl {
    type O = BlockAssignable<CesAppLoggingSettingsElRedactionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsElRedactionConfigEl {}
impl BuildCesAppLoggingSettingsElRedactionConfigEl {
    pub fn build(self) -> CesAppLoggingSettingsElRedactionConfigEl {
        CesAppLoggingSettingsElRedactionConfigEl {
            deidentify_template: core::default::Default::default(),
            enable_redaction: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElRedactionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElRedactionConfigElRef {
    fn new(shared: StackShared, base: String) -> CesAppLoggingSettingsElRedactionConfigElRef {
        CesAppLoggingSettingsElRedactionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElRedactionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name to\ninstruct on how to de-identify content.\nFormat:\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_redaction` after provisioning.\nIf true, redaction will be applied in various logging scenarios, including\nconversation history, Cloud Logging and audio recording."]
    pub fn enable_redaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_redaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name to configure\ndetection of sensitive data types.\nFormat:\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesAppLoggingSettingsElDynamic {
    audio_recording_config: Option<DynamicBlock<CesAppLoggingSettingsElAudioRecordingConfigEl>>,
    bigquery_export_settings: Option<DynamicBlock<CesAppLoggingSettingsElBigqueryExportSettingsEl>>,
    cloud_logging_settings: Option<DynamicBlock<CesAppLoggingSettingsElCloudLoggingSettingsEl>>,
    conversation_logging_settings:
        Option<DynamicBlock<CesAppLoggingSettingsElConversationLoggingSettingsEl>>,
    redaction_config: Option<DynamicBlock<CesAppLoggingSettingsElRedactionConfigEl>>,
}
#[derive(Serialize)]
pub struct CesAppLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_recording_config: Option<Vec<CesAppLoggingSettingsElAudioRecordingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_export_settings: Option<Vec<CesAppLoggingSettingsElBigqueryExportSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_logging_settings: Option<Vec<CesAppLoggingSettingsElCloudLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_logging_settings:
        Option<Vec<CesAppLoggingSettingsElConversationLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redaction_config: Option<Vec<CesAppLoggingSettingsElRedactionConfigEl>>,
    dynamic: CesAppLoggingSettingsElDynamic,
}
impl CesAppLoggingSettingsEl {
    #[doc = "Set the field `audio_recording_config`.\n"]
    pub fn set_audio_recording_config(
        mut self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsElAudioRecordingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_recording_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_recording_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bigquery_export_settings`.\n"]
    pub fn set_bigquery_export_settings(
        mut self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsElBigqueryExportSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_export_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_logging_settings`.\n"]
    pub fn set_cloud_logging_settings(
        mut self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsElCloudLoggingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_logging_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_logging_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `conversation_logging_settings`.\n"]
    pub fn set_conversation_logging_settings(
        mut self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsElConversationLoggingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conversation_logging_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_logging_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redaction_config`.\n"]
    pub fn set_redaction_config(
        mut self,
        v: impl Into<BlockAssignable<CesAppLoggingSettingsElRedactionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redaction_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redaction_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesAppLoggingSettingsEl {
    type O = BlockAssignable<CesAppLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppLoggingSettingsEl {}
impl BuildCesAppLoggingSettingsEl {
    pub fn build(self) -> CesAppLoggingSettingsEl {
        CesAppLoggingSettingsEl {
            audio_recording_config: core::default::Default::default(),
            bigquery_export_settings: core::default::Default::default(),
            cloud_logging_settings: core::default::Default::default(),
            conversation_logging_settings: core::default::Default::default(),
            redaction_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppLoggingSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppLoggingSettingsElRef {
        CesAppLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_recording_config` after provisioning.\n"]
    pub fn audio_recording_config(
        &self,
    ) -> ListRef<CesAppLoggingSettingsElAudioRecordingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_recording_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_export_settings` after provisioning.\n"]
    pub fn bigquery_export_settings(
        &self,
    ) -> ListRef<CesAppLoggingSettingsElBigqueryExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_export_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_logging_settings` after provisioning.\n"]
    pub fn cloud_logging_settings(
        &self,
    ) -> ListRef<CesAppLoggingSettingsElCloudLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conversation_logging_settings` after provisioning.\n"]
    pub fn conversation_logging_settings(
        &self,
    ) -> ListRef<CesAppLoggingSettingsElConversationLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_config` after provisioning.\n"]
    pub fn redaction_config(&self) -> ListRef<CesAppLoggingSettingsElRedactionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redaction_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppModelSettingsEl {
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
impl ToListMappable for CesAppModelSettingsEl {
    type O = BlockAssignable<CesAppModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppModelSettingsEl {}
impl BuildCesAppModelSettingsEl {
    pub fn build(self) -> CesAppModelSettingsEl {
        CesAppModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAppModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppModelSettingsElRef {
        CesAppModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppModelSettingsElRef {
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
pub struct CesAppTimeZoneSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl CesAppTimeZoneSettingsEl {
    #[doc = "Set the field `time_zone`.\nThe time zone of the app from the time zone database, e.g., America/Los_Angeles, Europe/Paris."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppTimeZoneSettingsEl {
    type O = BlockAssignable<CesAppTimeZoneSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppTimeZoneSettingsEl {}
impl BuildCesAppTimeZoneSettingsEl {
    pub fn build(self) -> CesAppTimeZoneSettingsEl {
        CesAppTimeZoneSettingsEl {
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct CesAppTimeZoneSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppTimeZoneSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppTimeZoneSettingsElRef {
        CesAppTimeZoneSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppTimeZoneSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the app from the time zone database, e.g., America/Los_Angeles, Europe/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesAppTimeoutsEl {
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
impl ToListMappable for CesAppTimeoutsEl {
    type O = BlockAssignable<CesAppTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppTimeoutsEl {}
impl BuildCesAppTimeoutsEl {
    pub fn build(self) -> CesAppTimeoutsEl {
        CesAppTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesAppTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesAppTimeoutsElRef {
        CesAppTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppTimeoutsElRef {
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
pub struct CesAppVariableDeclarationsElSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesAppVariableDeclarationsElSchemaEl {
    #[doc = "Set the field `additional_properties`.\nOptional. Defines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\nOptional. The instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\nOptional. Default value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be encoded as a JSON string.\nUse 'jsonencode' in Terraform HCL to encode the default value."]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\nOptional. Schemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVariableDeclarationsElSchemaEl {
    type O = BlockAssignable<CesAppVariableDeclarationsElSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVariableDeclarationsElSchemaEl {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: PrimField<String>,
}
impl BuildCesAppVariableDeclarationsElSchemaEl {
    pub fn build(self) -> CesAppVariableDeclarationsElSchemaEl {
        CesAppVariableDeclarationsElSchemaEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesAppVariableDeclarationsElSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVariableDeclarationsElSchemaElRef {
    fn new(shared: StackShared, base: String) -> CesAppVariableDeclarationsElSchemaElRef {
        CesAppVariableDeclarationsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVariableDeclarationsElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\nOptional. Defines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\nOptional. The instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\nOptional. Default value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be encoded as a JSON string.\nUse 'jsonencode' in Terraform HCL to encode the default value."]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\nOptional. Schemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesAppVariableDeclarationsElDynamic {
    schema: Option<DynamicBlock<CesAppVariableDeclarationsElSchemaEl>>,
}
#[derive(Serialize)]
pub struct CesAppVariableDeclarationsEl {
    description: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<CesAppVariableDeclarationsElSchemaEl>>,
    dynamic: CesAppVariableDeclarationsElDynamic,
}
impl CesAppVariableDeclarationsEl {
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<BlockAssignable<CesAppVariableDeclarationsElSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesAppVariableDeclarationsEl {
    type O = BlockAssignable<CesAppVariableDeclarationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVariableDeclarationsEl {
    #[doc = "The description of the variable."]
    pub description: PrimField<String>,
    #[doc = "The name of the variable. The name must start with a letter or underscore\nand contain only letters, numbers, or underscores."]
    pub name: PrimField<String>,
}
impl BuildCesAppVariableDeclarationsEl {
    pub fn build(self) -> CesAppVariableDeclarationsEl {
        CesAppVariableDeclarationsEl {
            description: self.description,
            name: self.name,
            schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesAppVariableDeclarationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVariableDeclarationsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVariableDeclarationsElRef {
        CesAppVariableDeclarationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVariableDeclarationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the variable."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the variable. The name must start with a letter or underscore\nand contain only letters, numbers, or underscores."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<CesAppVariableDeclarationsElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesAppDynamic {
    audio_processing_config: Option<DynamicBlock<CesAppAudioProcessingConfigEl>>,
    client_certificate_settings: Option<DynamicBlock<CesAppClientCertificateSettingsEl>>,
    data_store_settings: Option<DynamicBlock<CesAppDataStoreSettingsEl>>,
    default_channel_profile: Option<DynamicBlock<CesAppDefaultChannelProfileEl>>,
    evaluation_metrics_thresholds: Option<DynamicBlock<CesAppEvaluationMetricsThresholdsEl>>,
    language_settings: Option<DynamicBlock<CesAppLanguageSettingsEl>>,
    logging_settings: Option<DynamicBlock<CesAppLoggingSettingsEl>>,
    model_settings: Option<DynamicBlock<CesAppModelSettingsEl>>,
    time_zone_settings: Option<DynamicBlock<CesAppTimeZoneSettingsEl>>,
    variable_declarations: Option<DynamicBlock<CesAppVariableDeclarationsEl>>,
}
