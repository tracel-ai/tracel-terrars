use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KmsCryptoKeyVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    crypto_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_protection_level_options:
        Option<Vec<KmsCryptoKeyVersionExternalProtectionLevelOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsCryptoKeyVersionTimeoutsEl>,
    dynamic: KmsCryptoKeyVersionDynamic,
}
struct KmsCryptoKeyVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsCryptoKeyVersionData>,
}
#[derive(Clone)]
pub struct KmsCryptoKeyVersion(Rc<KmsCryptoKeyVersion_>);
impl KmsCryptoKeyVersion {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nThe current state of the CryptoKeyVersion. Note: you can only specify this field to manually 'ENABLE' or 'DISABLE' the CryptoKeyVersion,\notherwise the value of this field is always retrieved automatically. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }
    #[doc = "Set the field `external_protection_level_options`.\n"]
    pub fn set_external_protection_level_options(
        self,
        v: impl Into<BlockAssignable<KmsCryptoKeyVersionExternalProtectionLevelOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_protection_level_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .external_protection_level_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsCryptoKeyVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nThe CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly provided for key versions with protectionLevel HSM."]
    pub fn attestation(&self) -> ListRef<KmsCryptoKeyVersionAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\nThe name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generate_time` after provisioning.\nThe time this CryptoKeyVersion key material was generated"]
    pub fn generate_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generate_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CryptoKeyVersion."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\nThe ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the CryptoKeyVersion. Note: you can only specify this field to manually 'ENABLE' or 'DISABLE' the CryptoKeyVersion,\notherwise the value of this field is always retrieved automatically. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_protection_level_options` after provisioning.\n"]
    pub fn external_protection_level_options(
        &self,
    ) -> ListRef<KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_protection_level_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for KmsCryptoKeyVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KmsCryptoKeyVersion {}
impl ToListMappable for KmsCryptoKeyVersion {
    type O = ListRef<KmsCryptoKeyVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KmsCryptoKeyVersion_ {
    fn extract_resource_type(&self) -> String {
        "google_kms_crypto_key_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKmsCryptoKeyVersion {
    pub tf_id: String,
    #[doc = "The name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub crypto_key: PrimField<String>,
}
impl BuildKmsCryptoKeyVersion {
    pub fn build(self, stack: &mut Stack) -> KmsCryptoKeyVersion {
        let out = KmsCryptoKeyVersion(Rc::new(KmsCryptoKeyVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsCryptoKeyVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                crypto_key: self.crypto_key,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                state: core::default::Default::default(),
                external_protection_level_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KmsCryptoKeyVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KmsCryptoKeyVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nThe CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly provided for key versions with protectionLevel HSM."]
    pub fn attestation(&self) -> ListRef<KmsCryptoKeyVersionAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key` after provisioning.\nThe name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub fn crypto_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generate_time` after provisioning.\nThe time this CryptoKeyVersion key material was generated"]
    pub fn generate_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generate_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CryptoKeyVersion."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\nThe ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the CryptoKeyVersion. Note: you can only specify this field to manually 'ENABLE' or 'DISABLE' the CryptoKeyVersion,\notherwise the value of this field is always retrieved automatically. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_protection_level_options` after provisioning.\n"]
    pub fn external_protection_level_options(
        &self,
    ) -> ListRef<KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_protection_level_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationElCertChainsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cavium_certs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_card_certs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_partition_certs: Option<ListField<PrimField<String>>>,
}
impl KmsCryptoKeyVersionAttestationElCertChainsEl {
    #[doc = "Set the field `cavium_certs`.\n"]
    pub fn set_cavium_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cavium_certs = Some(v.into());
        self
    }
    #[doc = "Set the field `google_card_certs`.\n"]
    pub fn set_google_card_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.google_card_certs = Some(v.into());
        self
    }
    #[doc = "Set the field `google_partition_certs`.\n"]
    pub fn set_google_partition_certs(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.google_partition_certs = Some(v.into());
        self
    }
}
impl ToListMappable for KmsCryptoKeyVersionAttestationElCertChainsEl {
    type O = BlockAssignable<KmsCryptoKeyVersionAttestationElCertChainsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsCryptoKeyVersionAttestationElCertChainsEl {}
impl BuildKmsCryptoKeyVersionAttestationElCertChainsEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationElCertChainsEl {
        KmsCryptoKeyVersionAttestationElCertChainsEl {
            cavium_certs: core::default::Default::default(),
            google_card_certs: core::default::Default::default(),
            google_partition_certs: core::default::Default::default(),
        }
    }
}
pub struct KmsCryptoKeyVersionAttestationElCertChainsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionAttestationElCertChainsElRef {
    fn new(shared: StackShared, base: String) -> KmsCryptoKeyVersionAttestationElCertChainsElRef {
        KmsCryptoKeyVersionAttestationElCertChainsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsCryptoKeyVersionAttestationElCertChainsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cavium_certs` after provisioning.\n"]
    pub fn cavium_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cavium_certs", self.base))
    }
    #[doc = "Get a reference to the value of field `google_card_certs` after provisioning.\n"]
    pub fn google_card_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_card_certs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_partition_certs` after provisioning.\n"]
    pub fn google_partition_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_partition_certs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ekm_connection_key_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_key_uri: Option<PrimField<String>>,
}
impl KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    #[doc = "Set the field `ekm_connection_key_path`.\n"]
    pub fn set_ekm_connection_key_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ekm_connection_key_path = Some(v.into());
        self
    }
    #[doc = "Set the field `external_key_uri`.\n"]
    pub fn set_external_key_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_key_uri = Some(v.into());
        self
    }
}
impl ToListMappable for KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    type O = BlockAssignable<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {}
impl BuildKmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
        KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
            ekm_connection_key_path: core::default::Default::default(),
            external_key_uri: core::default::Default::default(),
        }
    }
}
pub struct KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
        KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ekm_connection_key_path` after provisioning.\n"]
    pub fn ekm_connection_key_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ekm_connection_key_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_key_uri` after provisioning.\n"]
    pub fn external_key_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_key_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_chains: Option<ListField<KmsCryptoKeyVersionAttestationElCertChainsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_protection_level_options:
        Option<ListField<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
}
impl KmsCryptoKeyVersionAttestationEl {
    #[doc = "Set the field `cert_chains`.\n"]
    pub fn set_cert_chains(
        mut self,
        v: impl Into<ListField<KmsCryptoKeyVersionAttestationElCertChainsEl>>,
    ) -> Self {
        self.cert_chains = Some(v.into());
        self
    }
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }
    #[doc = "Set the field `external_protection_level_options`.\n"]
    pub fn set_external_protection_level_options(
        mut self,
        v: impl Into<ListField<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>>,
    ) -> Self {
        self.external_protection_level_options = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
}
impl ToListMappable for KmsCryptoKeyVersionAttestationEl {
    type O = BlockAssignable<KmsCryptoKeyVersionAttestationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsCryptoKeyVersionAttestationEl {}
impl BuildKmsCryptoKeyVersionAttestationEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationEl {
        KmsCryptoKeyVersionAttestationEl {
            cert_chains: core::default::Default::default(),
            content: core::default::Default::default(),
            external_protection_level_options: core::default::Default::default(),
            format: core::default::Default::default(),
        }
    }
}
pub struct KmsCryptoKeyVersionAttestationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionAttestationElRef {
    fn new(shared: StackShared, base: String) -> KmsCryptoKeyVersionAttestationElRef {
        KmsCryptoKeyVersionAttestationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsCryptoKeyVersionAttestationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_chains` after provisioning.\n"]
    pub fn cert_chains(&self) -> ListRef<KmsCryptoKeyVersionAttestationElCertChainsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cert_chains", self.base))
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `external_protection_level_options` after provisioning.\n"]
    pub fn external_protection_level_options(
        &self,
    ) -> ListRef<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_protection_level_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}
#[derive(Serialize)]
pub struct KmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ekm_connection_key_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_key_uri: Option<PrimField<String>>,
}
impl KmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
    #[doc = "Set the field `ekm_connection_key_path`.\nThe path to the external key material on the EKM when using EkmConnection e.g., \"v0/my/key\". Set this field instead of externalKeyUri when using an EkmConnection."]
    pub fn set_ekm_connection_key_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ekm_connection_key_path = Some(v.into());
        self
    }
    #[doc = "Set the field `external_key_uri`.\nThe URI for an external resource that this CryptoKeyVersion represents."]
    pub fn set_external_key_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_key_uri = Some(v.into());
        self
    }
}
impl ToListMappable for KmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
    type O = BlockAssignable<KmsCryptoKeyVersionExternalProtectionLevelOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsCryptoKeyVersionExternalProtectionLevelOptionsEl {}
impl BuildKmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
    pub fn build(self) -> KmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
        KmsCryptoKeyVersionExternalProtectionLevelOptionsEl {
            ekm_connection_key_path: core::default::Default::default(),
            external_key_uri: core::default::Default::default(),
        }
    }
}
pub struct KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef {
        KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsCryptoKeyVersionExternalProtectionLevelOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ekm_connection_key_path` after provisioning.\nThe path to the external key material on the EKM when using EkmConnection e.g., \"v0/my/key\". Set this field instead of externalKeyUri when using an EkmConnection."]
    pub fn ekm_connection_key_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ekm_connection_key_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_key_uri` after provisioning.\nThe URI for an external resource that this CryptoKeyVersion represents."]
    pub fn external_key_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_key_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KmsCryptoKeyVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl KmsCryptoKeyVersionTimeoutsEl {
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
impl ToListMappable for KmsCryptoKeyVersionTimeoutsEl {
    type O = BlockAssignable<KmsCryptoKeyVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsCryptoKeyVersionTimeoutsEl {}
impl BuildKmsCryptoKeyVersionTimeoutsEl {
    pub fn build(self) -> KmsCryptoKeyVersionTimeoutsEl {
        KmsCryptoKeyVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct KmsCryptoKeyVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsCryptoKeyVersionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsCryptoKeyVersionTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct KmsCryptoKeyVersionDynamic {
    external_protection_level_options:
        Option<DynamicBlock<KmsCryptoKeyVersionExternalProtectionLevelOptionsEl>>,
}
