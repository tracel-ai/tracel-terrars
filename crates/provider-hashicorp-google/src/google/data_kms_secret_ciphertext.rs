use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsSecretCiphertextData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    crypto_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    plaintext: PrimField<String>,
}
struct DataKmsSecretCiphertext_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsSecretCiphertextData>,
}
#[derive(Clone)]
pub struct DataKmsSecretCiphertext(Rc<DataKmsSecretCiphertext_>);
impl DataKmsSecretCiphertext {
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
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\n"]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\n"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
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
impl Referable for DataKmsSecretCiphertext {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsSecretCiphertext {}
impl ToListMappable for DataKmsSecretCiphertext {
    type O = ListRef<DataKmsSecretCiphertextRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsSecretCiphertext_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_secret_ciphertext".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsSecretCiphertext {
    pub tf_id: String,
    #[doc = ""]
    pub crypto_key: PrimField<String>,
    #[doc = ""]
    pub plaintext: PrimField<String>,
}
impl BuildDataKmsSecretCiphertext {
    pub fn build(self, stack: &mut Stack) -> DataKmsSecretCiphertext {
        let out = DataKmsSecretCiphertext(Rc::new(DataKmsSecretCiphertext_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsSecretCiphertextData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                crypto_key: self.crypto_key,
                id: core::default::Default::default(),
                plaintext: self.plaintext,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsSecretCiphertextRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsSecretCiphertextRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsSecretCiphertextRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\n"]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\n"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
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
