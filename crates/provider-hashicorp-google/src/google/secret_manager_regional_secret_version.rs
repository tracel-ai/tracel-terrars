use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecretManagerRegionalSecretVersionData {
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
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_secret_data_base64: Option<PrimField<bool>>,
    secret: PrimField<String>,
    secret_data: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecretManagerRegionalSecretVersionTimeoutsEl>,
}
struct SecretManagerRegionalSecretVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretManagerRegionalSecretVersionData>,
}
#[derive(Clone)]
pub struct SecretManagerRegionalSecretVersion(Rc<SecretManagerRegionalSecretVersion_>);
impl SecretManagerRegionalSecretVersion {
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
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/secret_manager_regional_regional_secret_version.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nThe current state of the regional secret version."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_secret_data_base64`.\nIf set to 'true', the secret data is expected to be base64-encoded string and would be sent as is."]
    pub fn set_is_secret_data_base64(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_secret_data_base64 = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecretManagerRegionalSecretVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional secret version was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\nThe customer-managed encryption configuration of the regional secret."]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/secret_manager_regional_regional_secret_version.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_time` after provisioning.\nThe time at which the regional secret version was destroyed. Only present if state is DESTROYED."]
    pub fn destroy_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nThe current state of the regional secret version."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_secret_data_base64` after provisioning.\nIf set to 'true', the secret data is expected to be base64-encoded string and would be sent as is."]
    pub fn is_secret_data_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_secret_data_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of Secret Manager regional secret resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional secret version. Format:\n'projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret Manager regional secret resource."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_data` after provisioning.\nThe secret data. Must be no larger than 64KiB."]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the Regional Secret."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerRegionalSecretVersionTimeoutsElRef {
        SecretManagerRegionalSecretVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SecretManagerRegionalSecretVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecretManagerRegionalSecretVersion {}
impl ToListMappable for SecretManagerRegionalSecretVersion {
    type O = ListRef<SecretManagerRegionalSecretVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecretManagerRegionalSecretVersion_ {
    fn extract_resource_type(&self) -> String {
        "google_secret_manager_regional_secret_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecretManagerRegionalSecretVersion {
    pub tf_id: String,
    #[doc = "Secret Manager regional secret resource."]
    pub secret: PrimField<String>,
    #[doc = "The secret data. Must be no larger than 64KiB."]
    pub secret_data: PrimField<String>,
}
impl BuildSecretManagerRegionalSecretVersion {
    pub fn build(self, stack: &mut Stack) -> SecretManagerRegionalSecretVersion {
        let out =
            SecretManagerRegionalSecretVersion(Rc::new(SecretManagerRegionalSecretVersion_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SecretManagerRegionalSecretVersionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    enabled: core::default::Default::default(),
                    id: core::default::Default::default(),
                    is_secret_data_base64: core::default::Default::default(),
                    secret: self.secret,
                    secret_data: self.secret_data,
                    timeouts: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecretManagerRegionalSecretVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecretManagerRegionalSecretVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional secret version was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\nThe customer-managed encryption configuration of the regional secret."]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/secret_manager_regional_regional_secret_version.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_time` after provisioning.\nThe time at which the regional secret version was destroyed. Only present if state is DESTROYED."]
    pub fn destroy_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nThe current state of the regional secret version."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_secret_data_base64` after provisioning.\nIf set to 'true', the secret data is expected to be base64-encoded string and would be sent as is."]
    pub fn is_secret_data_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_secret_data_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of Secret Manager regional secret resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional secret version. Format:\n'projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret Manager regional secret resource."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_data` after provisioning.\nThe secret data. Must be no larger than 64KiB."]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the Regional Secret."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerRegionalSecretVersionTimeoutsElRef {
        SecretManagerRegionalSecretVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_version_name: Option<PrimField<String>>,
}
impl SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_version_name`.\n"]
    pub fn set_kms_key_version_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_version_name = Some(v.into());
        self
    }
}
impl ToListMappable for SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    type O = BlockAssignable<SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {}
impl BuildSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    pub fn build(self) -> SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
        SecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
            kms_key_version_name: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
        SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_version_name` after provisioning.\n"]
    pub fn kms_key_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecretManagerRegionalSecretVersionTimeoutsEl {
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
impl ToListMappable for SecretManagerRegionalSecretVersionTimeoutsEl {
    type O = BlockAssignable<SecretManagerRegionalSecretVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretVersionTimeoutsEl {}
impl BuildSecretManagerRegionalSecretVersionTimeoutsEl {
    pub fn build(self) -> SecretManagerRegionalSecretVersionTimeoutsEl {
        SecretManagerRegionalSecretVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerRegionalSecretVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretVersionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerRegionalSecretVersionTimeoutsElRef {
        SecretManagerRegionalSecretVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretVersionTimeoutsElRef {
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
