use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KmsKeyRingImportJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    import_job_id: PrimField<String>,
    import_method: PrimField<String>,
    key_ring: PrimField<String>,
    protection_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsKeyRingImportJobTimeoutsEl>,
}
struct KmsKeyRingImportJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsKeyRingImportJobData>,
}
#[derive(Clone)]
pub struct KmsKeyRingImportJob(Rc<KmsKeyRingImportJob_>);
impl KmsKeyRingImportJob {
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsKeyRingImportJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the key creator (for example, an HSM) at key creation time.\nUse this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly present if the chosen ImportMethod is one with a protection level of HSM."]
    pub fn attestation(&self) -> ListRef<KmsKeyRingImportJobAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which this resource is scheduled for expiration and can no longer be used.\nThis is in RFC3339 text format."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_job_id` after provisioning.\nIt must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub fn import_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_method` after provisioning.\nThe wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub fn import_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this ImportJob in the format projects/*/locations/*/keyRings/*/importJobs/*."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\nThe protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nThe public key with which to wrap key material prior to import. Only returned if state is 'ACTIVE'."]
    pub fn public_key(&self) -> ListRef<KmsKeyRingImportJobPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the ImportJob, indicating if it can be used."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyRingImportJobTimeoutsElRef {
        KmsKeyRingImportJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for KmsKeyRingImportJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KmsKeyRingImportJob {}
impl ToListMappable for KmsKeyRingImportJob {
    type O = ListRef<KmsKeyRingImportJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KmsKeyRingImportJob_ {
    fn extract_resource_type(&self) -> String {
        "google_kms_key_ring_import_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKmsKeyRingImportJob {
    pub tf_id: String,
    #[doc = "It must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub import_job_id: PrimField<String>,
    #[doc = "The wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub import_method: PrimField<String>,
    #[doc = "The KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub key_ring: PrimField<String>,
    #[doc = "The protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub protection_level: PrimField<String>,
}
impl BuildKmsKeyRingImportJob {
    pub fn build(self, stack: &mut Stack) -> KmsKeyRingImportJob {
        let out = KmsKeyRingImportJob(Rc::new(KmsKeyRingImportJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsKeyRingImportJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                import_job_id: self.import_job_id,
                import_method: self.import_method,
                key_ring: self.key_ring,
                protection_level: self.protection_level,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KmsKeyRingImportJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsKeyRingImportJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KmsKeyRingImportJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the key creator (for example, an HSM) at key creation time.\nUse this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly present if the chosen ImportMethod is one with a protection level of HSM."]
    pub fn attestation(&self) -> ListRef<KmsKeyRingImportJobAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which this resource is scheduled for expiration and can no longer be used.\nThis is in RFC3339 text format."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_job_id` after provisioning.\nIt must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub fn import_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_method` after provisioning.\nThe wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub fn import_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_ring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this ImportJob in the format projects/*/locations/*/keyRings/*/importJobs/*."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection_level` after provisioning.\nThe protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protection_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\nThe public key with which to wrap key material prior to import. Only returned if state is 'ACTIVE'."]
    pub fn public_key(&self) -> ListRef<KmsKeyRingImportJobPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the ImportJob, indicating if it can be used."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyRingImportJobTimeoutsElRef {
        KmsKeyRingImportJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct KmsKeyRingImportJobAttestationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
}
impl KmsKeyRingImportJobAttestationEl {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
}
impl ToListMappable for KmsKeyRingImportJobAttestationEl {
    type O = BlockAssignable<KmsKeyRingImportJobAttestationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsKeyRingImportJobAttestationEl {}
impl BuildKmsKeyRingImportJobAttestationEl {
    pub fn build(self) -> KmsKeyRingImportJobAttestationEl {
        KmsKeyRingImportJobAttestationEl {
            content: core::default::Default::default(),
            format: core::default::Default::default(),
        }
    }
}
pub struct KmsKeyRingImportJobAttestationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsKeyRingImportJobAttestationElRef {
    fn new(shared: StackShared, base: String) -> KmsKeyRingImportJobAttestationElRef {
        KmsKeyRingImportJobAttestationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsKeyRingImportJobAttestationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}
#[derive(Serialize)]
pub struct KmsKeyRingImportJobPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem: Option<PrimField<String>>,
}
impl KmsKeyRingImportJobPublicKeyEl {
    #[doc = "Set the field `pem`.\n"]
    pub fn set_pem(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem = Some(v.into());
        self
    }
}
impl ToListMappable for KmsKeyRingImportJobPublicKeyEl {
    type O = BlockAssignable<KmsKeyRingImportJobPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsKeyRingImportJobPublicKeyEl {}
impl BuildKmsKeyRingImportJobPublicKeyEl {
    pub fn build(self) -> KmsKeyRingImportJobPublicKeyEl {
        KmsKeyRingImportJobPublicKeyEl {
            pem: core::default::Default::default(),
        }
    }
}
pub struct KmsKeyRingImportJobPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsKeyRingImportJobPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> KmsKeyRingImportJobPublicKeyElRef {
        KmsKeyRingImportJobPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsKeyRingImportJobPublicKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem` after provisioning.\n"]
    pub fn pem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pem", self.base))
    }
}
#[derive(Serialize)]
pub struct KmsKeyRingImportJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl KmsKeyRingImportJobTimeoutsEl {
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
impl ToListMappable for KmsKeyRingImportJobTimeoutsEl {
    type O = BlockAssignable<KmsKeyRingImportJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKmsKeyRingImportJobTimeoutsEl {}
impl BuildKmsKeyRingImportJobTimeoutsEl {
    pub fn build(self) -> KmsKeyRingImportJobTimeoutsEl {
        KmsKeyRingImportJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct KmsKeyRingImportJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KmsKeyRingImportJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsKeyRingImportJobTimeoutsElRef {
        KmsKeyRingImportJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KmsKeyRingImportJobTimeoutsElRef {
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
