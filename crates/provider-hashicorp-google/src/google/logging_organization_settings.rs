use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LoggingOrganizationSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_default_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LoggingOrganizationSettingsTimeoutsEl>,
}
struct LoggingOrganizationSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingOrganizationSettingsData>,
}
#[derive(Clone)]
pub struct LoggingOrganizationSettings(Rc<LoggingOrganizationSettings_>);
impl LoggingOrganizationSettings {
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
    #[doc = "Set the field `disable_default_sink`.\nIf set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed."]
    pub fn set_disable_default_sink(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_default_sink = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe resource name for the configured Cloud KMS key."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_location`.\nThe storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided."]
    pub fn set_storage_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_location = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LoggingOrganizationSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `disable_default_sink` after provisioning.\nIf set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed."]
    pub fn disable_default_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_default_sink", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name for the configured Cloud KMS key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_service_account_id` after provisioning.\nThe service account that will be used by the Log Router to access your Cloud KMS key."]
    pub fn kms_service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_service_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_service_account_id` after provisioning.\nThe service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided."]
    pub fn logging_service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_service_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe organization for which to retrieve or configure settings."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\nThe storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided."]
    pub fn storage_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingOrganizationSettingsTimeoutsElRef {
        LoggingOrganizationSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LoggingOrganizationSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LoggingOrganizationSettings {}
impl ToListMappable for LoggingOrganizationSettings {
    type O = ListRef<LoggingOrganizationSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LoggingOrganizationSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_organization_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLoggingOrganizationSettings {
    pub tf_id: String,
    #[doc = "The organization for which to retrieve or configure settings."]
    pub organization: PrimField<String>,
}
impl BuildLoggingOrganizationSettings {
    pub fn build(self, stack: &mut Stack) -> LoggingOrganizationSettings {
        let out = LoggingOrganizationSettings(Rc::new(LoggingOrganizationSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingOrganizationSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                disable_default_sink: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                organization: self.organization,
                storage_location: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LoggingOrganizationSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingOrganizationSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LoggingOrganizationSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_default_sink` after provisioning.\nIf set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed."]
    pub fn disable_default_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_default_sink", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name for the configured Cloud KMS key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_service_account_id` after provisioning.\nThe service account that will be used by the Log Router to access your Cloud KMS key."]
    pub fn kms_service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_service_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_service_account_id` after provisioning.\nThe service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided."]
    pub fn logging_service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_service_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe organization for which to retrieve or configure settings."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\nThe storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided."]
    pub fn storage_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingOrganizationSettingsTimeoutsElRef {
        LoggingOrganizationSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingOrganizationSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LoggingOrganizationSettingsTimeoutsEl {
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
impl ToListMappable for LoggingOrganizationSettingsTimeoutsEl {
    type O = BlockAssignable<LoggingOrganizationSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingOrganizationSettingsTimeoutsEl {}
impl BuildLoggingOrganizationSettingsTimeoutsEl {
    pub fn build(self) -> LoggingOrganizationSettingsTimeoutsEl {
        LoggingOrganizationSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LoggingOrganizationSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingOrganizationSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LoggingOrganizationSettingsTimeoutsElRef {
        LoggingOrganizationSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingOrganizationSettingsTimeoutsElRef {
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
