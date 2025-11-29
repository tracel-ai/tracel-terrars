use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspaceswebUserSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_encryption_context: Option<RecField<PrimField<String>>>,
    copy_allowed: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deep_link_allowed: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disconnect_timeout_in_minutes: Option<PrimField<f64>>,
    download_allowed: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_disconnect_timeout_in_minutes: Option<PrimField<f64>>,
    paste_allowed: PrimField<String>,
    print_allowed: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    upload_allowed: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_synchronization_configuration: Option<Vec<WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolbar_configuration: Option<Vec<WorkspaceswebUserSettingsToolbarConfigurationEl>>,
    dynamic: WorkspaceswebUserSettingsDynamic,
}

struct WorkspaceswebUserSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspaceswebUserSettingsData>,
}

#[derive(Clone)]
pub struct WorkspaceswebUserSettings(Rc<WorkspaceswebUserSettings_>);

impl WorkspaceswebUserSettings {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = Some(v.into());
        self
    }

    #[doc = "Set the field `deep_link_allowed`.\n"]
    pub fn set_deep_link_allowed(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deep_link_allowed = Some(v.into());
        self
    }

    #[doc = "Set the field `disconnect_timeout_in_minutes`.\n"]
    pub fn set_disconnect_timeout_in_minutes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disconnect_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `idle_disconnect_timeout_in_minutes`.\n"]
    pub fn set_idle_disconnect_timeout_in_minutes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_disconnect_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_synchronization_configuration`.\n"]
    pub fn set_cookie_synchronization_configuration(
        self,
        v: impl Into<BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cookie_synchronization_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cookie_synchronization_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `toolbar_configuration`.\n"]
    pub fn set_toolbar_configuration(
        self,
        v: impl Into<BlockAssignable<WorkspaceswebUserSettingsToolbarConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().toolbar_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.toolbar_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_allowed` after provisioning.\n"]
    pub fn copy_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deep_link_allowed` after provisioning.\n"]
    pub fn deep_link_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deep_link_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn disconnect_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disconnect_timeout_in_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `download_allowed` after provisioning.\n"]
    pub fn download_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.download_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_disconnect_timeout_in_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paste_allowed` after provisioning.\n"]
    pub fn paste_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.paste_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `print_allowed` after provisioning.\n"]
    pub fn print_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.print_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upload_allowed` after provisioning.\n"]
    pub fn upload_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings_arn` after provisioning.\n"]
    pub fn user_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookie_synchronization_configuration` after provisioning.\n"]
    pub fn cookie_synchronization_configuration(
        &self,
    ) -> ListRef<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookie_synchronization_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `toolbar_configuration` after provisioning.\n"]
    pub fn toolbar_configuration(&self) -> ListRef<WorkspaceswebUserSettingsToolbarConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolbar_configuration", self.extract_ref()))
    }
}

impl Referable for WorkspaceswebUserSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WorkspaceswebUserSettings { }

impl ToListMappable for WorkspaceswebUserSettings {
    type O = ListRef<WorkspaceswebUserSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorkspaceswebUserSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspacesweb_user_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspaceswebUserSettings {
    pub tf_id: String,
    #[doc = ""]
    pub copy_allowed: PrimField<String>,
    #[doc = ""]
    pub download_allowed: PrimField<String>,
    #[doc = ""]
    pub paste_allowed: PrimField<String>,
    #[doc = ""]
    pub print_allowed: PrimField<String>,
    #[doc = ""]
    pub upload_allowed: PrimField<String>,
}

impl BuildWorkspaceswebUserSettings {
    pub fn build(self, stack: &mut Stack) -> WorkspaceswebUserSettings {
        let out = WorkspaceswebUserSettings(Rc::new(WorkspaceswebUserSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkspaceswebUserSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_encryption_context: core::default::Default::default(),
                copy_allowed: self.copy_allowed,
                customer_managed_key: core::default::Default::default(),
                deep_link_allowed: core::default::Default::default(),
                disconnect_timeout_in_minutes: core::default::Default::default(),
                download_allowed: self.download_allowed,
                idle_disconnect_timeout_in_minutes: core::default::Default::default(),
                paste_allowed: self.paste_allowed,
                print_allowed: self.print_allowed,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                upload_allowed: self.upload_allowed,
                cookie_synchronization_configuration: core::default::Default::default(),
                toolbar_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspaceswebUserSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebUserSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl WorkspaceswebUserSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_allowed` after provisioning.\n"]
    pub fn copy_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deep_link_allowed` after provisioning.\n"]
    pub fn deep_link_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deep_link_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn disconnect_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disconnect_timeout_in_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `download_allowed` after provisioning.\n"]
    pub fn download_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.download_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_disconnect_timeout_in_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paste_allowed` after provisioning.\n"]
    pub fn paste_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.paste_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `print_allowed` after provisioning.\n"]
    pub fn print_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.print_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upload_allowed` after provisioning.\n"]
    pub fn upload_allowed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings_arn` after provisioning.\n"]
    pub fn user_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookie_synchronization_configuration` after provisioning.\n"]
    pub fn cookie_synchronization_configuration(
        &self,
    ) -> ListRef<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookie_synchronization_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `toolbar_configuration` after provisioning.\n"]
    pub fn toolbar_configuration(&self) -> ListRef<WorkspaceswebUserSettingsToolbarConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolbar_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
    type O = BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
    #[doc = ""]
    pub domain: PrimField<String>,
}

impl BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
    pub fn build(self) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl {
            domain: self.domain,
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
    type O = BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
    #[doc = ""]
    pub domain: PrimField<String>,
}

impl BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
    pub fn build(self) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl {
            domain: self.domain,
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElDynamic {
    allowlist: Option<DynamicBlock<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl>>,
    blocklist: Option<DynamicBlock<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowlist: Option<Vec<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocklist: Option<Vec<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl>>,
    dynamic: WorkspaceswebUserSettingsCookieSynchronizationConfigurationElDynamic,
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
    #[doc = "Set the field `allowlist`.\n"]
    pub fn set_allowlist(
        mut self,
        v: impl Into<BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowlist = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowlist = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blocklist`.\n"]
    pub fn set_blocklist(
        mut self,
        v: impl Into<BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.blocklist = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.blocklist = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
    type O = BlockAssignable<WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {}

impl BuildWorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
    pub fn build(self) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl {
            allowlist: core::default::Default::default(),
            blocklist: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef {
        WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettingsCookieSynchronizationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowlist` after provisioning.\n"]
    pub fn allowlist(&self) -> ListRef<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElAllowlistElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowlist", self.base))
    }

    #[doc = "Get a reference to the value of field `blocklist` after provisioning.\n"]
    pub fn blocklist(&self) -> ListRef<WorkspaceswebUserSettingsCookieSynchronizationConfigurationElBlocklistElRef> {
        ListRef::new(self.shared().clone(), format!("{}.blocklist", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettingsToolbarConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_toolbar_items: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_display_resolution: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolbar_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visual_mode: Option<PrimField<String>>,
}

impl WorkspaceswebUserSettingsToolbarConfigurationEl {
    #[doc = "Set the field `hidden_toolbar_items`.\n"]
    pub fn set_hidden_toolbar_items(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.hidden_toolbar_items = Some(v.into());
        self
    }

    #[doc = "Set the field `max_display_resolution`.\n"]
    pub fn set_max_display_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_display_resolution = Some(v.into());
        self
    }

    #[doc = "Set the field `toolbar_type`.\n"]
    pub fn set_toolbar_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.toolbar_type = Some(v.into());
        self
    }

    #[doc = "Set the field `visual_mode`.\n"]
    pub fn set_visual_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visual_mode = Some(v.into());
        self
    }
}

impl ToListMappable for WorkspaceswebUserSettingsToolbarConfigurationEl {
    type O = BlockAssignable<WorkspaceswebUserSettingsToolbarConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettingsToolbarConfigurationEl {}

impl BuildWorkspaceswebUserSettingsToolbarConfigurationEl {
    pub fn build(self) -> WorkspaceswebUserSettingsToolbarConfigurationEl {
        WorkspaceswebUserSettingsToolbarConfigurationEl {
            hidden_toolbar_items: core::default::Default::default(),
            max_display_resolution: core::default::Default::default(),
            toolbar_type: core::default::Default::default(),
            visual_mode: core::default::Default::default(),
        }
    }
}

pub struct WorkspaceswebUserSettingsToolbarConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebUserSettingsToolbarConfigurationElRef {
    fn new(shared: StackShared, base: String) -> WorkspaceswebUserSettingsToolbarConfigurationElRef {
        WorkspaceswebUserSettingsToolbarConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettingsToolbarConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_toolbar_items` after provisioning.\n"]
    pub fn hidden_toolbar_items(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hidden_toolbar_items", self.base))
    }

    #[doc = "Get a reference to the value of field `max_display_resolution` after provisioning.\n"]
    pub fn max_display_resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_display_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `toolbar_type` after provisioning.\n"]
    pub fn toolbar_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolbar_type", self.base))
    }

    #[doc = "Get a reference to the value of field `visual_mode` after provisioning.\n"]
    pub fn visual_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visual_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebUserSettingsDynamic {
    cookie_synchronization_configuration: Option<
        DynamicBlock<WorkspaceswebUserSettingsCookieSynchronizationConfigurationEl>,
    >,
    toolbar_configuration: Option<DynamicBlock<WorkspaceswebUserSettingsToolbarConfigurationEl>>,
}
