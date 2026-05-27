use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppCheckDeviceCheckConfigData {
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
    id: Option<PrimField<String>>,
    key_id: PrimField<String>,
    private_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppCheckDeviceCheckConfigTimeoutsEl>,
}
struct FirebaseAppCheckDeviceCheckConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppCheckDeviceCheckConfigData>,
}
#[derive(Clone)]
pub struct FirebaseAppCheckDeviceCheckConfig(Rc<FirebaseAppCheckDeviceCheckConfig_>);
impl FirebaseAppCheckDeviceCheckConfig {
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
    #[doc = "Set the field `token_ttl`.\nSpecifies the duration for which App Check tokens exchanged from DeviceCheck artifacts will be valid.\nIf unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_token_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppCheckDeviceCheckConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe ID of an\n[Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id)."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the DeviceCheck configuration object"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nThe contents of the private key (.p8) file associated with the key specified by keyId."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_key_set` after provisioning.\nWhether the privateKey field was previously set. Since App Check will never return the\nprivateKey field, this field is the only way to find out whether it was previously set."]
    pub fn private_key_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token_ttl` after provisioning.\nSpecifies the duration for which App Check tokens exchanged from DeviceCheck artifacts will be valid.\nIf unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn token_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
        FirebaseAppCheckDeviceCheckConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppCheckDeviceCheckConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppCheckDeviceCheckConfig {}
impl ToListMappable for FirebaseAppCheckDeviceCheckConfig {
    type O = ListRef<FirebaseAppCheckDeviceCheckConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppCheckDeviceCheckConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_check_device_check_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppCheckDeviceCheckConfig {
    pub tf_id: String,
    #[doc = "The ID of an\n[Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id)."]
    pub app_id: PrimField<String>,
    #[doc = "The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account."]
    pub key_id: PrimField<String>,
    #[doc = "The contents of the private key (.p8) file associated with the key specified by keyId."]
    pub private_key: PrimField<String>,
}
impl BuildFirebaseAppCheckDeviceCheckConfig {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppCheckDeviceCheckConfig {
        let out = FirebaseAppCheckDeviceCheckConfig(Rc::new(FirebaseAppCheckDeviceCheckConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppCheckDeviceCheckConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_id: self.app_id,
                id: core::default::Default::default(),
                key_id: self.key_id,
                private_key: self.private_key,
                project: core::default::Default::default(),
                token_ttl: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppCheckDeviceCheckConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppCheckDeviceCheckConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppCheckDeviceCheckConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe ID of an\n[Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id)."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the DeviceCheck configuration object"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nThe contents of the private key (.p8) file associated with the key specified by keyId."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_key_set` after provisioning.\nWhether the privateKey field was previously set. Since App Check will never return the\nprivateKey field, this field is the only way to find out whether it was previously set."]
    pub fn private_key_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token_ttl` after provisioning.\nSpecifies the duration for which App Check tokens exchanged from DeviceCheck artifacts will be valid.\nIf unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn token_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
        FirebaseAppCheckDeviceCheckConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppCheckDeviceCheckConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppCheckDeviceCheckConfigTimeoutsEl {
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
impl ToListMappable for FirebaseAppCheckDeviceCheckConfigTimeoutsEl {
    type O = BlockAssignable<FirebaseAppCheckDeviceCheckConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppCheckDeviceCheckConfigTimeoutsEl {}
impl BuildFirebaseAppCheckDeviceCheckConfigTimeoutsEl {
    pub fn build(self) -> FirebaseAppCheckDeviceCheckConfigTimeoutsEl {
        FirebaseAppCheckDeviceCheckConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
        FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppCheckDeviceCheckConfigTimeoutsElRef {
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
