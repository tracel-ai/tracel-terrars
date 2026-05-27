use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsCryptoKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_ring: PrimField<String>,
}
struct DataKmsCryptoKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCryptoKeysData>,
}
#[derive(Clone)]
pub struct DataKmsCryptoKeys(Rc<DataKmsCryptoKeys_>);
impl DataKmsCryptoKeys {
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
    #[doc = "Set the field `filter`.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve keys that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve keys that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `keys` after provisioning.\nA list of all the retrieved keys from the provided key ring"]
    pub fn keys(&self) -> ListRef<DataKmsCryptoKeysKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.keys", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsCryptoKeys {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsCryptoKeys {}
impl ToListMappable for DataKmsCryptoKeys {
    type O = ListRef<DataKmsCryptoKeysRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsCryptoKeys_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_crypto_keys".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsCryptoKeys {
    pub tf_id: String,
    #[doc = "The key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'."]
    pub key_ring: PrimField<String>,
}
impl BuildDataKmsCryptoKeys {
    pub fn build(self, stack: &mut Stack) -> DataKmsCryptoKeys {
        let out = DataKmsCryptoKeys(Rc::new(DataKmsCryptoKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCryptoKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                key_ring: self.key_ring,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsCryptoKeysRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsCryptoKeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve keys that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1\" will only retrieve a key with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `keys` after provisioning.\nA list of all the retrieved keys from the provided key ring"]
    pub fn keys(&self) -> ListRef<DataKmsCryptoKeysKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.keys", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeysKeysElPrimaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataKmsCryptoKeysKeysElPrimaryEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsCryptoKeysKeysElPrimaryEl {
    type O = BlockAssignable<DataKmsCryptoKeysKeysElPrimaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeysKeysElPrimaryEl {}
impl BuildDataKmsCryptoKeysKeysElPrimaryEl {
    pub fn build(self) -> DataKmsCryptoKeysKeysElPrimaryEl {
        DataKmsCryptoKeysKeysElPrimaryEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeysKeysElPrimaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeysKeysElPrimaryElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeysKeysElPrimaryElRef {
        DataKmsCryptoKeysKeysElPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeysKeysElPrimaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeysKeysElVersionTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection_level: Option<PrimField<String>>,
}
impl DataKmsCryptoKeysKeysElVersionTemplateEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `protection_level`.\n"]
    pub fn set_protection_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protection_level = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsCryptoKeysKeysElVersionTemplateEl {
    type O = BlockAssignable<DataKmsCryptoKeysKeysElVersionTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeysKeysElVersionTemplateEl {}
impl BuildDataKmsCryptoKeysKeysElVersionTemplateEl {
    pub fn build(self) -> DataKmsCryptoKeysKeysElVersionTemplateEl {
        DataKmsCryptoKeysKeysElVersionTemplateEl {
            algorithm: core::default::Default::default(),
            protection_level: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeysKeysElVersionTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeysKeysElVersionTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeysKeysElVersionTemplateElRef {
        DataKmsCryptoKeysKeysElVersionTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeysKeysElVersionTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\n"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeysKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_key_backend: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_scheduled_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_ring: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<ListField<DataKmsCryptoKeysKeysElPrimaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_initial_version_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_template: Option<ListField<DataKmsCryptoKeysKeysElVersionTemplateEl>>,
}
impl DataKmsCryptoKeysKeysEl {
    #[doc = "Set the field `crypto_key_backend`.\n"]
    pub fn set_crypto_key_backend(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crypto_key_backend = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `destroy_scheduled_duration`.\n"]
    pub fn set_destroy_scheduled_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destroy_scheduled_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `import_only`.\n"]
    pub fn set_import_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.import_only = Some(v.into());
        self
    }
    #[doc = "Set the field `key_ring`.\n"]
    pub fn set_key_ring(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_ring = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v: impl Into<ListField<DataKmsCryptoKeysKeysElPrimaryEl>>,
    ) -> Self {
        self.primary = Some(v.into());
        self
    }
    #[doc = "Set the field `purpose`.\n"]
    pub fn set_purpose(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.purpose = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_initial_version_creation`.\n"]
    pub fn set_skip_initial_version_creation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.skip_initial_version_creation = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `version_template`.\n"]
    pub fn set_version_template(
        mut self,
        v: impl Into<ListField<DataKmsCryptoKeysKeysElVersionTemplateEl>>,
    ) -> Self {
        self.version_template = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsCryptoKeysKeysEl {
    type O = BlockAssignable<DataKmsCryptoKeysKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeysKeysEl {}
impl BuildDataKmsCryptoKeysKeysEl {
    pub fn build(self) -> DataKmsCryptoKeysKeysEl {
        DataKmsCryptoKeysKeysEl {
            crypto_key_backend: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            destroy_scheduled_duration: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            id: core::default::Default::default(),
            import_only: core::default::Default::default(),
            key_ring: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            primary: core::default::Default::default(),
            purpose: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
            skip_initial_version_creation: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            version_template: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeysKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeysKeysElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeysKeysElRef {
        DataKmsCryptoKeysKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeysKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crypto_key_backend` after provisioning.\n"]
    pub fn crypto_key_backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_backend", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_scheduled_duration` after provisioning.\n"]
    pub fn destroy_scheduled_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_scheduled_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `import_only` after provisioning.\n"]
    pub fn import_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_only", self.base))
    }
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\n"]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_ring", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> ListRef<DataKmsCryptoKeysKeysElPrimaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\n"]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `skip_initial_version_creation` after provisioning.\n"]
    pub fn skip_initial_version_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_initial_version_creation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version_template` after provisioning.\n"]
    pub fn version_template(&self) -> ListRef<DataKmsCryptoKeysKeysElVersionTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_template", self.base),
        )
    }
}
