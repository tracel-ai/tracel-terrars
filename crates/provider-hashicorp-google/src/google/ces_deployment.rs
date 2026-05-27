use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesDeploymentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    app_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_profile: Option<Vec<CesDeploymentChannelProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesDeploymentTimeoutsEl>,
    dynamic: CesDeploymentDynamic,
}
struct CesDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesDeploymentData>,
}
#[derive(Clone)]
pub struct CesDeployment(Rc<CesDeployment_>);
impl CesDeployment {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `channel_profile`.\n"]
    pub fn set_channel_profile(
        self,
        v: impl Into<BlockAssignable<CesDeploymentChannelProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().channel_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.channel_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_version` after provisioning.\nThe resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub fn app_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this deployment was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the deployment."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the deployment.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/deployments/{deployment}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this deployment was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `channel_profile` after provisioning.\n"]
    pub fn channel_profile(&self) -> ListRef<CesDeploymentChannelProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.channel_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesDeploymentTimeoutsElRef {
        CesDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesDeployment {}
impl ToListMappable for CesDeployment {
    type O = ListRef<CesDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesDeployment {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "The resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub app_version: PrimField<String>,
    #[doc = "Display name of the deployment."]
    pub display_name: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesDeployment {
    pub fn build(self, stack: &mut Stack) -> CesDeployment {
        let out = CesDeployment(Rc::new(CesDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesDeploymentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                app_version: self.app_version,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                channel_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_version` after provisioning.\nThe resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub fn app_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this deployment was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the deployment."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the deployment.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/deployments/{deployment}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this deployment was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `channel_profile` after provisioning.\n"]
    pub fn channel_profile(&self) -> ListRef<CesDeploymentChannelProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.channel_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesDeploymentTimeoutsElRef {
        CesDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesDeploymentChannelProfileElPersonaPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persona: Option<PrimField<String>>,
}
impl CesDeploymentChannelProfileElPersonaPropertyEl {
    #[doc = "Set the field `persona`.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn set_persona(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persona = Some(v.into());
        self
    }
}
impl ToListMappable for CesDeploymentChannelProfileElPersonaPropertyEl {
    type O = BlockAssignable<CesDeploymentChannelProfileElPersonaPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesDeploymentChannelProfileElPersonaPropertyEl {}
impl BuildCesDeploymentChannelProfileElPersonaPropertyEl {
    pub fn build(self) -> CesDeploymentChannelProfileElPersonaPropertyEl {
        CesDeploymentChannelProfileElPersonaPropertyEl {
            persona: core::default::Default::default(),
        }
    }
}
pub struct CesDeploymentChannelProfileElPersonaPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentChannelProfileElPersonaPropertyElRef {
    fn new(shared: StackShared, base: String) -> CesDeploymentChannelProfileElPersonaPropertyElRef {
        CesDeploymentChannelProfileElPersonaPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesDeploymentChannelProfileElPersonaPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persona` after provisioning.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn persona(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}
#[derive(Serialize)]
pub struct CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_origin_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_public_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_recaptcha: Option<PrimField<bool>>,
}
impl CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
    #[doc = "Set the field `allowed_origins`.\nThe origins that are allowed to host the web widget. An origin is defined by RFC 6454. If empty, all origins are allowed. A maximum of 100 origins is allowed. Example: \"https://example.com\""]
    pub fn set_allowed_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_origin_check`.\nIndicates whether origin check for the web widget is enabled. If true, the web widget will check the origin of the website that loads the web widget and only allow it to be loaded in the same origin or any of the allowed origins."]
    pub fn set_enable_origin_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_origin_check = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_public_access`.\nIndicates whether public access to the web widget is enabled. If true, the web widget will be publicly accessible. If false, the web widget must be integrated with your own authentication and authorization system to return valid credentials for accessing the CES agent."]
    pub fn set_enable_public_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_public_access = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_recaptcha`.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn set_enable_recaptcha(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_recaptcha = Some(v.into());
        self
    }
}
impl ToListMappable for CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
    type O = BlockAssignable<CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {}
impl BuildCesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
    pub fn build(self) -> CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
        CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl {
            allowed_origins: core::default::Default::default(),
            enable_origin_check: core::default::Default::default(),
            enable_public_access: core::default::Default::default(),
            enable_recaptcha: core::default::Default::default(),
        }
    }
}
pub struct CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef {
        CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_origins` after provisioning.\nThe origins that are allowed to host the web widget. An origin is defined by RFC 6454. If empty, all origins are allowed. A maximum of 100 origins is allowed. Example: \"https://example.com\""]
    pub fn allowed_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_origin_check` after provisioning.\nIndicates whether origin check for the web widget is enabled. If true, the web widget will check the origin of the website that loads the web widget and only allow it to be loaded in the same origin or any of the allowed origins."]
    pub fn enable_origin_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_origin_check", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_public_access` after provisioning.\nIndicates whether public access to the web widget is enabled. If true, the web widget will be publicly accessible. If false, the web widget must be integrated with your own authentication and authorization system to return valid credentials for accessing the CES agent."]
    pub fn enable_public_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_public_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_recaptcha` after provisioning.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn enable_recaptcha(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_recaptcha", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesDeploymentChannelProfileElWebWidgetConfigElDynamic {
    security_settings:
        Option<DynamicBlock<CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl>>,
}
#[derive(Serialize)]
pub struct CesDeploymentChannelProfileElWebWidgetConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    modality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings:
        Option<Vec<CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl>>,
    dynamic: CesDeploymentChannelProfileElWebWidgetConfigElDynamic,
}
impl CesDeploymentChannelProfileElWebWidgetConfigEl {
    #[doc = "Set the field `modality`.\nThe modality of the web widget.\nPossible values:\nMODALITY_UNSPECIFIED\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY\nCHAT_VOICE_AND_VIDEO"]
    pub fn set_modality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.modality = Some(v.into());
        self
    }
    #[doc = "Set the field `theme`.\nThe theme of the web widget.\nPossible values:\nTHEME_UNSPECIFIED\nLIGHT\nDARK"]
    pub fn set_theme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.theme = Some(v.into());
        self
    }
    #[doc = "Set the field `web_widget_title`.\nThe title of the web widget."]
    pub fn set_web_widget_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_widget_title = Some(v.into());
        self
    }
    #[doc = "Set the field `security_settings`.\n"]
    pub fn set_security_settings(
        mut self,
        v: impl Into<BlockAssignable<CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesDeploymentChannelProfileElWebWidgetConfigEl {
    type O = BlockAssignable<CesDeploymentChannelProfileElWebWidgetConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesDeploymentChannelProfileElWebWidgetConfigEl {}
impl BuildCesDeploymentChannelProfileElWebWidgetConfigEl {
    pub fn build(self) -> CesDeploymentChannelProfileElWebWidgetConfigEl {
        CesDeploymentChannelProfileElWebWidgetConfigEl {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
            security_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesDeploymentChannelProfileElWebWidgetConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentChannelProfileElWebWidgetConfigElRef {
    fn new(shared: StackShared, base: String) -> CesDeploymentChannelProfileElWebWidgetConfigElRef {
        CesDeploymentChannelProfileElWebWidgetConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesDeploymentChannelProfileElWebWidgetConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `modality` after provisioning.\nThe modality of the web widget.\nPossible values:\nMODALITY_UNSPECIFIED\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY\nCHAT_VOICE_AND_VIDEO"]
    pub fn modality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }
    #[doc = "Get a reference to the value of field `theme` after provisioning.\nThe theme of the web widget.\nPossible values:\nTHEME_UNSPECIFIED\nLIGHT\nDARK"]
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
    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(
        &self,
    ) -> ListRef<CesDeploymentChannelProfileElWebWidgetConfigElSecuritySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesDeploymentChannelProfileElDynamic {
    persona_property: Option<DynamicBlock<CesDeploymentChannelProfileElPersonaPropertyEl>>,
    web_widget_config: Option<DynamicBlock<CesDeploymentChannelProfileElWebWidgetConfigEl>>,
}
#[derive(Serialize)]
pub struct CesDeploymentChannelProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_barge_in_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_dtmf: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persona_property: Option<Vec<CesDeploymentChannelProfileElPersonaPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_config: Option<Vec<CesDeploymentChannelProfileElWebWidgetConfigEl>>,
    dynamic: CesDeploymentChannelProfileElDynamic,
}
impl CesDeploymentChannelProfileEl {
    #[doc = "Set the field `channel_type`.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE\nFIVE9\nCONTACT_CENTER_INTEGRATION"]
    pub fn set_channel_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel_type = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_barge_in_control`.\nWhether to disable user barge-in control in the conversation.\n- **true**: User interruptions are disabled while the agent is speaking.\n- **false**: The agent retains automatic control over when the user can\ninterrupt."]
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
        v: impl Into<BlockAssignable<CesDeploymentChannelProfileElPersonaPropertyEl>>,
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
        v: impl Into<BlockAssignable<CesDeploymentChannelProfileElWebWidgetConfigEl>>,
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
impl ToListMappable for CesDeploymentChannelProfileEl {
    type O = BlockAssignable<CesDeploymentChannelProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesDeploymentChannelProfileEl {}
impl BuildCesDeploymentChannelProfileEl {
    pub fn build(self) -> CesDeploymentChannelProfileEl {
        CesDeploymentChannelProfileEl {
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
pub struct CesDeploymentChannelProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentChannelProfileElRef {
    fn new(shared: StackShared, base: String) -> CesDeploymentChannelProfileElRef {
        CesDeploymentChannelProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesDeploymentChannelProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_type` after provisioning.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE\nFIVE9\nCONTACT_CENTER_INTEGRATION"]
    pub fn channel_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_barge_in_control` after provisioning.\nWhether to disable user barge-in control in the conversation.\n- **true**: User interruptions are disabled while the agent is speaking.\n- **false**: The agent retains automatic control over when the user can\ninterrupt."]
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
    pub fn persona_property(&self) -> ListRef<CesDeploymentChannelProfileElPersonaPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persona_property", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(&self) -> ListRef<CesDeploymentChannelProfileElWebWidgetConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_widget_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesDeploymentTimeoutsEl {
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
impl ToListMappable for CesDeploymentTimeoutsEl {
    type O = BlockAssignable<CesDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesDeploymentTimeoutsEl {}
impl BuildCesDeploymentTimeoutsEl {
    pub fn build(self) -> CesDeploymentTimeoutsEl {
        CesDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesDeploymentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesDeploymentTimeoutsElRef {
        CesDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesDeploymentTimeoutsElRef {
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
struct CesDeploymentDynamic {
    channel_profile: Option<DynamicBlock<CesDeploymentChannelProfileEl>>,
}
