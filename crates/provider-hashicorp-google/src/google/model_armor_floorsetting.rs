use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ModelArmorFloorsettingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_floor_setting_enforcement: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrated_services: Option<ListField<PrimField<String>>>,
    location: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ai_platform_floor_setting: Option<Vec<ModelArmorFloorsettingAiPlatformFloorSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_config: Option<Vec<ModelArmorFloorsettingFilterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    floor_setting_metadata: Option<Vec<ModelArmorFloorsettingFloorSettingMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_mcp_server_floor_setting:
        Option<Vec<ModelArmorFloorsettingGoogleMcpServerFloorSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ModelArmorFloorsettingTimeoutsEl>,
    dynamic: ModelArmorFloorsettingDynamic,
}
struct ModelArmorFloorsetting_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ModelArmorFloorsettingData>,
}
#[derive(Clone)]
pub struct ModelArmorFloorsetting(Rc<ModelArmorFloorsetting_>);
impl ModelArmorFloorsetting {
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
    #[doc = "Set the field `enable_floor_setting_enforcement`.\nFloor Settings enforcement status."]
    pub fn set_enable_floor_setting_enforcement(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_floor_setting_enforcement = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `integrated_services`.\nList of integrated services for which the floor setting is applicable."]
    pub fn set_integrated_services(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().integrated_services = Some(v.into());
        self
    }
    #[doc = "Set the field `ai_platform_floor_setting`.\n"]
    pub fn set_ai_platform_floor_setting(
        self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingAiPlatformFloorSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ai_platform_floor_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ai_platform_floor_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter_config`.\n"]
    pub fn set_filter_config(
        self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `floor_setting_metadata`.\n"]
    pub fn set_floor_setting_metadata(
        self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFloorSettingMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().floor_setting_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.floor_setting_metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_mcp_server_floor_setting`.\n"]
    pub fn set_google_mcp_server_floor_setting(
        self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingGoogleMcpServerFloorSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().google_mcp_server_floor_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .google_mcp_server_floor_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ModelArmorFloorsettingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_floor_setting_enforcement` after provisioning.\nFloor Settings enforcement status."]
    pub fn enable_floor_setting_enforcement(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_floor_setting_enforcement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `integrated_services` after provisioning.\nList of integrated services for which the floor setting is applicable."]
    pub fn integrated_services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integrated_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nWill be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ai_platform_floor_setting` after provisioning.\n"]
    pub fn ai_platform_floor_setting(
        &self,
    ) -> ListRef<ModelArmorFloorsettingAiPlatformFloorSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ai_platform_floor_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> ListRef<ModelArmorFloorsettingFilterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `floor_setting_metadata` after provisioning.\n"]
    pub fn floor_setting_metadata(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFloorSettingMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.floor_setting_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_mcp_server_floor_setting` after provisioning.\n"]
    pub fn google_mcp_server_floor_setting(
        &self,
    ) -> ListRef<ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_mcp_server_floor_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorFloorsettingTimeoutsElRef {
        ModelArmorFloorsettingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ModelArmorFloorsetting {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ModelArmorFloorsetting {}
impl ToListMappable for ModelArmorFloorsetting {
    type O = ListRef<ModelArmorFloorsettingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ModelArmorFloorsetting_ {
    fn extract_resource_type(&self) -> String {
        "google_model_armor_floorsetting".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildModelArmorFloorsetting {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Will be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub parent: PrimField<String>,
}
impl BuildModelArmorFloorsetting {
    pub fn build(self, stack: &mut Stack) -> ModelArmorFloorsetting {
        let out = ModelArmorFloorsetting(Rc::new(ModelArmorFloorsetting_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ModelArmorFloorsettingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable_floor_setting_enforcement: core::default::Default::default(),
                id: core::default::Default::default(),
                integrated_services: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                ai_platform_floor_setting: core::default::Default::default(),
                filter_config: core::default::Default::default(),
                floor_setting_metadata: core::default::Default::default(),
                google_mcp_server_floor_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ModelArmorFloorsettingRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ModelArmorFloorsettingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_floor_setting_enforcement` after provisioning.\nFloor Settings enforcement status."]
    pub fn enable_floor_setting_enforcement(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_floor_setting_enforcement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `integrated_services` after provisioning.\nList of integrated services for which the floor setting is applicable."]
    pub fn integrated_services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integrated_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nWill be any one of these:\n\n* 'projects/{project}'\n* 'folders/{folder}'\n* 'organizations/{organizationId}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ai_platform_floor_setting` after provisioning.\n"]
    pub fn ai_platform_floor_setting(
        &self,
    ) -> ListRef<ModelArmorFloorsettingAiPlatformFloorSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ai_platform_floor_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> ListRef<ModelArmorFloorsettingFilterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `floor_setting_metadata` after provisioning.\n"]
    pub fn floor_setting_metadata(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFloorSettingMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.floor_setting_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_mcp_server_floor_setting` after provisioning.\n"]
    pub fn google_mcp_server_floor_setting(
        &self,
    ) -> ListRef<ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_mcp_server_floor_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorFloorsettingTimeoutsElRef {
        ModelArmorFloorsettingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingAiPlatformFloorSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cloud_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_and_block: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_only: Option<PrimField<bool>>,
}
impl ModelArmorFloorsettingAiPlatformFloorSettingEl {
    #[doc = "Set the field `enable_cloud_logging`.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn set_enable_cloud_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_cloud_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_and_block`.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn set_inspect_and_block(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inspect_and_block = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_only`.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn set_inspect_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inspect_only = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingAiPlatformFloorSettingEl {
    type O = BlockAssignable<ModelArmorFloorsettingAiPlatformFloorSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingAiPlatformFloorSettingEl {}
impl BuildModelArmorFloorsettingAiPlatformFloorSettingEl {
    pub fn build(self) -> ModelArmorFloorsettingAiPlatformFloorSettingEl {
        ModelArmorFloorsettingAiPlatformFloorSettingEl {
            enable_cloud_logging: core::default::Default::default(),
            inspect_and_block: core::default::Default::default(),
            inspect_only: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingAiPlatformFloorSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingAiPlatformFloorSettingElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorFloorsettingAiPlatformFloorSettingElRef {
        ModelArmorFloorsettingAiPlatformFloorSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingAiPlatformFloorSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn enable_cloud_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cloud_logging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_and_block` after provisioning.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn inspect_and_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_and_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_only` after provisioning.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn inspect_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inspect_only", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
    #[doc = "Set the field `filter_enforcement`.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {}
impl BuildModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
        ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl {
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef {
        ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidence_level = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_enforcement`.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {}
impl BuildModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
        ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl {
            confidence_level: core::default::Default::default(),
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef {
        ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence_level: Option<PrimField<String>>,
    filter_type: PrimField<String>,
}
impl ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidence_level = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
    #[doc = "Possible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub filter_type: PrimField<String>,
}
impl BuildModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
        ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl {
            confidence_level: core::default::Default::default(),
            filter_type: self.filter_type,
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef {
        ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\nPossible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ModelArmorFloorsettingFilterConfigElRaiSettingsElDynamic {
    rai_filters:
        Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElRaiSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rai_filters: Option<Vec<ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl>>,
    dynamic: ModelArmorFloorsettingFilterConfigElRaiSettingsElDynamic,
}
impl ModelArmorFloorsettingFilterConfigElRaiSettingsEl {
    #[doc = "Set the field `rai_filters`.\n"]
    pub fn set_rai_filters(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rai_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElRaiSettingsEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElRaiSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElRaiSettingsEl {}
impl BuildModelArmorFloorsettingFilterConfigElRaiSettingsEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElRaiSettingsEl {
        ModelArmorFloorsettingFilterConfigElRaiSettingsEl {
            rai_filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElRaiSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElRaiSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElRaiSettingsElRef {
        ModelArmorFloorsettingFilterConfigElRaiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElRaiSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rai_filters` after provisioning.\n"]
    pub fn rai_filters(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFilterConfigElRaiSettingsElRaiFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rai_filters", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_template: Option<PrimField<String>>,
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
    #[doc = "Set the field `deidentify_template`.\nOptional Sensitive Data Protection Deidentify template resource name.\n\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\n\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deidentify_template = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template`.\nSensitive Data Protection inspect template resource name\n\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\n\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inspect_template = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {}
impl BuildModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
        ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl {
            deidentify_template: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef {
        ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\nOptional Sensitive Data Protection Deidentify template resource name.\n\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\n\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\nSensitive Data Protection inspect template resource name\n\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\n\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
    #[doc = "Set the field `filter_enforcement`.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {}
impl BuildModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
        ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl {
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef {
        ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ModelArmorFloorsettingFilterConfigElSdpSettingsElDynamic {
    advanced_config:
        Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    basic_config:
        Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_config: Option<Vec<ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_config: Option<Vec<ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl>>,
    dynamic: ModelArmorFloorsettingFilterConfigElSdpSettingsElDynamic,
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsEl {
    #[doc = "Set the field `advanced_config`.\n"]
    pub fn set_advanced_config(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `basic_config`.\n"]
    pub fn set_basic_config(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigElSdpSettingsEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigElSdpSettingsEl {}
impl BuildModelArmorFloorsettingFilterConfigElSdpSettingsEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigElSdpSettingsEl {
        ModelArmorFloorsettingFilterConfigElSdpSettingsEl {
            advanced_config: core::default::Default::default(),
            basic_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElSdpSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElSdpSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFilterConfigElSdpSettingsElRef {
        ModelArmorFloorsettingFilterConfigElSdpSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElSdpSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_config` after provisioning.\n"]
    pub fn advanced_config(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFilterConfigElSdpSettingsElAdvancedConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_config` after provisioning.\n"]
    pub fn basic_config(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFilterConfigElSdpSettingsElBasicConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.basic_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct ModelArmorFloorsettingFilterConfigElDynamic {
    malicious_uri_filter_settings:
        Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl>>,
    pi_and_jailbreak_filter_settings:
        Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl>>,
    rai_settings: Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElRaiSettingsEl>>,
    sdp_settings: Option<DynamicBlock<ModelArmorFloorsettingFilterConfigElSdpSettingsEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFilterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    malicious_uri_filter_settings:
        Option<Vec<ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pi_and_jailbreak_filter_settings:
        Option<Vec<ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rai_settings: Option<Vec<ModelArmorFloorsettingFilterConfigElRaiSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdp_settings: Option<Vec<ModelArmorFloorsettingFilterConfigElSdpSettingsEl>>,
    dynamic: ModelArmorFloorsettingFilterConfigElDynamic,
}
impl ModelArmorFloorsettingFilterConfigEl {
    #[doc = "Set the field `malicious_uri_filter_settings`.\n"]
    pub fn set_malicious_uri_filter_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malicious_uri_filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malicious_uri_filter_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pi_and_jailbreak_filter_settings`.\n"]
    pub fn set_pi_and_jailbreak_filter_settings(
        mut self,
        v: impl Into<
            BlockAssignable<ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pi_and_jailbreak_filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pi_and_jailbreak_filter_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rai_settings`.\n"]
    pub fn set_rai_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElRaiSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rai_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sdp_settings`.\n"]
    pub fn set_sdp_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorFloorsettingFilterConfigElSdpSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sdp_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sdp_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFilterConfigEl {
    type O = BlockAssignable<ModelArmorFloorsettingFilterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFilterConfigEl {}
impl BuildModelArmorFloorsettingFilterConfigEl {
    pub fn build(self) -> ModelArmorFloorsettingFilterConfigEl {
        ModelArmorFloorsettingFilterConfigEl {
            malicious_uri_filter_settings: core::default::Default::default(),
            pi_and_jailbreak_filter_settings: core::default::Default::default(),
            rai_settings: core::default::Default::default(),
            sdp_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFilterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFilterConfigElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorFloorsettingFilterConfigElRef {
        ModelArmorFloorsettingFilterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFilterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `malicious_uri_filter_settings` after provisioning.\n"]
    pub fn malicious_uri_filter_settings(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFilterConfigElMaliciousUriFilterSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.malicious_uri_filter_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pi_and_jailbreak_filter_settings` after provisioning.\n"]
    pub fn pi_and_jailbreak_filter_settings(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFilterConfigElPiAndJailbreakFilterSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pi_and_jailbreak_filter_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rai_settings` after provisioning.\n"]
    pub fn rai_settings(&self) -> ListRef<ModelArmorFloorsettingFilterConfigElRaiSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rai_settings", self.base))
    }
    #[doc = "Get a reference to the value of field `sdp_settings` after provisioning.\n"]
    pub fn sdp_settings(&self) -> ListRef<ModelArmorFloorsettingFilterConfigElSdpSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sdp_settings", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
    enable_multi_language_detection: PrimField<bool>,
}
impl ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {}
impl ToListMappable for ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
    type O = BlockAssignable<ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
    #[doc = "If true, multi language detection will be enabled."]
    pub enable_multi_language_detection: PrimField<bool>,
}
impl BuildModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
    pub fn build(self) -> ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
        ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl {
            enable_multi_language_detection: self.enable_multi_language_detection,
        }
    }
}
pub struct ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef {
        ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_multi_language_detection` after provisioning.\nIf true, multi language detection will be enabled."]
    pub fn enable_multi_language_detection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_detection", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ModelArmorFloorsettingFloorSettingMetadataElDynamic {
    multi_language_detection:
        Option<DynamicBlock<ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingFloorSettingMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_language_detection:
        Option<Vec<ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl>>,
    dynamic: ModelArmorFloorsettingFloorSettingMetadataElDynamic,
}
impl ModelArmorFloorsettingFloorSettingMetadataEl {
    #[doc = "Set the field `multi_language_detection`.\n"]
    pub fn set_multi_language_detection(
        mut self,
        v: impl Into<
            BlockAssignable<ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_language_detection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_language_detection = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingFloorSettingMetadataEl {
    type O = BlockAssignable<ModelArmorFloorsettingFloorSettingMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingFloorSettingMetadataEl {}
impl BuildModelArmorFloorsettingFloorSettingMetadataEl {
    pub fn build(self) -> ModelArmorFloorsettingFloorSettingMetadataEl {
        ModelArmorFloorsettingFloorSettingMetadataEl {
            multi_language_detection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingFloorSettingMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingFloorSettingMetadataElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorFloorsettingFloorSettingMetadataElRef {
        ModelArmorFloorsettingFloorSettingMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingFloorSettingMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `multi_language_detection` after provisioning.\n"]
    pub fn multi_language_detection(
        &self,
    ) -> ListRef<ModelArmorFloorsettingFloorSettingMetadataElMultiLanguageDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_language_detection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cloud_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_and_block: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_only: Option<PrimField<bool>>,
}
impl ModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
    #[doc = "Set the field `enable_cloud_logging`.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn set_enable_cloud_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_cloud_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_and_block`.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn set_inspect_and_block(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inspect_and_block = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_only`.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn set_inspect_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inspect_only = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
    type O = BlockAssignable<ModelArmorFloorsettingGoogleMcpServerFloorSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingGoogleMcpServerFloorSettingEl {}
impl BuildModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
    pub fn build(self) -> ModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
        ModelArmorFloorsettingGoogleMcpServerFloorSettingEl {
            enable_cloud_logging: core::default::Default::default(),
            inspect_and_block: core::default::Default::default(),
            inspect_only: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef {
        ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingGoogleMcpServerFloorSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nIf true, log Model Armor filter results to Cloud Logging."]
    pub fn enable_cloud_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cloud_logging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_and_block` after provisioning.\nIf true, Model Armor filters will be run in inspect and block mode.\nRequests that trip Model Armor filters will be blocked."]
    pub fn inspect_and_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_and_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_only` after provisioning.\nIf true, Model Armor filters will be run in inspect only mode. No action\nwill be taken on the request."]
    pub fn inspect_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inspect_only", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorFloorsettingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ModelArmorFloorsettingTimeoutsEl {
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
impl ToListMappable for ModelArmorFloorsettingTimeoutsEl {
    type O = BlockAssignable<ModelArmorFloorsettingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorFloorsettingTimeoutsEl {}
impl BuildModelArmorFloorsettingTimeoutsEl {
    pub fn build(self) -> ModelArmorFloorsettingTimeoutsEl {
        ModelArmorFloorsettingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorFloorsettingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorFloorsettingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorFloorsettingTimeoutsElRef {
        ModelArmorFloorsettingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorFloorsettingTimeoutsElRef {
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
struct ModelArmorFloorsettingDynamic {
    ai_platform_floor_setting: Option<DynamicBlock<ModelArmorFloorsettingAiPlatformFloorSettingEl>>,
    filter_config: Option<DynamicBlock<ModelArmorFloorsettingFilterConfigEl>>,
    floor_setting_metadata: Option<DynamicBlock<ModelArmorFloorsettingFloorSettingMetadataEl>>,
    google_mcp_server_floor_setting:
        Option<DynamicBlock<ModelArmorFloorsettingGoogleMcpServerFloorSettingEl>>,
}
