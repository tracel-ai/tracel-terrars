use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsCryptoKeyVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    crypto_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataKmsCryptoKeyVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCryptoKeyVersionsData>,
}
#[derive(Clone)]
pub struct DataKmsCryptoKeyVersions(Rc<DataKmsCryptoKeyVersions_>);
impl DataKmsCryptoKeyVersions {
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
    #[doc = "Set the field `filter`.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which cryptoKeyVersions are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-cryptokey-version-\" will retrieve cryptoKeyVersions that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}/cryptoKeyVersions/{{cryptoKeyVersion}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1/cryptoKeyVersions/1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\n"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which cryptoKeyVersions are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-cryptokey-version-\" will retrieve cryptoKeyVersions that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}/cryptoKeyVersions/{{cryptoKeyVersion}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1/cryptoKeyVersions/1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyVersionsPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\nA list of all the retrieved cryptoKeyVersions from the provided crypto key"]
    pub fn versions(&self) -> ListRef<DataKmsCryptoKeyVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsCryptoKeyVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsCryptoKeyVersions {}
impl ToListMappable for DataKmsCryptoKeyVersions {
    type O = ListRef<DataKmsCryptoKeyVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsCryptoKeyVersions_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_crypto_key_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsCryptoKeyVersions {
    pub tf_id: String,
    #[doc = ""]
    pub crypto_key: PrimField<String>,
}
impl BuildDataKmsCryptoKeyVersions {
    pub fn build(self, stack: &mut Stack) -> DataKmsCryptoKeyVersions {
        let out = DataKmsCryptoKeyVersions(Rc::new(DataKmsCryptoKeyVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCryptoKeyVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                crypto_key: self.crypto_key,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsCryptoKeyVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsCryptoKeyVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\n"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which cryptoKeyVersions are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-cryptokey-version-\" will retrieve cryptoKeyVersions that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}/cryptoKeyVersions/{{cryptoKeyVersion}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1/cryptoKeyVersions/1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyVersionsPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\nA list of all the retrieved cryptoKeyVersions from the provided crypto key"]
    pub fn versions(&self) -> ListRef<DataKmsCryptoKeyVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeyVersionsPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyVersionsPublicKeyEl {
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
impl ToListMappable for DataKmsCryptoKeyVersionsPublicKeyEl {
    type O = BlockAssignable<DataKmsCryptoKeyVersionsPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyVersionsPublicKeyEl {}
impl BuildDataKmsCryptoKeyVersionsPublicKeyEl {
    pub fn build(self) -> DataKmsCryptoKeyVersionsPublicKeyEl {
        DataKmsCryptoKeyVersionsPublicKeyEl {
            algorithm: core::default::Default::default(),
            pem: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyVersionsPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionsPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyVersionsPublicKeyElRef {
        DataKmsCryptoKeyVersionsPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyVersionsPublicKeyElRef {
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
#[derive(Serialize)]
pub struct DataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
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
impl ToListMappable for DataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
    type O = BlockAssignable<DataKmsCryptoKeyVersionsVersionsElPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyVersionsVersionsElPublicKeyEl {}
impl BuildDataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
    pub fn build(self) -> DataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
        DataKmsCryptoKeyVersionsVersionsElPublicKeyEl {
            algorithm: core::default::Default::default(),
            pem: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef {
        DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef {
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
#[derive(Serialize)]
pub struct DataKmsCryptoKeyVersionsVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<ListField<DataKmsCryptoKeyVersionsVersionsElPublicKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
}
impl DataKmsCryptoKeyVersionsVersionsEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `crypto_key`.\n"]
    pub fn set_crypto_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crypto_key = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `protection_level`.\n"]
    pub fn set_protection_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protection_level = Some(v.into());
        self
    }
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(
        mut self,
        v: impl Into<ListField<DataKmsCryptoKeyVersionsVersionsElPublicKeyEl>>,
    ) -> Self {
        self.public_key = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsCryptoKeyVersionsVersionsEl {
    type O = BlockAssignable<DataKmsCryptoKeyVersionsVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyVersionsVersionsEl {}
impl BuildDataKmsCryptoKeyVersionsVersionsEl {
    pub fn build(self) -> DataKmsCryptoKeyVersionsVersionsEl {
        DataKmsCryptoKeyVersionsVersionsEl {
            algorithm: core::default::Default::default(),
            crypto_key: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            protection_level: core::default::Default::default(),
            public_key: core::default::Default::default(),
            state: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyVersionsVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionsVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyVersionsVersionsElRef {
        DataKmsCryptoKeyVersionsVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyVersionsVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\n"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crypto_key", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\n"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyVersionsVersionsElPublicKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
