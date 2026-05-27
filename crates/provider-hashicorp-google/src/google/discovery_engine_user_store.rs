use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineUserStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_license_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_expired_license_auto_update: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_license_auto_register: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineUserStoreTimeoutsEl>,
}
struct DiscoveryEngineUserStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineUserStoreData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineUserStore(Rc<DiscoveryEngineUserStore_>);
impl DiscoveryEngineUserStore {
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
    #[doc = "Set the field `default_license_config`.\nThe resource name of the default license config assigned to users created in\nthis user store. Format:\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'.\nIf 'enableLicenseAutoRegister' is true, new users will automatically\nregister under the default subscription.\nIf the default license config doesn't have remaining license seats left,\nnew users will not be assigned with license."]
    pub fn set_default_license_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_license_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_expired_license_auto_update`.\nWhether to enable automatic license update for users with expired licenses\nin this user store. If enabled, users with expired licenses will\nautomatically be updated to the default subscription if there are\nremaining license seats."]
    pub fn set_enable_expired_license_auto_update(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_expired_license_auto_update = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_license_auto_register`.\nWhether to enable automatic license registration for new users created in\nthis user store. If enabled, new users will automatically register under\nthe default subscription."]
    pub fn set_enable_license_auto_register(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_license_auto_register = Some(v.into());
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
    #[doc = "Set the field `user_store_id`.\nThe ID of the user store. Currently only accepts \"default_user_store\"."]
    pub fn set_user_store_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_store_id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineUserStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `default_license_config` after provisioning.\nThe resource name of the default license config assigned to users created in\nthis user store. Format:\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'.\nIf 'enableLicenseAutoRegister' is true, new users will automatically\nregister under the default subscription.\nIf the default license config doesn't have remaining license seats left,\nnew users will not be assigned with license."]
    pub fn default_license_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_license_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_expired_license_auto_update` after provisioning.\nWhether to enable automatic license update for users with expired licenses\nin this user store. If enabled, users with expired licenses will\nautomatically be updated to the default subscription if there are\nremaining license seats."]
    pub fn enable_expired_license_auto_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_expired_license_auto_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_license_auto_register` after provisioning.\nWhether to enable automatic license registration for new users created in\nthis user store. If enabled, new users will automatically register under\nthe default subscription."]
    pub fn enable_license_auto_register(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_license_auto_register", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the user store. Values are of the format\n'projects/{project}/locations/{location}/userStores/{user_store_id}'."]
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
    #[doc = "Get a reference to the value of field `user_store_id` after provisioning.\nThe ID of the user store. Currently only accepts \"default_user_store\"."]
    pub fn user_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineUserStoreTimeoutsElRef {
        DiscoveryEngineUserStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineUserStore {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineUserStore {}
impl ToListMappable for DiscoveryEngineUserStore {
    type O = ListRef<DiscoveryEngineUserStoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineUserStore_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_user_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineUserStore {
    pub tf_id: String,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineUserStore {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineUserStore {
        let out = DiscoveryEngineUserStore(Rc::new(DiscoveryEngineUserStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineUserStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_license_config: core::default::Default::default(),
                enable_expired_license_auto_update: core::default::Default::default(),
                enable_license_auto_register: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                user_store_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineUserStoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineUserStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineUserStoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_license_config` after provisioning.\nThe resource name of the default license config assigned to users created in\nthis user store. Format:\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'.\nIf 'enableLicenseAutoRegister' is true, new users will automatically\nregister under the default subscription.\nIf the default license config doesn't have remaining license seats left,\nnew users will not be assigned with license."]
    pub fn default_license_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_license_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_expired_license_auto_update` after provisioning.\nWhether to enable automatic license update for users with expired licenses\nin this user store. If enabled, users with expired licenses will\nautomatically be updated to the default subscription if there are\nremaining license seats."]
    pub fn enable_expired_license_auto_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_expired_license_auto_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_license_auto_register` after provisioning.\nWhether to enable automatic license registration for new users created in\nthis user store. If enabled, new users will automatically register under\nthe default subscription."]
    pub fn enable_license_auto_register(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_license_auto_register", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the user store. Values are of the format\n'projects/{project}/locations/{location}/userStores/{user_store_id}'."]
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
    #[doc = "Get a reference to the value of field `user_store_id` after provisioning.\nThe ID of the user store. Currently only accepts \"default_user_store\"."]
    pub fn user_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineUserStoreTimeoutsElRef {
        DiscoveryEngineUserStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineUserStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineUserStoreTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineUserStoreTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineUserStoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineUserStoreTimeoutsEl {}
impl BuildDiscoveryEngineUserStoreTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineUserStoreTimeoutsEl {
        DiscoveryEngineUserStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineUserStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineUserStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineUserStoreTimeoutsElRef {
        DiscoveryEngineUserStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineUserStoreTimeoutsElRef {
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
