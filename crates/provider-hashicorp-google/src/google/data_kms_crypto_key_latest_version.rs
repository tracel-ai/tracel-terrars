use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsCryptoKeyLatestVersionData {
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
struct DataKmsCryptoKeyLatestVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCryptoKeyLatestVersionData>,
}
#[derive(Clone)]
pub struct DataKmsCryptoKeyLatestVersion(Rc<DataKmsCryptoKeyLatestVersion_>);
impl DataKmsCryptoKeyLatestVersion {
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
    #[doc = "Set the field `filter`.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which type of cryptoKeyVersion is retrieved as the latest by the data source: ?filter={{filter}}. When no value is provided there is no filtering.\n\n\t\t\t\t\tExample filter values if filtering on state.\n\n\t\t\t\t\t* \"state:ENABLED\" will retrieve the latest cryptoKeyVersion that has the state \"ENABLED\".\n\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which type of cryptoKeyVersion is retrieved as the latest by the data source: ?filter={{filter}}. When no value is provided there is no filtering.\n\n\t\t\t\t\tExample filter values if filtering on state.\n\n\t\t\t\t\t* \"state:ENABLED\" will retrieve the latest cryptoKeyVersion that has the state \"ENABLED\".\n\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyLatestVersionPublicKeyElRef> {
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
impl Referable for DataKmsCryptoKeyLatestVersion {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsCryptoKeyLatestVersion {}
impl ToListMappable for DataKmsCryptoKeyLatestVersion {
    type O = ListRef<DataKmsCryptoKeyLatestVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsCryptoKeyLatestVersion_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_crypto_key_latest_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsCryptoKeyLatestVersion {
    pub tf_id: String,
    #[doc = ""]
    pub crypto_key: PrimField<String>,
}
impl BuildDataKmsCryptoKeyLatestVersion {
    pub fn build(self, stack: &mut Stack) -> DataKmsCryptoKeyLatestVersion {
        let out = DataKmsCryptoKeyLatestVersion(Rc::new(DataKmsCryptoKeyLatestVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCryptoKeyLatestVersionData {
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
pub struct DataKmsCryptoKeyLatestVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyLatestVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsCryptoKeyLatestVersionRef {
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which type of cryptoKeyVersion is retrieved as the latest by the data source: ?filter={{filter}}. When no value is provided there is no filtering.\n\n\t\t\t\t\tExample filter values if filtering on state.\n\n\t\t\t\t\t* \"state:ENABLED\" will retrieve the latest cryptoKeyVersion that has the state \"ENABLED\".\n\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    pub fn public_key(&self) -> ListRef<DataKmsCryptoKeyLatestVersionPublicKeyElRef> {
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
pub struct DataKmsCryptoKeyLatestVersionPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyLatestVersionPublicKeyEl {
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
impl ToListMappable for DataKmsCryptoKeyLatestVersionPublicKeyEl {
    type O = BlockAssignable<DataKmsCryptoKeyLatestVersionPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyLatestVersionPublicKeyEl {}
impl BuildDataKmsCryptoKeyLatestVersionPublicKeyEl {
    pub fn build(self) -> DataKmsCryptoKeyLatestVersionPublicKeyEl {
        DataKmsCryptoKeyLatestVersionPublicKeyEl {
            algorithm: core::default::Default::default(),
            pem: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyLatestVersionPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyLatestVersionPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyLatestVersionPublicKeyElRef {
        DataKmsCryptoKeyLatestVersionPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyLatestVersionPublicKeyElRef {
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
