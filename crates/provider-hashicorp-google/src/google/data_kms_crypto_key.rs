use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsCryptoKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_ring: PrimField<String>,
    name: PrimField<String>,
}
struct DataKmsCryptoKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCryptoKeyData>,
}
#[derive(Clone)]
pub struct DataKmsCryptoKey(Rc<DataKmsCryptoKey_>);
impl DataKmsCryptoKey {
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
    #[doc = "Get a reference to the value of field `crypto_key_backend` after provisioning.\nThe resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.\nThe resource name is in the format \"projects/*/locations/*/ekmConnections/*\" and only applies to \"EXTERNAL_VPC\" keys."]
    pub fn crypto_key_backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_scheduled_duration` after provisioning.\nThe period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.\nIf not specified at creation time, the default duration is 30 days."]
    pub fn destroy_scheduled_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_scheduled_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_only` after provisioning.\nWhether this key may contain imported versions only."]
    pub fn import_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the CryptoKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary` after provisioning.\nA copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.\nKeys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset."]
    pub fn primary(&self) -> ListRef<DataKmsCryptoKeyPrimaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\nThe immutable purpose of this CryptoKey. See the\n[purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)\nfor possible inputs.\nDefault value is \"ENCRYPT_DECRYPT\"."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\nEvery time this period passes, generate a new CryptoKeyVersion and set it as the primary.\nThe first rotation will take place after the specified period. The rotation period has\nthe format of a decimal number with up to 9 fractional digits, followed by the\nletter 's' (seconds). It must be greater than a day (ie, 86400)."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_initial_version_creation` after provisioning.\nIf set to true, the request will create a CryptoKey without any CryptoKeyVersions.\nYou must use the 'google_kms_crypto_key_version' resource to create a new CryptoKeyVersion\nor 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion.\nThis field is only applicable during initial CryptoKey creation."]
    pub fn skip_initial_version_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_initial_version_creation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_template` after provisioning.\nA template describing settings for new crypto key versions."]
    pub fn version_template(&self) -> ListRef<DataKmsCryptoKeyVersionTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_template", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsCryptoKey {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsCryptoKey {}
impl ToListMappable for DataKmsCryptoKey {
    type O = ListRef<DataKmsCryptoKeyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsCryptoKey_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_crypto_key".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsCryptoKey {
    pub tf_id: String,
    #[doc = "The KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub key_ring: PrimField<String>,
    #[doc = "The resource name for the CryptoKey."]
    pub name: PrimField<String>,
}
impl BuildDataKmsCryptoKey {
    pub fn build(self, stack: &mut Stack) -> DataKmsCryptoKey {
        let out = DataKmsCryptoKey(Rc::new(DataKmsCryptoKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCryptoKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                key_ring: self.key_ring,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsCryptoKeyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsCryptoKeyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `crypto_key_backend` after provisioning.\nThe resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.\nThe resource name is in the format \"projects/*/locations/*/ekmConnections/*\" and only applies to \"EXTERNAL_VPC\" keys."]
    pub fn crypto_key_backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_scheduled_duration` after provisioning.\nThe period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.\nIf not specified at creation time, the default duration is 30 days."]
    pub fn destroy_scheduled_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_scheduled_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_only` after provisioning.\nWhether this key may contain imported versions only."]
    pub fn import_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the CryptoKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary` after provisioning.\nA copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.\nKeys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset."]
    pub fn primary(&self) -> ListRef<DataKmsCryptoKeyPrimaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\nThe immutable purpose of this CryptoKey. See the\n[purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)\nfor possible inputs.\nDefault value is \"ENCRYPT_DECRYPT\"."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\nEvery time this period passes, generate a new CryptoKeyVersion and set it as the primary.\nThe first rotation will take place after the specified period. The rotation period has\nthe format of a decimal number with up to 9 fractional digits, followed by the\nletter 's' (seconds). It must be greater than a day (ie, 86400)."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_initial_version_creation` after provisioning.\nIf set to true, the request will create a CryptoKey without any CryptoKeyVersions.\nYou must use the 'google_kms_crypto_key_version' resource to create a new CryptoKeyVersion\nor 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion.\nThis field is only applicable during initial CryptoKey creation."]
    pub fn skip_initial_version_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_initial_version_creation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_template` after provisioning.\nA template describing settings for new crypto key versions."]
    pub fn version_template(&self) -> ListRef<DataKmsCryptoKeyVersionTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_template", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsCryptoKeyPrimaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyPrimaryEl {
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
impl ToListMappable for DataKmsCryptoKeyPrimaryEl {
    type O = BlockAssignable<DataKmsCryptoKeyPrimaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyPrimaryEl {}
impl BuildDataKmsCryptoKeyPrimaryEl {
    pub fn build(self) -> DataKmsCryptoKeyPrimaryEl {
        DataKmsCryptoKeyPrimaryEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyPrimaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyPrimaryElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyPrimaryElRef {
        DataKmsCryptoKeyPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyPrimaryElRef {
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
pub struct DataKmsCryptoKeyVersionTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection_level: Option<PrimField<String>>,
}
impl DataKmsCryptoKeyVersionTemplateEl {
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
impl ToListMappable for DataKmsCryptoKeyVersionTemplateEl {
    type O = BlockAssignable<DataKmsCryptoKeyVersionTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsCryptoKeyVersionTemplateEl {}
impl BuildDataKmsCryptoKeyVersionTemplateEl {
    pub fn build(self) -> DataKmsCryptoKeyVersionTemplateEl {
        DataKmsCryptoKeyVersionTemplateEl {
            algorithm: core::default::Default::default(),
            protection_level: core::default::Default::default(),
        }
    }
}
pub struct DataKmsCryptoKeyVersionTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsCryptoKeyVersionTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataKmsCryptoKeyVersionTemplateElRef {
        DataKmsCryptoKeyVersionTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsCryptoKeyVersionTemplateElRef {
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
