use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsSecretAsymmetricData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    ciphertext: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32: Option<PrimField<String>>,
    crypto_key_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataKmsSecretAsymmetric_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsSecretAsymmetricData>,
}
#[derive(Clone)]
pub struct DataKmsSecretAsymmetric(Rc<DataKmsSecretAsymmetric_>);
impl DataKmsSecretAsymmetric {
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
    #[doc = "Set the field `crc32`.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn set_crc32(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().crc32 = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\nThe public key encrypted ciphertext in base64 encoding"]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32` after provisioning.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn crc32(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_version` after provisioning.\nThe fully qualified KMS crypto key version name"]
    pub fn crypto_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plaintext", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsSecretAsymmetric {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsSecretAsymmetric {}
impl ToListMappable for DataKmsSecretAsymmetric {
    type O = ListRef<DataKmsSecretAsymmetricRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsSecretAsymmetric_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_secret_asymmetric".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsSecretAsymmetric {
    pub tf_id: String,
    #[doc = "The public key encrypted ciphertext in base64 encoding"]
    pub ciphertext: PrimField<String>,
    #[doc = "The fully qualified KMS crypto key version name"]
    pub crypto_key_version: PrimField<String>,
}
impl BuildDataKmsSecretAsymmetric {
    pub fn build(self, stack: &mut Stack) -> DataKmsSecretAsymmetric {
        let out = DataKmsSecretAsymmetric(Rc::new(DataKmsSecretAsymmetric_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsSecretAsymmetricData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                ciphertext: self.ciphertext,
                crc32: core::default::Default::default(),
                crypto_key_version: self.crypto_key_version,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsSecretAsymmetricRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsSecretAsymmetricRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsSecretAsymmetricRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\nThe public key encrypted ciphertext in base64 encoding"]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32` after provisioning.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn crc32(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_version` after provisioning.\nThe fully qualified KMS crypto key version name"]
    pub fn crypto_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plaintext", self.extract_ref()),
        )
    }
}
