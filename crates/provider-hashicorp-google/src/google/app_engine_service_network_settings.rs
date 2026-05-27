use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineServiceNetworkSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_settings: Option<Vec<AppEngineServiceNetworkSettingsNetworkSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineServiceNetworkSettingsTimeoutsEl>,
    dynamic: AppEngineServiceNetworkSettingsDynamic,
}
struct AppEngineServiceNetworkSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineServiceNetworkSettingsData>,
}
#[derive(Clone)]
pub struct AppEngineServiceNetworkSettings(Rc<AppEngineServiceNetworkSettings_>);
impl AppEngineServiceNetworkSettings {
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
    #[doc = "Set the field `network_settings`.\n"]
    pub fn set_network_settings(
        self,
        v: impl Into<BlockAssignable<AppEngineServiceNetworkSettingsNetworkSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppEngineServiceNetworkSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_settings` after provisioning.\n"]
    pub fn network_settings(&self) -> ListRef<AppEngineServiceNetworkSettingsNetworkSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceNetworkSettingsTimeoutsElRef {
        AppEngineServiceNetworkSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineServiceNetworkSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineServiceNetworkSettings {}
impl ToListMappable for AppEngineServiceNetworkSettings {
    type O = ListRef<AppEngineServiceNetworkSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineServiceNetworkSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_service_network_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineServiceNetworkSettings {
    pub tf_id: String,
    #[doc = "The name of the service these settings apply to."]
    pub service: PrimField<String>,
}
impl BuildAppEngineServiceNetworkSettings {
    pub fn build(self, stack: &mut Stack) -> AppEngineServiceNetworkSettings {
        let out = AppEngineServiceNetworkSettings(Rc::new(AppEngineServiceNetworkSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppEngineServiceNetworkSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                service: self.service,
                network_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineServiceNetworkSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceNetworkSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineServiceNetworkSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_settings` after provisioning.\n"]
    pub fn network_settings(&self) -> ListRef<AppEngineServiceNetworkSettingsNetworkSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceNetworkSettingsTimeoutsElRef {
        AppEngineServiceNetworkSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineServiceNetworkSettingsNetworkSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_traffic_allowed: Option<PrimField<String>>,
}
impl AppEngineServiceNetworkSettingsNetworkSettingsEl {
    #[doc = "Set the field `ingress_traffic_allowed`.\nThe ingress settings for version or service. Default value: \"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\" Possible values: [\"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\", \"INGRESS_TRAFFIC_ALLOWED_ALL\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB\"]"]
    pub fn set_ingress_traffic_allowed(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingress_traffic_allowed = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineServiceNetworkSettingsNetworkSettingsEl {
    type O = BlockAssignable<AppEngineServiceNetworkSettingsNetworkSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineServiceNetworkSettingsNetworkSettingsEl {}
impl BuildAppEngineServiceNetworkSettingsNetworkSettingsEl {
    pub fn build(self) -> AppEngineServiceNetworkSettingsNetworkSettingsEl {
        AppEngineServiceNetworkSettingsNetworkSettingsEl {
            ingress_traffic_allowed: core::default::Default::default(),
        }
    }
}
pub struct AppEngineServiceNetworkSettingsNetworkSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceNetworkSettingsNetworkSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppEngineServiceNetworkSettingsNetworkSettingsElRef {
        AppEngineServiceNetworkSettingsNetworkSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineServiceNetworkSettingsNetworkSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingress_traffic_allowed` after provisioning.\nThe ingress settings for version or service. Default value: \"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\" Possible values: [\"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\", \"INGRESS_TRAFFIC_ALLOWED_ALL\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB\"]"]
    pub fn ingress_traffic_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_traffic_allowed", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineServiceNetworkSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineServiceNetworkSettingsTimeoutsEl {
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
impl ToListMappable for AppEngineServiceNetworkSettingsTimeoutsEl {
    type O = BlockAssignable<AppEngineServiceNetworkSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineServiceNetworkSettingsTimeoutsEl {}
impl BuildAppEngineServiceNetworkSettingsTimeoutsEl {
    pub fn build(self) -> AppEngineServiceNetworkSettingsTimeoutsEl {
        AppEngineServiceNetworkSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineServiceNetworkSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceNetworkSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineServiceNetworkSettingsTimeoutsElRef {
        AppEngineServiceNetworkSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineServiceNetworkSettingsTimeoutsElRef {
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
struct AppEngineServiceNetworkSettingsDynamic {
    network_settings: Option<DynamicBlock<AppEngineServiceNetworkSettingsNetworkSettingsEl>>,
}
