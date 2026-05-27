use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KmsSecretCiphertextData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_authenticated_data: Option<PrimField<String>>,
    crypto_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    plaintext: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsSecretCiphertextTimeoutsEl>,
}
struct KmsSecretCiphertext_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsSecretCiphertextData>,
}
#[derive(Clone)]
pub struct KmsSecretCiphertext(Rc<KmsSecretCiphertext_>);
impl KmsSecretCiphertext {
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
    #[doc = "Set the field `additional_authenticated_data`.\nThe additional authenticated data used for integrity checks during encryption and decryption."]
    pub fn set_additional_authenticated_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().additional_authenticated_data = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsSecretCiphertextTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_authenticated_data` after provisioning.\nThe additional authenticated data used for integrity checks during encryption and decryption."]
    pub fn additional_authenticated_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_authenticated_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\nContains the result of encrypting the provided plaintext, encoded in base64."]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\nThe full name of the CryptoKey that will be used to encrypt the provided plaintext.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}''"]
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
    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\nThe plaintext to be encrypted."]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plaintext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsSecretCiphertextTimeoutsElRef {
        KmsSecretCiphertextTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for KmsSecretCiphertext {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KmsSecretCiphertext {}
impl ToListMappable for KmsSecretCiphertext {
    type O = ListRef<KmsSecretCiphertextRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KmsSecretCiphertext_ {
    fn extract_resource_type(&self) -> String {
        "google_kms_secret_ciphertext".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKmsSecretCiphertext {
    pub tf_id: String,
    #[doc = "The full name of the CryptoKey that will be used to encrypt the provided plaintext.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}''"]
    pub crypto_key: PrimField<String>,
    #[doc = "The plaintext to be encrypted."]
    pub plaintext: PrimField<String>,
}
impl BuildKmsSecretCiphertext {
    pub fn build(self, stack: &mut Stack) -> KmsSecretCiphertext {
        let out = KmsSecretCiphertext(Rc::new(KmsSecretCiphertext_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsSecretCiphertextData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_authenticated_data: core::default::Default::default(),
                crypto_key: self.crypto_key,
                id: core::default::Default::default(),
                plaintext: self.plaintext,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KmsSecretCiphertextRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsSecretCiphertextRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KmsSecretCiphertextRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_authenticated_data` after provisioning.\nThe additional authenticated data used for integrity checks during encryption and decryption."]
    pub fn additional_authenticated_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_authenticated_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ciphertext` after provisioning.\nContains the result of encrypting the provided plaintext, encoded in base64."]
    pub fn ciphertext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ciphertext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\nThe full name of the CryptoKey that will be used to encrypt the provided plaintext.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}''"]
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
    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\nThe plaintext to be encrypted."]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plaintext", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsSecretCiphertextTimeoutsElRef {
        KmsSecretCiphertextTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct KmsSecretCiphertextTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl KmsSecretCiphertextTimeoutsEl {
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
}
impl ToListMappable for KmsSecretCiphertextTimeoutsEl {
    type O = BlockAssignable<KmsSecretCiphertextTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsSecretCiphertextTimeoutsEl {}
impl BuildKmsSecretCiphertextTimeoutsEl {
    pub fn build(self) -> KmsSecretCiphertextTimeoutsEl {
        KmsSecretCiphertextTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct KmsSecretCiphertextTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsSecretCiphertextTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsSecretCiphertextTimeoutsElRef {
        KmsSecretCiphertextTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsSecretCiphertextTimeoutsElRef {
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
}
