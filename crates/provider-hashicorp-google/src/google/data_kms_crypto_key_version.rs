use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsCryptoKeyVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    crypto_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
}
struct DataKmsCryptoKeyVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCryptoKeyVersionData>,
}
#[derive(Clone)]
pub struct DataKmsCryptoKeyVersion(Rc<DataKmsCryptoKeyVersion_>);
impl DataKmsCryptoKeyVersion {
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
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.algorithm", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\n"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyVersionPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsCryptoKeyVersion {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsCryptoKeyVersion {}
impl ToListMappable for DataKmsCryptoKeyVersion {
    type O = ListRef<DataKmsCryptoKeyVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsCryptoKeyVersion_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_crypto_key_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsCryptoKeyVersion {
    pub tf_id: String,
    #[doc = ""]
    pub crypto_key: PrimField<String>,
}
impl BuildDataKmsCryptoKeyVersion {
    pub fn build(self, stack: &mut Stack) -> DataKmsCryptoKeyVersion {
        let out = DataKmsCryptoKeyVersion(Rc::new(DataKmsCryptoKeyVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCryptoKeyVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                crypto_key: self.crypto_key,
                id: core::default::Default::default(),
                version: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsCryptoKeyVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsCryptoKeyVersionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.algorithm", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\n"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyVersionPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeyVersionPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyVersionPublicKeyEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `pem`.\n"]
    pub fn set_pem(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsCryptoKeyVersionPublicKeyEl {
    type O = BlockAssignable<DataKmsCryptoKeyVersionPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyVersionPublicKeyEl {}
impl BuildDataKmsCryptoKeyVersionPublicKeyEl {
    pub fn build(self) -> DataKmsCryptoKeyVersionPublicKeyEl {
        DataKmsCryptoKeyVersionPublicKeyEl {
            algorithm: core::default::Default::default(),
            pem: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyVersionPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyVersionPublicKeyElRef {
        DataKmsCryptoKeyVersionPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyVersionPublicKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `pem` after provisioning.\n"]
    pub fn pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pem", self.base))
    }
}
