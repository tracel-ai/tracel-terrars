use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApihubPluginData {
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
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_category: Option<PrimField<String>>,
    plugin_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions_config: Option<Vec<ApihubPluginActionsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_template: Option<Vec<ApihubPluginConfigTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<Vec<ApihubPluginDocumentationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosting_service: Option<Vec<ApihubPluginHostingServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApihubPluginTimeoutsEl>,
    dynamic: ApihubPluginDynamic,
}
struct ApihubPlugin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApihubPluginData>,
}
#[derive(Clone)]
pub struct ApihubPlugin(Rc<ApihubPlugin_>);
impl ApihubPlugin {
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
    #[doc = "Set the field `description`.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `plugin_category`.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn set_plugin_category(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plugin_category = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `actions_config`.\n"]
    pub fn set_actions_config(
        self,
        v: impl Into<BlockAssignable<ApihubPluginActionsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `config_template`.\n"]
    pub fn set_config_template(
        self,
        v: impl Into<BlockAssignable<ApihubPluginConfigTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config_template = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `documentation`.\n"]
    pub fn set_documentation(
        self,
        v: impl Into<BlockAssignable<ApihubPluginDocumentationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().documentation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.documentation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hosting_service`.\n"]
    pub fn set_hosting_service(
        self,
        v: impl Into<BlockAssignable<ApihubPluginHostingServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hosting_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hosting_service = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApihubPluginTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the plugin.\nFormat: 'projects/{project}/locations/{location}/plugins/{plugin}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ownership_type` after provisioning.\nThe type of the plugin, indicating whether it is 'SYSTEM_OWNED' or\n'USER_OWNED'.\nPossible values:\nOWNERSHIP_TYPE_UNSPECIFIED\nSYSTEM_OWNED\nUSER_OWNED"]
    pub fn ownership_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_category` after provisioning.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn plugin_category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_id` after provisioning.\nThe ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub fn plugin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the state of the plugin.\nNote this field will not be set for plugins developed via plugin\nframework as the state will be managed at plugin instance level.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions_config` after provisioning.\n"]
    pub fn actions_config(&self) -> ListRef<ApihubPluginActionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_template` after provisioning.\n"]
    pub fn config_template(&self) -> ListRef<ApihubPluginConfigTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> ListRef<ApihubPluginDocumentationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosting_service` after provisioning.\n"]
    pub fn hosting_service(&self) -> ListRef<ApihubPluginHostingServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hosting_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginTimeoutsElRef {
        ApihubPluginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApihubPlugin {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApihubPlugin {}
impl ToListMappable for ApihubPlugin {
    type O = ListRef<ApihubPluginRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApihubPlugin_ {
    fn extract_resource_type(&self) -> String {
        "google_apihub_plugin".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApihubPlugin {
    pub tf_id: String,
    #[doc = "The display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub plugin_id: PrimField<String>,
}
impl BuildApihubPlugin {
    pub fn build(self, stack: &mut Stack) -> ApihubPlugin {
        let out = ApihubPlugin(Rc::new(ApihubPlugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApihubPluginData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                plugin_category: core::default::Default::default(),
                plugin_id: self.plugin_id,
                project: core::default::Default::default(),
                actions_config: core::default::Default::default(),
                config_template: core::default::Default::default(),
                documentation: core::default::Default::default(),
                hosting_service: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApihubPluginRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApihubPluginRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the plugin.\nFormat: 'projects/{project}/locations/{location}/plugins/{plugin}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ownership_type` after provisioning.\nThe type of the plugin, indicating whether it is 'SYSTEM_OWNED' or\n'USER_OWNED'.\nPossible values:\nOWNERSHIP_TYPE_UNSPECIFIED\nSYSTEM_OWNED\nUSER_OWNED"]
    pub fn ownership_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_category` after provisioning.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn plugin_category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_id` after provisioning.\nThe ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub fn plugin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the state of the plugin.\nNote this field will not be set for plugins developed via plugin\nframework as the state will be managed at plugin instance level.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions_config` after provisioning.\n"]
    pub fn actions_config(&self) -> ListRef<ApihubPluginActionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_template` after provisioning.\n"]
    pub fn config_template(&self) -> ListRef<ApihubPluginConfigTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> ListRef<ApihubPluginDocumentationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosting_service` after provisioning.\n"]
    pub fn hosting_service(&self) -> ListRef<ApihubPluginHostingServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hosting_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginTimeoutsElRef {
        ApihubPluginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginActionsConfigEl {
    description: PrimField<String>,
    display_name: PrimField<String>,
    id: PrimField<String>,
    trigger_mode: PrimField<String>,
}
impl ApihubPluginActionsConfigEl {}
impl ToListMappable for ApihubPluginActionsConfigEl {
    type O = BlockAssignable<ApihubPluginActionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginActionsConfigEl {
    #[doc = "The description of the operation performed by the action."]
    pub description: PrimField<String>,
    #[doc = "The display name of the action."]
    pub display_name: PrimField<String>,
    #[doc = "The id of the action."]
    pub id: PrimField<String>,
    #[doc = "The trigger mode supported by the action.\nPossible values:\nTRIGGER_MODE_UNSPECIFIED\nAPI_HUB_ON_DEMAND_TRIGGER\nAPI_HUB_SCHEDULE_TRIGGER\nNON_API_HUB_MANAGED"]
    pub trigger_mode: PrimField<String>,
}
impl BuildApihubPluginActionsConfigEl {
    pub fn build(self) -> ApihubPluginActionsConfigEl {
        ApihubPluginActionsConfigEl {
            description: self.description,
            display_name: self.display_name,
            id: self.id,
            trigger_mode: self.trigger_mode,
        }
    }
}
pub struct ApihubPluginActionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginActionsConfigElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginActionsConfigElRef {
        ApihubPluginActionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginActionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the operation performed by the action."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the action."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the action."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger_mode` after provisioning.\nThe trigger mode supported by the action.\nPossible values:\nTRIGGER_MODE_UNSPECIFIED\nAPI_HUB_ON_DEMAND_TRIGGER\nAPI_HUB_SCHEDULE_TRIGGER\nNON_API_HUB_MANAGED"]
    pub fn trigger_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    id: PrimField<String>,
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
    #[doc = "Set the field `description`.\nDescription of the option."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
    type O = BlockAssignable<ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
    #[doc = "Display name of the option."]
    pub display_name: PrimField<String>,
    #[doc = "Id of the option."]
    pub id: PrimField<String>,
}
impl BuildApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
    pub fn build(self) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl {
            description: core::default::Default::default(),
            display_name: self.display_name,
            id: self.id,
        }
    }
}
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the option."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the option."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the option."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    id: PrimField<String>,
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
    #[doc = "Set the field `description`.\nDescription of the option."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
    type O =
        BlockAssignable<ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
    #[doc = "Display name of the option."]
    pub display_name: PrimField<String>,
    #[doc = "Id of the option."]
    pub id: PrimField<String>,
}
impl BuildApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
    pub fn build(
        self,
    ) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl {
            description: core::default::Default::default(),
            display_name: self.display_name,
            id: self.id,
        }
    }
}
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the option."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the option."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the option."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElDynamic {
    enum_options:
        Option<DynamicBlock<ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl>>,
    multi_select_options: Option<
        DynamicBlock<ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl>,
    >,
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_regex: Option<PrimField<String>>,
    value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_options: Option<Vec<ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_select_options:
        Option<Vec<ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl>>,
    dynamic: ApihubPluginConfigTemplateElAdditionalConfigTemplateElDynamic,
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
    #[doc = "Set the field `description`.\nDescription."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\nFlag represents that this 'ConfigVariable' must be provided for a\nPluginInstance."]
    pub fn set_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `validation_regex`.\nRegular expression in RE2 syntax used for validating the 'value' of a\n'ConfigVariable'."]
    pub fn set_validation_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.validation_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_options`.\n"]
    pub fn set_enum_options(
        mut self,
        v: impl Into<
            BlockAssignable<ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enum_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_select_options`.\n"]
    pub fn set_multi_select_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_select_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_select_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
    type O = BlockAssignable<ApihubPluginConfigTemplateElAdditionalConfigTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
    #[doc = "ID of the config variable. Must be unique within the configuration."]
    pub id: PrimField<String>,
    #[doc = "Type of the parameter: string, int, bool etc.\nPossible values:\nVALUE_TYPE_UNSPECIFIED\nSTRING\nINT\nBOOL\nSECRET\nENUM\nMULTI_SELECT\nMULTI_STRING\nMULTI_INT"]
    pub value_type: PrimField<String>,
}
impl BuildApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
    pub fn build(self) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateEl {
            description: core::default::Default::default(),
            id: self.id,
            required: core::default::Default::default(),
            validation_regex: core::default::Default::default(),
            value_type: self.value_type,
            enum_options: core::default::Default::default(),
            multi_select_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef {
        ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the config variable. Must be unique within the configuration."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nFlag represents that this 'ConfigVariable' must be provided for a\nPluginInstance."]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `validation_regex` after provisioning.\nRegular expression in RE2 syntax used for validating the 'value' of a\n'ConfigVariable'."]
    pub fn validation_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_regex", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nType of the parameter: string, int, bool etc.\nPossible values:\nVALUE_TYPE_UNSPECIFIED\nSTRING\nINT\nBOOL\nSECRET\nENUM\nMULTI_SELECT\nMULTI_STRING\nMULTI_INT"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_options` after provisioning.\n"]
    pub fn enum_options(
        &self,
    ) -> ListRef<ApihubPluginConfigTemplateElAdditionalConfigTemplateElEnumOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enum_options", self.base))
    }
    #[doc = "Get a reference to the value of field `multi_select_options` after provisioning.\n"]
    pub fn multi_select_options(
        &self,
    ) -> ListRef<ApihubPluginConfigTemplateElAdditionalConfigTemplateElMultiSelectOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_select_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
    service_account: PrimField<String>,
}
impl ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {}
impl ToListMappable for ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
    type O = BlockAssignable<ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
    #[doc = "The service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub service_account: PrimField<String>,
}
impl BuildApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
    pub fn build(self) -> ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
        ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl {
            service_account: self.service_account,
        }
    }
}
pub struct ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef {
        ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef {
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
#[derive(Serialize, Default)]
struct ApihubPluginConfigTemplateElAuthConfigTemplateElDynamic {
    service_account:
        Option<DynamicBlock<ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateElAuthConfigTemplateEl {
    supported_auth_types: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<Vec<ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl>>,
    dynamic: ApihubPluginConfigTemplateElAuthConfigTemplateElDynamic,
}
impl ApihubPluginConfigTemplateElAuthConfigTemplateEl {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginConfigTemplateElAuthConfigTemplateEl {
    type O = BlockAssignable<ApihubPluginConfigTemplateElAuthConfigTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateElAuthConfigTemplateEl {
    #[doc = "The list of authentication types supported by the plugin."]
    pub supported_auth_types: ListField<PrimField<String>>,
}
impl BuildApihubPluginConfigTemplateElAuthConfigTemplateEl {
    pub fn build(self) -> ApihubPluginConfigTemplateElAuthConfigTemplateEl {
        ApihubPluginConfigTemplateElAuthConfigTemplateEl {
            supported_auth_types: self.supported_auth_types,
            service_account: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginConfigTemplateElAuthConfigTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElAuthConfigTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApihubPluginConfigTemplateElAuthConfigTemplateElRef {
        ApihubPluginConfigTemplateElAuthConfigTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElAuthConfigTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `supported_auth_types` after provisioning.\nThe list of authentication types supported by the plugin."]
    pub fn supported_auth_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_auth_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(
        &self,
    ) -> ListRef<ApihubPluginConfigTemplateElAuthConfigTemplateElServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApihubPluginConfigTemplateElDynamic {
    additional_config_template:
        Option<DynamicBlock<ApihubPluginConfigTemplateElAdditionalConfigTemplateEl>>,
    auth_config_template: Option<DynamicBlock<ApihubPluginConfigTemplateElAuthConfigTemplateEl>>,
}
#[derive(Serialize)]
pub struct ApihubPluginConfigTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_config_template: Option<Vec<ApihubPluginConfigTemplateElAdditionalConfigTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_config_template: Option<Vec<ApihubPluginConfigTemplateElAuthConfigTemplateEl>>,
    dynamic: ApihubPluginConfigTemplateElDynamic,
}
impl ApihubPluginConfigTemplateEl {
    #[doc = "Set the field `additional_config_template`.\n"]
    pub fn set_additional_config_template(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginConfigTemplateElAdditionalConfigTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_config_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_config_template = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auth_config_template`.\n"]
    pub fn set_auth_config_template(
        mut self,
        v: impl Into<BlockAssignable<ApihubPluginConfigTemplateElAuthConfigTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_config_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config_template = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApihubPluginConfigTemplateEl {
    type O = BlockAssignable<ApihubPluginConfigTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginConfigTemplateEl {}
impl BuildApihubPluginConfigTemplateEl {
    pub fn build(self) -> ApihubPluginConfigTemplateEl {
        ApihubPluginConfigTemplateEl {
            additional_config_template: core::default::Default::default(),
            auth_config_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApihubPluginConfigTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginConfigTemplateElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginConfigTemplateElRef {
        ApihubPluginConfigTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginConfigTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_config_template` after provisioning.\n"]
    pub fn additional_config_template(
        &self,
    ) -> ListRef<ApihubPluginConfigTemplateElAdditionalConfigTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_config_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_config_template` after provisioning.\n"]
    pub fn auth_config_template(
        &self,
    ) -> ListRef<ApihubPluginConfigTemplateElAuthConfigTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_config_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubPluginDocumentationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_uri: Option<PrimField<String>>,
}
impl ApihubPluginDocumentationEl {
    #[doc = "Set the field `external_uri`.\nThe uri of the externally hosted documentation."]
    pub fn set_external_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_uri = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginDocumentationEl {
    type O = BlockAssignable<ApihubPluginDocumentationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginDocumentationEl {}
impl BuildApihubPluginDocumentationEl {
    pub fn build(self) -> ApihubPluginDocumentationEl {
        ApihubPluginDocumentationEl {
            external_uri: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginDocumentationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginDocumentationElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginDocumentationElRef {
        ApihubPluginDocumentationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginDocumentationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_uri` after provisioning.\nThe uri of the externally hosted documentation."]
    pub fn external_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginHostingServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_uri: Option<PrimField<String>>,
}
impl ApihubPluginHostingServiceEl {
    #[doc = "Set the field `service_uri`.\nThe URI of the service implemented by the plugin developer, used to\ninvoke the plugin's functionality. This information is only required for\nuser defined plugins."]
    pub fn set_service_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_uri = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubPluginHostingServiceEl {
    type O = BlockAssignable<ApihubPluginHostingServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginHostingServiceEl {}
impl BuildApihubPluginHostingServiceEl {
    pub fn build(self) -> ApihubPluginHostingServiceEl {
        ApihubPluginHostingServiceEl {
            service_uri: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginHostingServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginHostingServiceElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginHostingServiceElRef {
        ApihubPluginHostingServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginHostingServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_uri` after provisioning.\nThe URI of the service implemented by the plugin developer, used to\ninvoke the plugin's functionality. This information is only required for\nuser defined plugins."]
    pub fn service_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ApihubPluginTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ApihubPluginTimeoutsEl {
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
}
impl ToListMappable for ApihubPluginTimeoutsEl {
    type O = BlockAssignable<ApihubPluginTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubPluginTimeoutsEl {}
impl BuildApihubPluginTimeoutsEl {
    pub fn build(self) -> ApihubPluginTimeoutsEl {
        ApihubPluginTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ApihubPluginTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubPluginTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApihubPluginTimeoutsElRef {
        ApihubPluginTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubPluginTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ApihubPluginDynamic {
    actions_config: Option<DynamicBlock<ApihubPluginActionsConfigEl>>,
    config_template: Option<DynamicBlock<ApihubPluginConfigTemplateEl>>,
    documentation: Option<DynamicBlock<ApihubPluginDocumentationEl>>,
    hosting_service: Option<DynamicBlock<ApihubPluginHostingServiceEl>>,
}
