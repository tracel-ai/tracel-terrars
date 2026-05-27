use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineWidgetConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<PrimField<String>>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    widget_config_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_settings: Option<Vec<DiscoveryEngineWidgetConfigAccessSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_setting: Option<Vec<DiscoveryEngineWidgetConfigHomepageSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineWidgetConfigTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_branding: Option<Vec<DiscoveryEngineWidgetConfigUiBrandingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_settings: Option<Vec<DiscoveryEngineWidgetConfigUiSettingsEl>>,
    dynamic: DiscoveryEngineWidgetConfigDynamic,
}
struct DiscoveryEngineWidgetConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineWidgetConfigData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineWidgetConfig(Rc<DiscoveryEngineWidgetConfig_>);
impl DiscoveryEngineWidgetConfig {
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
    #[doc = "Set the field `collection_id`.\nThe collection ID."]
    pub fn set_collection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().collection_id = Some(v.into());
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
    #[doc = "Set the field `widget_config_id`.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn set_widget_config_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().widget_config_id = Some(v.into());
        self
    }
    #[doc = "Set the field `access_settings`.\n"]
    pub fn set_access_settings(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigAccessSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `homepage_setting`.\n"]
    pub fn set_homepage_setting(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().homepage_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.homepage_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineWidgetConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `ui_branding`.\n"]
    pub fn set_ui_branding(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigUiBrandingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ui_branding = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ui_branding = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ui_settings`.\n"]
    pub fn set_ui_settings(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ui_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ui_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nOutput only. Unique obfuscated identifier of a WidgetConfig."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine ID."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the widget config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/widgetConfigs/{widget_config_id}'."]
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
    #[doc = "Get a reference to the value of field `widget_config_id` after provisioning.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn widget_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.widget_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> ListRef<DiscoveryEngineWidgetConfigAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `homepage_setting` after provisioning.\n"]
    pub fn homepage_setting(&self) -> ListRef<DiscoveryEngineWidgetConfigHomepageSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.homepage_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineWidgetConfigTimeoutsElRef {
        DiscoveryEngineWidgetConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ui_branding` after provisioning.\n"]
    pub fn ui_branding(&self) -> ListRef<DiscoveryEngineWidgetConfigUiBrandingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_branding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ui_settings` after provisioning.\n"]
    pub fn ui_settings(&self) -> ListRef<DiscoveryEngineWidgetConfigUiSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_settings", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineWidgetConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineWidgetConfig {}
impl ToListMappable for DiscoveryEngineWidgetConfig {
    type O = ListRef<DiscoveryEngineWidgetConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineWidgetConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_widget_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineWidgetConfig {
    pub tf_id: String,
    #[doc = "The engine ID."]
    pub engine_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineWidgetConfig {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineWidgetConfig {
        let out = DiscoveryEngineWidgetConfig(Rc::new(DiscoveryEngineWidgetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineWidgetConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collection_id: core::default::Default::default(),
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                widget_config_id: core::default::Default::default(),
                access_settings: core::default::Default::default(),
                homepage_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ui_branding: core::default::Default::default(),
                ui_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineWidgetConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineWidgetConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nOutput only. Unique obfuscated identifier of a WidgetConfig."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine ID."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the widget config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/widgetConfigs/{widget_config_id}'."]
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
    #[doc = "Get a reference to the value of field `widget_config_id` after provisioning.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn widget_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.widget_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> ListRef<DiscoveryEngineWidgetConfigAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `homepage_setting` after provisioning.\n"]
    pub fn homepage_setting(&self) -> ListRef<DiscoveryEngineWidgetConfigHomepageSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.homepage_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineWidgetConfigTimeoutsElRef {
        DiscoveryEngineWidgetConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ui_branding` after provisioning.\n"]
    pub fn ui_branding(&self) -> ListRef<DiscoveryEngineWidgetConfigUiBrandingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_branding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ui_settings` after provisioning.\n"]
    pub fn ui_settings(&self) -> ListRef<DiscoveryEngineWidgetConfigUiSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigAccessSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_public_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowlisted_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_web_app: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_identity_pool_provider: Option<PrimField<String>>,
}
impl DiscoveryEngineWidgetConfigAccessSettingsEl {
    #[doc = "Set the field `allow_public_access`.\nWhether public unauthenticated access is allowed."]
    pub fn set_allow_public_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_public_access = Some(v.into());
        self
    }
    #[doc = "Set the field `allowlisted_domains`.\nList of domains that are allowed to integrate the search widget."]
    pub fn set_allowlisted_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowlisted_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_web_app`.\nWhether web app access is enabled."]
    pub fn set_enable_web_app(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_web_app = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nLanguage code for user interface. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). If unset, the\ndefault language code is \"en-US\"."]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `workforce_identity_pool_provider`.\nThe workforce identity pool provider used to access the widget."]
    pub fn set_workforce_identity_pool_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workforce_identity_pool_provider = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigAccessSettingsEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigAccessSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigAccessSettingsEl {}
impl BuildDiscoveryEngineWidgetConfigAccessSettingsEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigAccessSettingsEl {
        DiscoveryEngineWidgetConfigAccessSettingsEl {
            allow_public_access: core::default::Default::default(),
            allowlisted_domains: core::default::Default::default(),
            enable_web_app: core::default::Default::default(),
            language_code: core::default::Default::default(),
            workforce_identity_pool_provider: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigAccessSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigAccessSettingsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigAccessSettingsElRef {
        DiscoveryEngineWidgetConfigAccessSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigAccessSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_public_access` after provisioning.\nWhether public unauthenticated access is allowed."]
    pub fn allow_public_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_public_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowlisted_domains` after provisioning.\nList of domains that are allowed to integrate the search widget."]
    pub fn allowlisted_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowlisted_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_web_app` after provisioning.\nWhether web app access is enabled."]
    pub fn enable_web_app(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_web_app", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for user interface. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). If unset, the\ndefault language code is \"en-US\"."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_identity_pool_provider` after provisioning.\nThe workforce identity pool provider used to access the widget."]
    pub fn workforce_identity_pool_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_identity_pool_provider", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
    #[doc = "Set the field `url`.\nImage URL."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {}
impl BuildDiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
        DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl {
            url: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef {
        DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nImage URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElDynamic {
    icon: Option<DynamicBlock<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Vec<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl>>,
    dynamic: DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElDynamic,
}
impl DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
    #[doc = "Set the field `destination_uri`.\nDestination URL of shortcut."]
    pub fn set_destination_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle of the shortcut."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `icon`.\n"]
    pub fn set_icon(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.icon = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.icon = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {}
impl BuildDiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
        DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl {
            destination_uri: core::default::Default::default(),
            title: core::default::Default::default(),
            icon: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef {
        DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_uri` after provisioning.\nDestination URL of shortcut."]
    pub fn destination_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle of the shortcut."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `icon` after provisioning.\n"]
    pub fn icon(
        &self,
    ) -> ListRef<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElIconElRef> {
        ListRef::new(self.shared().clone(), format!("{}.icon", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigHomepageSettingElDynamic {
    shortcuts: Option<DynamicBlock<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigHomepageSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    shortcuts: Option<Vec<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl>>,
    dynamic: DiscoveryEngineWidgetConfigHomepageSettingElDynamic,
}
impl DiscoveryEngineWidgetConfigHomepageSettingEl {
    #[doc = "Set the field `shortcuts`.\n"]
    pub fn set_shortcuts(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shortcuts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shortcuts = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigHomepageSettingEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigHomepageSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigHomepageSettingEl {}
impl BuildDiscoveryEngineWidgetConfigHomepageSettingEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigHomepageSettingEl {
        DiscoveryEngineWidgetConfigHomepageSettingEl {
            shortcuts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigHomepageSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigHomepageSettingElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigHomepageSettingElRef {
        DiscoveryEngineWidgetConfigHomepageSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigHomepageSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `shortcuts` after provisioning.\n"]
    pub fn shortcuts(&self) -> ListRef<DiscoveryEngineWidgetConfigHomepageSettingElShortcutsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shortcuts", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineWidgetConfigTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineWidgetConfigTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigTimeoutsEl {}
impl BuildDiscoveryEngineWidgetConfigTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigTimeoutsEl {
        DiscoveryEngineWidgetConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigTimeoutsElRef {
        DiscoveryEngineWidgetConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigTimeoutsElRef {
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
pub struct DiscoveryEngineWidgetConfigUiBrandingElLogoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl DiscoveryEngineWidgetConfigUiBrandingElLogoEl {
    #[doc = "Set the field `url`.\nImage URL."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiBrandingElLogoEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigUiBrandingElLogoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiBrandingElLogoEl {}
impl BuildDiscoveryEngineWidgetConfigUiBrandingElLogoEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiBrandingElLogoEl {
        DiscoveryEngineWidgetConfigUiBrandingElLogoEl {
            url: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiBrandingElLogoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiBrandingElLogoElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigUiBrandingElLogoElRef {
        DiscoveryEngineWidgetConfigUiBrandingElLogoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiBrandingElLogoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nImage URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigUiBrandingElDynamic {
    logo: Option<DynamicBlock<DiscoveryEngineWidgetConfigUiBrandingElLogoEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiBrandingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    logo: Option<Vec<DiscoveryEngineWidgetConfigUiBrandingElLogoEl>>,
    dynamic: DiscoveryEngineWidgetConfigUiBrandingElDynamic,
}
impl DiscoveryEngineWidgetConfigUiBrandingEl {
    #[doc = "Set the field `logo`.\n"]
    pub fn set_logo(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigUiBrandingElLogoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logo = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiBrandingEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigUiBrandingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiBrandingEl {}
impl BuildDiscoveryEngineWidgetConfigUiBrandingEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiBrandingEl {
        DiscoveryEngineWidgetConfigUiBrandingEl {
            logo: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiBrandingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiBrandingElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigUiBrandingElRef {
        DiscoveryEngineWidgetConfigUiBrandingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiBrandingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> ListRef<DiscoveryEngineWidgetConfigUiBrandingElLogoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logo", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    field: PrimField<String>,
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
    #[doc = "Set the field `display_name`.\nThe field name that end users will see."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
    type O =
        BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
    #[doc = "Registered field name. The format is 'field.abc'."]
    pub field: PrimField<String>,
}
impl BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl {
            display_name: core::default::Default::default(),
            field: self.field,
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe field name that end users will see."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nRegistered field name. The format is 'field.abc'."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_visibility: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_template: Option<PrimField<String>>,
    field: PrimField<String>,
    ui_component: PrimField<String>,
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
    #[doc = "Set the field `device_visibility`.\n Possible values: [\"MOBILE\", \"DESKTOP\"]"]
    pub fn set_device_visibility(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_visibility = Some(v.into());
        self
    }
    #[doc = "Set the field `display_template`.\nThe template to customize how the field is displayed.\nAn example value would be a string that looks like: \"Price: {value}\"."]
    pub fn set_display_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_template = Some(v.into());
        self
    }
}
impl ToListMappable
    for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl
{
    type O = BlockAssignable<
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
    #[doc = "Registered field name. The format is 'field.abc'."]
    pub field: PrimField<String>,
    #[doc = ""]
    pub ui_component: PrimField<String>,
}
impl BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
    pub fn build(
        self,
    ) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl {
            device_visibility: core::default::Default::default(),
            display_template: core::default::Default::default(),
            field: self.field,
            ui_component: self.ui_component,
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapElRef {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_visibility` after provisioning.\n Possible values: [\"MOBILE\", \"DESKTOP\"]"]
    pub fn device_visibility(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_visibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_template` after provisioning.\nThe template to customize how the field is displayed.\nAn example value would be a string that looks like: \"Price: {value}\"."]
    pub fn display_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nRegistered field name. The format is 'field.abc'."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `ui_component` after provisioning.\n"]
    pub fn ui_component(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ui_component", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElDynamic {
    facet_field: Option<
        DynamicBlock<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl>,
    >,
    fields_ui_components_map: Option<
        DynamicBlock<
            DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_field:
        Option<Vec<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields_ui_components_map: Option<
        Vec<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl>,
    >,
    dynamic: DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElDynamic,
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
    #[doc = "Set the field `name`.\nThe name of the data store. It should be data store resource name. Format:\n'projects/{project}/locations/{location}/collections/{collectionId}/dataStores/{dataStoreId}'.\nFor APIs under 'WidgetService', such as [WidgetService.LookUpWidgetConfig][],\nthe project number and location part is erased in this field."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `facet_field`.\n"]
    pub fn set_facet_field(
        mut self,
        v: impl Into<
            BlockAssignable<
                DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.facet_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.facet_field = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fields_ui_components_map`.\n"]
    pub fn set_fields_ui_components_map(
        mut self,
        v: impl Into<
            BlockAssignable<
                DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFieldsUiComponentsMapEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields_ui_components_map = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields_ui_components_map = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {}
impl BuildDiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl {
            name: core::default::Default::default(),
            facet_field: core::default::Default::default(),
            fields_ui_components_map: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef {
        DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the data store. It should be data store resource name. Format:\n'projects/{project}/locations/{location}/collections/{collectionId}/dataStores/{dataStoreId}'.\nFor APIs under 'WidgetService', such as [WidgetService.LookUpWidgetConfig][],\nthe project number and location part is erased in this field."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `facet_field` after provisioning.\n"]
    pub fn facet_field(
        &self,
    ) -> ListRef<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElFacetFieldElRef> {
        ListRef::new(self.shared().clone(), format!("{}.facet_field", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_related_questions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_adversarial_query: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_low_relevant_content: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_non_answer_seeking_query: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rephrase_steps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_prompt_preamble: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_count: Option<PrimField<f64>>,
}
impl DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
    #[doc = "Set the field `disable_related_questions`.\nWhether generated answer contains suggested related questions."]
    pub fn set_disable_related_questions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_related_questions = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_adversarial_query`.\nSpecifies whether to filter out queries that are adversarial."]
    pub fn set_ignore_adversarial_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_adversarial_query = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_low_relevant_content`.\nSpecifies whether to filter out queries that are not relevant to the content."]
    pub fn set_ignore_low_relevant_content(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_low_relevant_content = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_non_answer_seeking_query`.\nSpecifies whether to filter out queries that are not answer-seeking.\nThe default value is 'false'. No answer is returned if the search query\nis classified as a non-answer seeking query.\nIf this field is set to 'true', we skip generating answers for\nnon-answer seeking queries and return fallback messages instead."]
    pub fn set_ignore_non_answer_seeking_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_non_answer_seeking_query = Some(v.into());
        self
    }
    #[doc = "Set the field `image_source`.\nSource of image returned in the answer. Possible values: [\"ALL_AVAILABLE_SOURCES\", \"CORPUS_IMAGE_ONLY\", \"FIGURE_GENERATION_ONLY\"]"]
    pub fn set_image_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_source = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nLanguage code for Summary. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Note: This\nis an experimental feature."]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `max_rephrase_steps`.\nMax rephrase steps. The max number is 5 steps. If not set or\nset to < 1, it will be set to 1 by default."]
    pub fn set_max_rephrase_steps(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_rephrase_steps = Some(v.into());
        self
    }
    #[doc = "Set the field `model_prompt_preamble`.\nText at the beginning of the prompt that instructs the model that generates the answer."]
    pub fn set_model_prompt_preamble(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_prompt_preamble = Some(v.into());
        self
    }
    #[doc = "Set the field `model_version`.\nThe model version used to generate the answer."]
    pub fn set_model_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_version = Some(v.into());
        self
    }
    #[doc = "Set the field `result_count`.\nThe number of top results to generate the answer from. Up to 10."]
    pub fn set_result_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.result_count = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {}
impl BuildDiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
        DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl {
            disable_related_questions: core::default::Default::default(),
            ignore_adversarial_query: core::default::Default::default(),
            ignore_low_relevant_content: core::default::Default::default(),
            ignore_non_answer_seeking_query: core::default::Default::default(),
            image_source: core::default::Default::default(),
            language_code: core::default::Default::default(),
            max_rephrase_steps: core::default::Default::default(),
            model_prompt_preamble: core::default::Default::default(),
            model_version: core::default::Default::default(),
            result_count: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef {
        DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_related_questions` after provisioning.\nWhether generated answer contains suggested related questions."]
    pub fn disable_related_questions(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_related_questions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_adversarial_query` after provisioning.\nSpecifies whether to filter out queries that are adversarial."]
    pub fn ignore_adversarial_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_adversarial_query", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_low_relevant_content` after provisioning.\nSpecifies whether to filter out queries that are not relevant to the content."]
    pub fn ignore_low_relevant_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_low_relevant_content", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_non_answer_seeking_query` after provisioning.\nSpecifies whether to filter out queries that are not answer-seeking.\nThe default value is 'false'. No answer is returned if the search query\nis classified as a non-answer seeking query.\nIf this field is set to 'true', we skip generating answers for\nnon-answer seeking queries and return fallback messages instead."]
    pub fn ignore_non_answer_seeking_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_non_answer_seeking_query", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_source` after provisioning.\nSource of image returned in the answer. Possible values: [\"ALL_AVAILABLE_SOURCES\", \"CORPUS_IMAGE_ONLY\", \"FIGURE_GENERATION_ONLY\"]"]
    pub fn image_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_source", self.base))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for Summary. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Note: This\nis an experimental feature."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_rephrase_steps` after provisioning.\nMax rephrase steps. The max number is 5 steps. If not set or\nset to < 1, it will be set to 1 by default."]
    pub fn max_rephrase_steps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_rephrase_steps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_prompt_preamble` after provisioning.\nText at the beginning of the prompt that instructs the model that generates the answer."]
    pub fn model_prompt_preamble(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_prompt_preamble", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_version` after provisioning.\nThe model version used to generate the answer."]
    pub fn model_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `result_count` after provisioning.\nThe number of top results to generate the answer from. Up to 10."]
    pub fn result_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.result_count", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigUiSettingsElDynamic {
    data_store_ui_configs:
        Option<DynamicBlock<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl>>,
    generative_answer_config:
        Option<DynamicBlock<DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfigUiSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_search_request_order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_user_events_collection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_autocomplete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_create_agent_button: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_people_search: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_quality_feedback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_safe_search: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_search_as_you_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_visual_content_summary: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interaction_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_description_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_ui_configs: Option<Vec<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_answer_config:
        Option<Vec<DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl>>,
    dynamic: DiscoveryEngineWidgetConfigUiSettingsElDynamic,
}
impl DiscoveryEngineWidgetConfigUiSettingsEl {
    #[doc = "Set the field `default_search_request_order_by`.\nThe default ordering for search results if specified.\nUsed to set SearchRequest#orderBy on applicable requests."]
    pub fn set_default_search_request_order_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_search_request_order_by = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_user_events_collection`.\nIf set to true, the widget will not collect user events."]
    pub fn set_disable_user_events_collection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_user_events_collection = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_autocomplete`.\nWhether or not to enable autocomplete."]
    pub fn set_enable_autocomplete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_autocomplete = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_create_agent_button`.\nIf set to true, the widget will enable the create agent button."]
    pub fn set_enable_create_agent_button(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_create_agent_button = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_people_search`.\nIf set to true, the widget will enable people search."]
    pub fn set_enable_people_search(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_people_search = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_quality_feedback`.\nTurn on or off collecting the search result quality feedback from end users."]
    pub fn set_enable_quality_feedback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_quality_feedback = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_safe_search`.\nWhether to enable safe search."]
    pub fn set_enable_safe_search(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_safe_search = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_search_as_you_type`.\nWhether to enable search-as-you-type behavior for the search widget."]
    pub fn set_enable_search_as_you_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_search_as_you_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_visual_content_summary`.\nIf set to true, the widget will enable visual content summary on applicable\nsearch requests. Only used by healthcare search."]
    pub fn set_enable_visual_content_summary(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_visual_content_summary = Some(v.into());
        self
    }
    #[doc = "Set the field `interaction_type`.\nDescribes widget (or web app) interaction type Possible values: [\"SEARCH_ONLY\", \"SEARCH_WITH_ANSWER\", \"SEARCH_WITH_FOLLOW_UPS\"]"]
    pub fn set_interaction_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interaction_type = Some(v.into());
        self
    }
    #[doc = "Set the field `result_description_type`.\nControls whether result extract is display and how (snippet or extractive answer).\nDefault to no result if unspecified. Possible values: [\"SNIPPET\", \"EXTRACTIVE_ANSWER\"]"]
    pub fn set_result_description_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result_description_type = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_ui_configs`.\n"]
    pub fn set_data_store_ui_configs(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store_ui_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_ui_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generative_answer_config`.\n"]
    pub fn set_generative_answer_config(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generative_answer_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_answer_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineWidgetConfigUiSettingsEl {
    type O = BlockAssignable<DiscoveryEngineWidgetConfigUiSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineWidgetConfigUiSettingsEl {}
impl BuildDiscoveryEngineWidgetConfigUiSettingsEl {
    pub fn build(self) -> DiscoveryEngineWidgetConfigUiSettingsEl {
        DiscoveryEngineWidgetConfigUiSettingsEl {
            default_search_request_order_by: core::default::Default::default(),
            disable_user_events_collection: core::default::Default::default(),
            enable_autocomplete: core::default::Default::default(),
            enable_create_agent_button: core::default::Default::default(),
            enable_people_search: core::default::Default::default(),
            enable_quality_feedback: core::default::Default::default(),
            enable_safe_search: core::default::Default::default(),
            enable_search_as_you_type: core::default::Default::default(),
            enable_visual_content_summary: core::default::Default::default(),
            interaction_type: core::default::Default::default(),
            result_description_type: core::default::Default::default(),
            data_store_ui_configs: core::default::Default::default(),
            generative_answer_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineWidgetConfigUiSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineWidgetConfigUiSettingsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineWidgetConfigUiSettingsElRef {
        DiscoveryEngineWidgetConfigUiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineWidgetConfigUiSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_search_request_order_by` after provisioning.\nThe default ordering for search results if specified.\nUsed to set SearchRequest#orderBy on applicable requests."]
    pub fn default_search_request_order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_search_request_order_by", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_user_events_collection` after provisioning.\nIf set to true, the widget will not collect user events."]
    pub fn disable_user_events_collection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_user_events_collection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_autocomplete` after provisioning.\nWhether or not to enable autocomplete."]
    pub fn enable_autocomplete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_autocomplete", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_create_agent_button` after provisioning.\nIf set to true, the widget will enable the create agent button."]
    pub fn enable_create_agent_button(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_create_agent_button", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_people_search` after provisioning.\nIf set to true, the widget will enable people search."]
    pub fn enable_people_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_people_search", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_quality_feedback` after provisioning.\nTurn on or off collecting the search result quality feedback from end users."]
    pub fn enable_quality_feedback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_quality_feedback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_safe_search` after provisioning.\nWhether to enable safe search."]
    pub fn enable_safe_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_safe_search", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_search_as_you_type` after provisioning.\nWhether to enable search-as-you-type behavior for the search widget."]
    pub fn enable_search_as_you_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_search_as_you_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_visual_content_summary` after provisioning.\nIf set to true, the widget will enable visual content summary on applicable\nsearch requests. Only used by healthcare search."]
    pub fn enable_visual_content_summary(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_visual_content_summary", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interaction_type` after provisioning.\nDescribes widget (or web app) interaction type Possible values: [\"SEARCH_ONLY\", \"SEARCH_WITH_ANSWER\", \"SEARCH_WITH_FOLLOW_UPS\"]"]
    pub fn interaction_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interaction_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `result_description_type` after provisioning.\nControls whether result extract is display and how (snippet or extractive answer).\nDefault to no result if unspecified. Possible values: [\"SNIPPET\", \"EXTRACTIVE_ANSWER\"]"]
    pub fn result_description_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.result_description_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ui_configs` after provisioning.\n"]
    pub fn data_store_ui_configs(
        &self,
    ) -> ListRef<DiscoveryEngineWidgetConfigUiSettingsElDataStoreUiConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ui_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generative_answer_config` after provisioning.\n"]
    pub fn generative_answer_config(
        &self,
    ) -> ListRef<DiscoveryEngineWidgetConfigUiSettingsElGenerativeAnswerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_answer_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigDynamic {
    access_settings: Option<DynamicBlock<DiscoveryEngineWidgetConfigAccessSettingsEl>>,
    homepage_setting: Option<DynamicBlock<DiscoveryEngineWidgetConfigHomepageSettingEl>>,
    ui_branding: Option<DynamicBlock<DiscoveryEngineWidgetConfigUiBrandingEl>>,
    ui_settings: Option<DynamicBlock<DiscoveryEngineWidgetConfigUiSettingsEl>>,
}
