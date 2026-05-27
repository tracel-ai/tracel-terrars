use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BinaryAuthorizationAttestorData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation_authority_note: Option<Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BinaryAuthorizationAttestorTimeoutsEl>,
    dynamic: BinaryAuthorizationAttestorDynamic,
}
struct BinaryAuthorizationAttestor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BinaryAuthorizationAttestorData>,
}
#[derive(Clone)]
pub struct BinaryAuthorizationAttestor(Rc<BinaryAuthorizationAttestor_>);
impl BinaryAuthorizationAttestor {
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
    #[doc = "Set the field `description`.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `attestation_authority_note`.\n"]
    pub fn set_attestation_authority_note(
        self,
        v: impl Into<BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_authority_note = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_authority_note = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BinaryAuthorizationAttestorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation_authority_note` after provisioning.\n"]
    pub fn attestation_authority_note(
        &self,
    ) -> ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation_authority_note", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BinaryAuthorizationAttestor {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BinaryAuthorizationAttestor {}
impl ToListMappable for BinaryAuthorizationAttestor {
    type O = ListRef<BinaryAuthorizationAttestorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BinaryAuthorizationAttestor_ {
    fn extract_resource_type(&self) -> String {
        "google_binary_authorization_attestor".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBinaryAuthorizationAttestor {
    pub tf_id: String,
    #[doc = "The resource name."]
    pub name: PrimField<String>,
}
impl BuildBinaryAuthorizationAttestor {
    pub fn build(self, stack: &mut Stack) -> BinaryAuthorizationAttestor {
        let out = BinaryAuthorizationAttestor(Rc::new(BinaryAuthorizationAttestor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BinaryAuthorizationAttestorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                attestation_authority_note: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BinaryAuthorizationAttestorRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationAttestorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BinaryAuthorizationAttestorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation_authority_note` after provisioning.\n"]
    pub fn attestation_authority_note(
        &self,
    ) -> ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation_authority_note", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key_pem: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_algorithm: Option<PrimField<String>>,
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    #[doc = "Set the field `public_key_pem`.\nA PEM-encoded public key, as described in\n'https://tools.ietf.org/html/rfc7468#section-13'"]
    pub fn set_public_key_pem(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key_pem = Some(v.into());
        self
    }
    #[doc = "Set the field `signature_algorithm`.\nThe signature algorithm used to verify a message against\na signature using this key. These signature algorithm must\nmatch the structure and any object identifiers encoded in\npublicKeyPem (i.e. this algorithm must match that of the\npublic key)."]
    pub fn set_signature_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signature_algorithm = Some(v.into());
        self
    }
}
impl ToListMappable
    for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl
{
    type O = BlockAssignable<
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
}
impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    pub fn build(
        self,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
            public_key_pem: core::default::Default::default(),
            signature_algorithm: core::default::Default::default(),
        }
    }
}
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_key_pem` after provisioning.\nA PEM-encoded public key, as described in\n'https://tools.ietf.org/html/rfc7468#section-13'"]
    pub fn public_key_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_key_pem", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signature_algorithm` after provisioning.\nThe signature algorithm used to verify a message against\na signature using this key. These signature algorithm must\nmatch the structure and any object identifiers encoded in\npublicKeyPem (i.e. this algorithm must match that of the\npublic key)."]
    pub fn signature_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signature_algorithm", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElDynamic {
    pkix_public_key: Option<
        DynamicBlock<
            BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ascii_armored_pgp_public_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pkix_public_key: Option<
        Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl>,
    >,
    dynamic: BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElDynamic,
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    #[doc = "Set the field `ascii_armored_pgp_public_key`.\nASCII-armored representation of a PGP public key, as the\nentire output by the command\n'gpg --export --armor foo@example.com' (either LF or CRLF\nline endings). When using this field, id should be left\nblank. The BinAuthz API handlers will calculate the ID\nand fill it in automatically. BinAuthz computes this ID\nas the OpenPGP RFC4880 V4 fingerprint, represented as\nupper-case hex. If id is provided by the caller, it will\nbe overwritten by the API-calculated ID."]
    pub fn set_ascii_armored_pgp_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ascii_armored_pgp_public_key = Some(v.into());
        self
    }
    #[doc = "Set the field `comment`.\nA descriptive comment. This field may be updated."]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\nThe ID of this public key. Signatures verified by BinAuthz\nmust include the ID of the public key that can be used to\nverify them, and that ID must match the contents of this\nfield exactly. Additional restrictions on this field can\nbe imposed based on which public key type is encapsulated.\nSee the documentation on publicKey cases below for details."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `pkix_public_key`.\n"]
    pub fn set_pkix_public_key(
        mut self,
        v: impl Into<
            BlockAssignable<
                BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pkix_public_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pkix_public_key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    type O = BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {}
impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    pub fn build(self) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
            ascii_armored_pgp_public_key: core::default::Default::default(),
            comment: core::default::Default::default(),
            id: core::default::Default::default(),
            pkix_public_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ascii_armored_pgp_public_key` after provisioning.\nASCII-armored representation of a PGP public key, as the\nentire output by the command\n'gpg --export --armor foo@example.com' (either LF or CRLF\nline endings). When using this field, id should be left\nblank. The BinAuthz API handlers will calculate the ID\nand fill it in automatically. BinAuthz computes this ID\nas the OpenPGP RFC4880 V4 fingerprint, represented as\nupper-case hex. If id is provided by the caller, it will\nbe overwritten by the API-calculated ID."]
    pub fn ascii_armored_pgp_public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ascii_armored_pgp_public_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\nA descriptive comment. This field may be updated."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this public key. Signatures verified by BinAuthz\nmust include the ID of the public key that can be used to\nverify them, and that ID must match the contents of this\nfield exactly. Additional restrictions on this field can\nbe imposed based on which public key type is encapsulated.\nSee the documentation on publicKey cases below for details."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `pkix_public_key` after provisioning.\n"]
    pub fn pkix_public_key(
        &self,
    ) -> ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pkix_public_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BinaryAuthorizationAttestorAttestationAuthorityNoteElDynamic {
    public_keys:
        Option<DynamicBlock<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>>,
}
#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    note_reference: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_keys: Option<Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>>,
    dynamic: BinaryAuthorizationAttestorAttestationAuthorityNoteElDynamic,
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    #[doc = "Set the field `public_keys`.\n"]
    pub fn set_public_keys(
        mut self,
        v: impl Into<BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_keys = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_keys = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    type O = BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    #[doc = "The resource name of a ATTESTATION_AUTHORITY Note, created by the\nuser. If the Note is in a different project from the Attestor, it\nshould be specified in the format 'projects/*/notes/*' (or the legacy\n'providers/*/notes/*'). This field may not be updated.\nAn attestation by this attestor is stored as a Container Analysis\nATTESTATION_AUTHORITY Occurrence that names a container image\nand that links to this Note."]
    pub note_reference: PrimField<String>,
}
impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    pub fn build(self) -> BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
            note_reference: self.note_reference,
            public_keys: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delegation_service_account_email` after provisioning.\nThis field will contain the service account email address that\nthis Attestor will use as the principal when querying Container\nAnalysis. Attestor administrators must grant this service account\nthe IAM role needed to read attestations from the noteReference in\nContainer Analysis (containeranalysis.notes.occurrences.viewer).\nThis email address is fixed for the lifetime of the Attestor, but\ncallers should not make any other assumptions about the service\naccount email; future versions may use an email based on a\ndifferent naming pattern."]
    pub fn delegation_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegation_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `note_reference` after provisioning.\nThe resource name of a ATTESTATION_AUTHORITY Note, created by the\nuser. If the Note is in a different project from the Attestor, it\nshould be specified in the format 'projects/*/notes/*' (or the legacy\n'providers/*/notes/*'). This field may not be updated.\nAn attestation by this attestor is stored as a Container Analysis\nATTESTATION_AUTHORITY Occurrence that names a container image\nand that links to this Note."]
    pub fn note_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.note_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `public_keys` after provisioning.\n"]
    pub fn public_keys(
        &self,
    ) -> ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.public_keys", self.base))
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BinaryAuthorizationAttestorTimeoutsEl {
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
impl ToListMappable for BinaryAuthorizationAttestorTimeoutsEl {
    type O = BlockAssignable<BinaryAuthorizationAttestorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationAttestorTimeoutsEl {}
impl BuildBinaryAuthorizationAttestorTimeoutsEl {
    pub fn build(self) -> BinaryAuthorizationAttestorTimeoutsEl {
        BinaryAuthorizationAttestorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BinaryAuthorizationAttestorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationAttestorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationAttestorTimeoutsElRef {
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
struct BinaryAuthorizationAttestorDynamic {
    attestation_authority_note:
        Option<DynamicBlock<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
}
