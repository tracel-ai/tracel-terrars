use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLoggingProjectCmekSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    project: PrimField<String>,
}
struct DataLoggingProjectCmekSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLoggingProjectCmekSettingsData>,
}
#[derive(Clone)]
pub struct DataLoggingProjectCmekSettings(Rc<DataLoggingProjectCmekSettings_>);
impl DataLoggingProjectCmekSettings {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe resource name for the configured Cloud KMS key.\n\t\t\t\tKMS key name format:\n\t\t\t\t\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]\"\n\t\t\t\tTo enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.\n\t\t\t\tThe Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.\n\t\t\t\tSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name for the configured Cloud KMS key.\n\t\t\t\tKMS key name format:\n\t\t\t\t\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]\"\n\t\t\t\tTo enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.\n\t\t\t\tThe Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.\n\t\t\t\tSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_version_name` after provisioning.\nThe CryptoKeyVersion resource name for the configured Cloud KMS key.\n\t\t\t\tKMS key name format:\n\t\t\t\t\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]\"\n\t\t\t\tFor example:\n\t\t\t\t\"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1\"\n\t\t\t\tThis is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version."]
    pub fn kms_key_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the CMEK settings."]
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
    #[doc = "Get a reference to the value of field `service_account_id` after provisioning.\nThe service account associated with a project for which CMEK will apply.\n\t\t\t\tBefore enabling CMEK for a logging bucket, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account associated with the project for which CMEK will apply. Use [v2.getCmekSettings](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getCmekSettings#google.logging.v2.ConfigServiceV2.GetCmekSettings) to obtain the service account ID.\n\t\t\t\tSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_id", self.extract_ref()),
        )
    }
}
impl Referable for DataLoggingProjectCmekSettings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLoggingProjectCmekSettings {}
impl ToListMappable for DataLoggingProjectCmekSettings {
    type O = ListRef<DataLoggingProjectCmekSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLoggingProjectCmekSettings_ {
    fn extract_datasource_type(&self) -> String {
        "google_logging_project_cmek_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLoggingProjectCmekSettings {
    pub tf_id: String,
    #[doc = ""]
    pub project: PrimField<String>,
}
impl BuildDataLoggingProjectCmekSettings {
    pub fn build(self, stack: &mut Stack) -> DataLoggingProjectCmekSettings {
        let out = DataLoggingProjectCmekSettings(Rc::new(DataLoggingProjectCmekSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLoggingProjectCmekSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLoggingProjectCmekSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLoggingProjectCmekSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLoggingProjectCmekSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name for the configured Cloud KMS key.\n\t\t\t\tKMS key name format:\n\t\t\t\t\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]\"\n\t\t\t\tTo enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.\n\t\t\t\tThe Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.\n\t\t\t\tSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_version_name` after provisioning.\nThe CryptoKeyVersion resource name for the configured Cloud KMS key.\n\t\t\t\tKMS key name format:\n\t\t\t\t\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]\"\n\t\t\t\tFor example:\n\t\t\t\t\"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1\"\n\t\t\t\tThis is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version."]
    pub fn kms_key_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the CMEK settings."]
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
    #[doc = "Get a reference to the value of field `service_account_id` after provisioning.\nThe service account associated with a project for which CMEK will apply.\n\t\t\t\tBefore enabling CMEK for a logging bucket, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account associated with the project for which CMEK will apply. Use [v2.getCmekSettings](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getCmekSettings#google.logging.v2.ConfigServiceV2.GetCmekSettings) to obtain the service account ID.\n\t\t\t\tSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_id", self.extract_ref()),
        )
    }
}
