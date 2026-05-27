use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApihubPluginInstanceData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    disable: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    plugin: PrimField<String>,
    plugin_instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<ApihubPluginInstanceActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_config: Option<Vec<ApihubPluginInstanceAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApihubPluginInstanceTimeoutsEl>,
    dynamic: ApihubPluginInstanceDynamic,
}
struct ApihubPluginInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApihubPluginInstanceData>,
}
#[derive(Clone)]
pub struct ApihubPluginInstance(Rc<ApihubPluginInstance_>);
impl ApihubPluginInstance {
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
    #[doc = "Set the field `disable`.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn set_disable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable = Some(v.into());
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
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(self, v: impl Into<BlockAssignable<ApihubPluginInstanceActionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApihubPluginInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin instance was created."]
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
    #[doc = "Get a reference to the value of field `disable` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn disable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\nError message describing the failure, if any, during Create, Delete or\nApplyConfig operation corresponding to the plugin instance.This field will\nonly be populated if the plugin instance is in the ERROR or FAILED state."]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name of the plugin instance resource.\nFormat:\n'projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_instance_id` after provisioning.\nThe ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn plugin_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin instance (e.g., enabled, disabled,\nprovisioning).\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nACTIVE\nAPPLYING_CONFIG\nERROR\nFAILED\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin instance was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<ApihubPluginInstanceActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> ListRef<ApihubPluginInstanceAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginInstanceTimeoutsElRef {
        ApihubPluginInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApihubPluginInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApihubPluginInstance {}
impl ToListMappable for ApihubPluginInstance {
    type O = ListRef<ApihubPluginInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApihubPluginInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_apihub_plugin_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApihubPluginInstance {
    pub tf_id: String,
    #[doc = "The display name for this plugin instance. Max length is 255 characters."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub plugin: PrimField<String>,
    #[doc = "The ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub plugin_instance_id: PrimField<String>,
}
impl BuildApihubPluginInstance {
    pub fn build(self, stack: &mut Stack) -> ApihubPluginInstance {
        let out = ApihubPluginInstance(Rc::new(ApihubPluginInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApihubPluginInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                disable: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                plugin: self.plugin,
                plugin_instance_id: self.plugin_instance_id,
                project: core::default::Default::default(),
                actions: core::default::Default::default(),
                auth_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApihubPluginInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApihubPluginInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin instance was created."]
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
    #[doc = "Get a reference to the value of field `disable` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn disable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\nError message describing the failure, if any, during Create, Delete or\nApplyConfig operation corresponding to the plugin instance.This field will\nonly be populated if the plugin instance is in the ERROR or FAILED state."]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name of the plugin instance resource.\nFormat:\n'projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_instance_id` after provisioning.\nThe ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn plugin_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin instance (e.g., enabled, disabled,\nprovisioning).\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nACTIVE\nAPPLYING_CONFIG\nERROR\nFAILED\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin instance was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<ApihubPluginInstanceActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> ListRef<ApihubPluginInstanceAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginInstanceTimeoutsElRef {
        ApihubPluginInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_message = Some(v.into());
        self
    }
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
    type O = BlockAssignable<ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {}
impl BuildApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
    pub fn build(self) -> ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
        ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl {
            end_time: core::default::Default::default(),
            error_message: core::default::Default::default(),
            result: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef {
        ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceActionsElHubInstanceActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_execution_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_execution:
        Option<ListField<ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl>>,
}
impl ApihubPluginInstanceActionsElHubInstanceActionEl {
    #[doc = "Set the field `current_execution_state`.\n"]
    pub fn set_current_execution_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.current_execution_state = Some(v.into());
        self
    }
    #[doc = "Set the field `last_execution`.\n"]
    pub fn set_last_execution(
        mut self,
        v: impl Into<ListField<ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionEl>>,
    ) -> Self {
        self.last_execution = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginInstanceActionsElHubInstanceActionEl {
    type O = BlockAssignable<ApihubPluginInstanceActionsElHubInstanceActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceActionsElHubInstanceActionEl {}
impl BuildApihubPluginInstanceActionsElHubInstanceActionEl {
    pub fn build(self) -> ApihubPluginInstanceActionsElHubInstanceActionEl {
        ApihubPluginInstanceActionsElHubInstanceActionEl {
            current_execution_state: core::default::Default::default(),
            last_execution: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceActionsElHubInstanceActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceActionsElHubInstanceActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceActionsElHubInstanceActionElRef {
        ApihubPluginInstanceActionsElHubInstanceActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceActionsElHubInstanceActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_execution_state` after provisioning.\n"]
    pub fn current_execution_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_execution_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_execution` after provisioning.\n"]
    pub fn last_execution(
        &self,
    ) -> ListRef<ApihubPluginInstanceActionsElHubInstanceActionElLastExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_execution", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
    curation: PrimField<String>,
}
impl ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {}
impl ToListMappable for ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
    type O = BlockAssignable<ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
    #[doc = "The unique name of the curation resource. This will be the name of the\ncuration resource in the format:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub curation: PrimField<String>,
}
impl BuildApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
    pub fn build(self) -> ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
        ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl {
            curation: self.curation,
        }
    }
}
pub struct ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef {
        ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `curation` after provisioning.\nThe unique name of the curation resource. This will be the name of the\ncuration resource in the format:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn curation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.curation", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceActionsElCurationConfigElDynamic {
    custom_curation:
        Option<DynamicBlock<ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceActionsElCurationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    curation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_curation: Option<Vec<ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl>>,
    dynamic: ApihubPluginInstanceActionsElCurationConfigElDynamic,
}
impl ApihubPluginInstanceActionsElCurationConfigEl {
    #[doc = "Set the field `curation_type`.\nPossible values:\nCURATION_TYPE_UNSPECIFIED\nDEFAULT_CURATION_FOR_API_METADATA\nCUSTOM_CURATION_FOR_API_METADATA"]
    pub fn set_curation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.curation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_curation`.\n"]
    pub fn set_custom_curation(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceActionsElCurationConfigElCustomCurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_curation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_curation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceActionsElCurationConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceActionsElCurationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceActionsElCurationConfigEl {}
impl BuildApihubPluginInstanceActionsElCurationConfigEl {
    pub fn build(self) -> ApihubPluginInstanceActionsElCurationConfigEl {
        ApihubPluginInstanceActionsElCurationConfigEl {
            curation_type: core::default::Default::default(),
            custom_curation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceActionsElCurationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceActionsElCurationConfigElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginInstanceActionsElCurationConfigElRef {
        ApihubPluginInstanceActionsElCurationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceActionsElCurationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `curation_type` after provisioning.\nPossible values:\nCURATION_TYPE_UNSPECIFIED\nDEFAULT_CURATION_FOR_API_METADATA\nCUSTOM_CURATION_FOR_API_METADATA"]
    pub fn curation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.curation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_curation` after provisioning.\n"]
    pub fn custom_curation(
        &self,
    ) -> ListRef<ApihubPluginInstanceActionsElCurationConfigElCustomCurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_curation", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceActionsElDynamic {
    curation_config: Option<DynamicBlock<ApihubPluginInstanceActionsElCurationConfigEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceActionsEl {
    action_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_cron_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    curation_config: Option<Vec<ApihubPluginInstanceActionsElCurationConfigEl>>,
    dynamic: ApihubPluginInstanceActionsElDynamic,
}
impl ApihubPluginInstanceActionsEl {
    #[doc = "Set the field `schedule_cron_expression`.\nThe schedule for this plugin instance action. This can only be set if the\nplugin supports API_HUB_SCHEDULE_TRIGGER mode for this action."]
    pub fn set_schedule_cron_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_cron_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_time_zone`.\nThe time zone for the schedule cron expression. If not provided, UTC will\nbe used."]
    pub fn set_schedule_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_time_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `curation_config`.\n"]
    pub fn set_curation_config(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceActionsElCurationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.curation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.curation_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceActionsEl {
    type O = BlockAssignable<ApihubPluginInstanceActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceActionsEl {
    #[doc = "This should map to one of the action id specified\nin actions_config in the plugin."]
    pub action_id: PrimField<String>,
}
impl BuildApihubPluginInstanceActionsEl {
    pub fn build(self) -> ApihubPluginInstanceActionsEl {
        ApihubPluginInstanceActionsEl {
            action_id: self.action_id,
            schedule_cron_expression: core::default::Default::default(),
            schedule_time_zone: core::default::Default::default(),
            curation_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceActionsElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginInstanceActionsElRef {
        ApihubPluginInstanceActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_id` after provisioning.\nThis should map to one of the action id specified\nin actions_config in the plugin."]
    pub fn action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }
    #[doc = "Get a reference to the value of field `hub_instance_action` after provisioning.\nThe execution status for the plugin instance."]
    pub fn hub_instance_action(
        &self,
    ) -> ListRef<ApihubPluginInstanceActionsElHubInstanceActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hub_instance_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_cron_expression` after provisioning.\nThe schedule for this plugin instance action. This can only be set if the\nplugin supports API_HUB_SCHEDULE_TRIGGER mode for this action."]
    pub fn schedule_cron_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_cron_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_time_zone` after provisioning.\nThe time zone for the schedule cron expression. If not provided, UTC will\nbe used."]
    pub fn schedule_time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_time_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin action in the plugin instance.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED\nENABLING\nDISABLING\nERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `curation_config` after provisioning.\n"]
    pub fn curation_config(&self) -> ListRef<ApihubPluginInstanceActionsElCurationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.curation_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
    secret_version: PrimField<String>,
}
impl ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {}
impl ToListMappable for ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
        ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef {
        ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceAuthConfigElApiKeyConfigElDynamic {
    api_key: Option<DynamicBlock<ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElApiKeyConfigEl {
    http_element_location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<Vec<ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl>>,
    dynamic: ApihubPluginInstanceAuthConfigElApiKeyConfigElDynamic,
}
impl ApihubPluginInstanceAuthConfigElApiKeyConfigEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceAuthConfigElApiKeyConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElApiKeyConfigEl {
    #[doc = "The location of the API key.\nThe default value is QUERY.\nPossible values:\nHTTP_ELEMENT_LOCATION_UNSPECIFIED\nQUERY\nHEADER\nPATH\nBODY\nCOOKIE"]
    pub http_element_location: PrimField<String>,
    #[doc = "The parameter name of the API key.\nE.g. If the API request is \"https://example.com/act?api_key=\",\n\"api_key\" would be the parameter name."]
    pub name: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElApiKeyConfigEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElApiKeyConfigEl {
        ApihubPluginInstanceAuthConfigElApiKeyConfigEl {
            http_element_location: self.http_element_location,
            name: self.name,
            api_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElApiKeyConfigElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginInstanceAuthConfigElApiKeyConfigElRef {
        ApihubPluginInstanceAuthConfigElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_element_location` after provisioning.\nThe location of the API key.\nThe default value is QUERY.\nPossible values:\nHTTP_ELEMENT_LOCATION_UNSPECIFIED\nQUERY\nHEADER\nPATH\nBODY\nCOOKIE"]
    pub fn http_element_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_element_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe parameter name of the API key.\nE.g. If the API request is \"https://example.com/act?api_key=\",\n\"api_key\" would be the parameter name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> ListRef<ApihubPluginInstanceAuthConfigElApiKeyConfigElApiKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
    service_account: PrimField<String>,
}
impl ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {}
impl ToListMappable for ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
    #[doc = "The service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub service_account: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
        ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl {
            service_account: self.service_account,
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef {
        ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {
    secret_version: PrimField<String>,
}
impl ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {}
impl ToListMappable
    for ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl
{
    type O = BlockAssignable<
        ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {
    pub fn build(
        self,
    ) -> ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {
        ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef {
        ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElDynamic {
    client_secret: Option<
        DynamicBlock<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl>,
    >,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret:
        Option<Vec<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl>>,
    dynamic: ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElDynamic,
}
impl ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v: impl Into<
            BlockAssignable<
                ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
    #[doc = "The client identifier."]
    pub client_id: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
        ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef {
        ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client identifier."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(
        &self,
    ) -> ListRef<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElClientSecretElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
    secret_version: PrimField<String>,
}
impl ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {}
impl ToListMappable for ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
        ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef {
        ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceAuthConfigElUserPasswordConfigElDynamic {
    password: Option<DynamicBlock<ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<Vec<ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl>>,
    dynamic: ApihubPluginInstanceAuthConfigElUserPasswordConfigElDynamic,
}
impl ApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.password = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.password = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigElUserPasswordConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
    #[doc = "Username."]
    pub username: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
        ApihubPluginInstanceAuthConfigElUserPasswordConfigEl {
            username: self.username,
            password: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef {
        ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(
        &self,
    ) -> ListRef<ApihubPluginInstanceAuthConfigElUserPasswordConfigElPasswordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginInstanceAuthConfigElDynamic {
    api_key_config: Option<DynamicBlock<ApihubPluginInstanceAuthConfigElApiKeyConfigEl>>,
    google_service_account_config:
        Option<DynamicBlock<ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl>>,
    oauth2_client_credentials_config:
        Option<DynamicBlock<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl>>,
    user_password_config:
        Option<DynamicBlock<ApihubPluginInstanceAuthConfigElUserPasswordConfigEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceAuthConfigEl {
    auth_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config: Option<Vec<ApihubPluginInstanceAuthConfigElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_service_account_config:
        Option<Vec<ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_client_credentials_config:
        Option<Vec<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_password_config: Option<Vec<ApihubPluginInstanceAuthConfigElUserPasswordConfigEl>>,
    dynamic: ApihubPluginInstanceAuthConfigElDynamic,
}
impl ApihubPluginInstanceAuthConfigEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElApiKeyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_service_account_config`.\n"]
    pub fn set_google_service_account_config(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_service_account_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_service_account_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth2_client_credentials_config`.\n"]
    pub fn set_oauth2_client_credentials_config(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2_client_credentials_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_credentials_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_password_config`.\n"]
    pub fn set_user_password_config(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginInstanceAuthConfigElUserPasswordConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_password_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_password_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginInstanceAuthConfigEl {
    type O = BlockAssignable<ApihubPluginInstanceAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceAuthConfigEl {
    #[doc = "Possible values:\nAUTH_TYPE_UNSPECIFIED\nNO_AUTH\nGOOGLE_SERVICE_ACCOUNT\nUSER_PASSWORD\nAPI_KEY\nOAUTH2_CLIENT_CREDENTIALS"]
    pub auth_type: PrimField<String>,
}
impl BuildApihubPluginInstanceAuthConfigEl {
    pub fn build(self) -> ApihubPluginInstanceAuthConfigEl {
        ApihubPluginInstanceAuthConfigEl {
            auth_type: self.auth_type,
            api_key_config: core::default::Default::default(),
            google_service_account_config: core::default::Default::default(),
            oauth2_client_credentials_config: core::default::Default::default(),
            user_password_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceAuthConfigElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginInstanceAuthConfigElRef {
        ApihubPluginInstanceAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\nPossible values:\nAUTH_TYPE_UNSPECIFIED\nNO_AUTH\nGOOGLE_SERVICE_ACCOUNT\nUSER_PASSWORD\nAPI_KEY\nOAUTH2_CLIENT_CREDENTIALS"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> ListRef<ApihubPluginInstanceAuthConfigElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_service_account_config` after provisioning.\n"]
    pub fn google_service_account_config(
        &self,
    ) -> ListRef<ApihubPluginInstanceAuthConfigElGoogleServiceAccountConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_service_account_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_credentials_config` after provisioning.\n"]
    pub fn oauth2_client_credentials_config(
        &self,
    ) -> ListRef<ApihubPluginInstanceAuthConfigElOauth2ClientCredentialsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_client_credentials_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_password_config` after provisioning.\n"]
    pub fn user_password_config(
        &self,
    ) -> ListRef<ApihubPluginInstanceAuthConfigElUserPasswordConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_password_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApihubPluginInstanceTimeoutsEl {
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
impl ToListMappable for ApihubPluginInstanceTimeoutsEl {
    type O = BlockAssignable<ApihubPluginInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginInstanceTimeoutsEl {}
impl BuildApihubPluginInstanceTimeoutsEl {
    pub fn build(self) -> ApihubPluginInstanceTimeoutsEl {
        ApihubPluginInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginInstanceTimeoutsElRef {
        ApihubPluginInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginInstanceTimeoutsElRef {
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
struct ApihubPluginInstanceDynamic {
    actions: Option<DynamicBlock<ApihubPluginInstanceActionsEl>>,
    auth_config: Option<DynamicBlock<ApihubPluginInstanceAuthConfigEl>>,
}
