use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApihubCurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    curation_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<Vec<ApihubCurationEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApihubCurationTimeoutsEl>,
    dynamic: ApihubCurationDynamic,
}
struct ApihubCuration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApihubCurationData>,
}
#[derive(Clone)]
pub struct ApihubCuration(Rc<ApihubCuration_>);
impl ApihubCuration {
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
    #[doc = "Set the field `description`.\nThe description of the curation."]
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
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(self, v: impl Into<BlockAssignable<ApihubCurationEndpointEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApihubCurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the curation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `curation_id` after provisioning.\nThe ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn curation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.curation_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the curation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the curation."]
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
    #[doc = "Get a reference to the value of field `last_execution_error_code` after provisioning.\nThe error code of the last execution of the curation. The error code is\npopulated only when the last execution state is failed.\nPossible values:\nERROR_CODE_UNSPECIFIED\nINTERNAL_ERROR\nUNAUTHORIZED"]
    pub fn last_execution_error_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_error_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_execution_error_message` after provisioning.\nError message describing the failure, if any, during the last execution of\nthe curation."]
    pub fn last_execution_error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_error_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_execution_state` after provisioning.\nThe last execution state of the curation.\nPossible values:\nLAST_EXECUTION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn last_execution_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the curation.\n\nFormat:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_instance_actions` after provisioning.\nThe plugin instances and associated actions that are using the curation.\nNote: A particular curation could be used by multiple plugin instances or\nmultiple actions in a plugin instance."]
    pub fn plugin_instance_actions(&self) -> ListRef<ApihubCurationPluginInstanceActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.plugin_instance_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the curation was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<ApihubCurationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubCurationTimeoutsElRef {
        ApihubCurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApihubCuration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApihubCuration {}
impl ToListMappable for ApihubCuration {
    type O = ListRef<ApihubCurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApihubCuration_ {
    fn extract_resource_type(&self) -> String {
        "google_apihub_curation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApihubCuration {
    pub tf_id: String,
    #[doc = "The ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub curation_id: PrimField<String>,
    #[doc = "The display name of the curation."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildApihubCuration {
    pub fn build(self, stack: &mut Stack) -> ApihubCuration {
        let out = ApihubCuration(Rc::new(ApihubCuration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApihubCurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                curation_id: self.curation_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApihubCurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubCurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApihubCurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the curation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `curation_id` after provisioning.\nThe ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn curation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.curation_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the curation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the curation."]
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
    #[doc = "Get a reference to the value of field `last_execution_error_code` after provisioning.\nThe error code of the last execution of the curation. The error code is\npopulated only when the last execution state is failed.\nPossible values:\nERROR_CODE_UNSPECIFIED\nINTERNAL_ERROR\nUNAUTHORIZED"]
    pub fn last_execution_error_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_error_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_execution_error_message` after provisioning.\nError message describing the failure, if any, during the last execution of\nthe curation."]
    pub fn last_execution_error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_error_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_execution_state` after provisioning.\nThe last execution state of the curation.\nPossible values:\nLAST_EXECUTION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn last_execution_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execution_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the curation.\n\nFormat:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_instance_actions` after provisioning.\nThe plugin instances and associated actions that are using the curation.\nNote: A particular curation could be used by multiple plugin instances or\nmultiple actions in a plugin instance."]
    pub fn plugin_instance_actions(&self) -> ListRef<ApihubCurationPluginInstanceActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.plugin_instance_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the curation was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<ApihubCurationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubCurationTimeoutsElRef {
        ApihubCurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubCurationPluginInstanceActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_instance: Option<PrimField<String>>,
}
impl ApihubCurationPluginInstanceActionsEl {
    #[doc = "Set the field `action_id`.\n"]
    pub fn set_action_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_id = Some(v.into());
        self
    }
    #[doc = "Set the field `plugin_instance`.\n"]
    pub fn set_plugin_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.plugin_instance = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubCurationPluginInstanceActionsEl {
    type O = BlockAssignable<ApihubCurationPluginInstanceActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubCurationPluginInstanceActionsEl {}
impl BuildApihubCurationPluginInstanceActionsEl {
    pub fn build(self) -> ApihubCurationPluginInstanceActionsEl {
        ApihubCurationPluginInstanceActionsEl {
            action_id: core::default::Default::default(),
            plugin_instance: core::default::Default::default(),
        }
    }
}
pub struct ApihubCurationPluginInstanceActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubCurationPluginInstanceActionsElRef {
    fn new(shared: StackShared, base: String) -> ApihubCurationPluginInstanceActionsElRef {
        ApihubCurationPluginInstanceActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubCurationPluginInstanceActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }
    #[doc = "Get a reference to the value of field `plugin_instance` after provisioning.\n"]
    pub fn plugin_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
    trigger_id: PrimField<String>,
    uri: PrimField<String>,
}
impl ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {}
impl ToListMappable for ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
    type O = BlockAssignable<ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
    #[doc = "The API trigger ID of the Application Integration workflow."]
    pub trigger_id: PrimField<String>,
    #[doc = "The endpoint URI should be a valid REST URI for triggering an Application\nIntegration.\nFormat:\n'https://integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'\nor\n'https://{location}-integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'"]
    pub uri: PrimField<String>,
}
impl BuildApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
    pub fn build(self) -> ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
        ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl {
            trigger_id: self.trigger_id,
            uri: self.uri,
        }
    }
}
pub struct ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef {
        ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trigger_id` after provisioning.\nThe API trigger ID of the Application Integration workflow."]
    pub fn trigger_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe endpoint URI should be a valid REST URI for triggering an Application\nIntegration.\nFormat:\n'https://integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'\nor\n'https://{location}-integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApihubCurationEndpointElDynamic {
    application_integration_endpoint_details:
        Option<DynamicBlock<ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl>>,
}
#[derive(Serialize)]
pub struct ApihubCurationEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_integration_endpoint_details:
        Option<Vec<ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl>>,
    dynamic: ApihubCurationEndpointElDynamic,
}
impl ApihubCurationEndpointEl {
    #[doc = "Set the field `application_integration_endpoint_details`.\n"]
    pub fn set_application_integration_endpoint_details(
        mut self,
        v: impl Into<BlockAssignable<ApihubCurationEndpointElApplicationIntegrationEndpointDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.application_integration_endpoint_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.application_integration_endpoint_details = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubCurationEndpointEl {
    type O = BlockAssignable<ApihubCurationEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubCurationEndpointEl {}
impl BuildApihubCurationEndpointEl {
    pub fn build(self) -> ApihubCurationEndpointEl {
        ApihubCurationEndpointEl {
            application_integration_endpoint_details: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubCurationEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubCurationEndpointElRef {
    fn new(shared: StackShared, base: String) -> ApihubCurationEndpointElRef {
        ApihubCurationEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubCurationEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_integration_endpoint_details` after provisioning.\n"]
    pub fn application_integration_endpoint_details(
        &self,
    ) -> ListRef<ApihubCurationEndpointElApplicationIntegrationEndpointDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_integration_endpoint_details", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubCurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApihubCurationTimeoutsEl {
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
impl ToListMappable for ApihubCurationTimeoutsEl {
    type O = BlockAssignable<ApihubCurationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubCurationTimeoutsEl {}
impl BuildApihubCurationTimeoutsEl {
    pub fn build(self) -> ApihubCurationTimeoutsEl {
        ApihubCurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApihubCurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubCurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApihubCurationTimeoutsElRef {
        ApihubCurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubCurationTimeoutsElRef {
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
struct ApihubCurationDynamic {
    endpoint: Option<DynamicBlock<ApihubCurationEndpointEl>>,
}
