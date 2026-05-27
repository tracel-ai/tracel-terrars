use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAnalysisOccurrenceData {
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
    note_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation: Option<PrimField<String>>,
    resource_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation: Option<Vec<ContainerAnalysisOccurrenceAttestationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAnalysisOccurrenceTimeoutsEl>,
    dynamic: ContainerAnalysisOccurrenceDynamic,
}
struct ContainerAnalysisOccurrence_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAnalysisOccurrenceData>,
}
#[derive(Clone)]
pub struct ContainerAnalysisOccurrence(Rc<ContainerAnalysisOccurrence_>);
impl ContainerAnalysisOccurrence {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `remediation`.\nA description of actions that can be taken to remedy the note."]
    pub fn set_remediation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remediation = Some(v.into());
        self
    }
    #[doc = "Set the field `attestation`.\n"]
    pub fn set_attestation(
        self,
        v: impl Into<BlockAssignable<ContainerAnalysisOccurrenceAttestationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerAnalysisOccurrenceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nThe note kind which explicitly denotes which of the occurrence\ndetails are specified. This field can be used as a filter in list\nrequests."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the occurrence."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `note_name` after provisioning.\nThe analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub fn note_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.note_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remediation` after provisioning.\nA description of actions that can be taken to remedy the note."]
    pub fn remediation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remediation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_uri` after provisioning.\nRequired. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub fn resource_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\n"]
    pub fn attestation(&self) -> ListRef<ContainerAnalysisOccurrenceAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisOccurrenceTimeoutsElRef {
        ContainerAnalysisOccurrenceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAnalysisOccurrence {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAnalysisOccurrence {}
impl ToListMappable for ContainerAnalysisOccurrence {
    type O = ListRef<ContainerAnalysisOccurrenceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAnalysisOccurrence_ {
    fn extract_resource_type(&self) -> String {
        "google_container_analysis_occurrence".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAnalysisOccurrence {
    pub tf_id: String,
    #[doc = "The analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub note_name: PrimField<String>,
    #[doc = "Required. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub resource_uri: PrimField<String>,
}
impl BuildContainerAnalysisOccurrence {
    pub fn build(self, stack: &mut Stack) -> ContainerAnalysisOccurrence {
        let out = ContainerAnalysisOccurrence(Rc::new(ContainerAnalysisOccurrence_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAnalysisOccurrenceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                note_name: self.note_name,
                project: core::default::Default::default(),
                remediation: core::default::Default::default(),
                resource_uri: self.resource_uri,
                attestation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAnalysisOccurrenceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisOccurrenceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAnalysisOccurrenceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nThe note kind which explicitly denotes which of the occurrence\ndetails are specified. This field can be used as a filter in list\nrequests."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the occurrence."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `note_name` after provisioning.\nThe analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub fn note_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.note_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remediation` after provisioning.\nA description of actions that can be taken to remedy the note."]
    pub fn remediation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remediation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_uri` after provisioning.\nRequired. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub fn resource_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation` after provisioning.\n"]
    pub fn attestation(&self) -> ListRef<ContainerAnalysisOccurrenceAttestationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisOccurrenceTimeoutsElRef {
        ContainerAnalysisOccurrenceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAnalysisOccurrenceAttestationElSignaturesEl {
    public_key_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<PrimField<String>>,
}
impl ContainerAnalysisOccurrenceAttestationElSignaturesEl {
    #[doc = "Set the field `signature`.\nThe content of the signature, an opaque bytestring.\nThe payload that this signature verifies MUST be\nunambiguously provided with the Signature during\nverification. A wrapper message might provide the\npayload explicitly. Alternatively, a message might\nhave a canonical serialization that can always be\nunambiguously computed to derive the payload."]
    pub fn set_signature(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signature = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAnalysisOccurrenceAttestationElSignaturesEl {
    type O = BlockAssignable<ContainerAnalysisOccurrenceAttestationElSignaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisOccurrenceAttestationElSignaturesEl {
    #[doc = "The identifier for the public key that verifies this\nsignature. MUST be an RFC3986 conformant\nURI. * When possible, the key id should be an\nimmutable reference, such as a cryptographic digest.\nExamples of valid values:\n\n* OpenPGP V4 public key fingerprint. See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr\n  for more details on this scheme.\n    * 'openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA'\n* RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization):\n    * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\""]
    pub public_key_id: PrimField<String>,
}
impl BuildContainerAnalysisOccurrenceAttestationElSignaturesEl {
    pub fn build(self) -> ContainerAnalysisOccurrenceAttestationElSignaturesEl {
        ContainerAnalysisOccurrenceAttestationElSignaturesEl {
            public_key_id: self.public_key_id,
            signature: core::default::Default::default(),
        }
    }
}
pub struct ContainerAnalysisOccurrenceAttestationElSignaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisOccurrenceAttestationElSignaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAnalysisOccurrenceAttestationElSignaturesElRef {
        ContainerAnalysisOccurrenceAttestationElSignaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisOccurrenceAttestationElSignaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_key_id` after provisioning.\nThe identifier for the public key that verifies this\nsignature. MUST be an RFC3986 conformant\nURI. * When possible, the key id should be an\nimmutable reference, such as a cryptographic digest.\nExamples of valid values:\n\n* OpenPGP V4 public key fingerprint. See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr\n  for more details on this scheme.\n    * 'openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA'\n* RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization):\n    * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\""]
    pub fn public_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signature` after provisioning.\nThe content of the signature, an opaque bytestring.\nThe payload that this signature verifies MUST be\nunambiguously provided with the Signature during\nverification. A wrapper message might provide the\npayload explicitly. Alternatively, a message might\nhave a canonical serialization that can always be\nunambiguously computed to derive the payload."]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerAnalysisOccurrenceAttestationElDynamic {
    signatures: Option<DynamicBlock<ContainerAnalysisOccurrenceAttestationElSignaturesEl>>,
}
#[derive(Serialize)]
pub struct ContainerAnalysisOccurrenceAttestationEl {
    serialized_payload: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signatures: Option<Vec<ContainerAnalysisOccurrenceAttestationElSignaturesEl>>,
    dynamic: ContainerAnalysisOccurrenceAttestationElDynamic,
}
impl ContainerAnalysisOccurrenceAttestationEl {
    #[doc = "Set the field `signatures`.\n"]
    pub fn set_signatures(
        mut self,
        v: impl Into<BlockAssignable<ContainerAnalysisOccurrenceAttestationElSignaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.signatures = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.signatures = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAnalysisOccurrenceAttestationEl {
    type O = BlockAssignable<ContainerAnalysisOccurrenceAttestationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisOccurrenceAttestationEl {
    #[doc = "The serialized payload that is verified by one or\nmore signatures. A base64-encoded string."]
    pub serialized_payload: PrimField<String>,
}
impl BuildContainerAnalysisOccurrenceAttestationEl {
    pub fn build(self) -> ContainerAnalysisOccurrenceAttestationEl {
        ContainerAnalysisOccurrenceAttestationEl {
            serialized_payload: self.serialized_payload,
            signatures: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAnalysisOccurrenceAttestationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisOccurrenceAttestationElRef {
    fn new(shared: StackShared, base: String) -> ContainerAnalysisOccurrenceAttestationElRef {
        ContainerAnalysisOccurrenceAttestationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisOccurrenceAttestationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `serialized_payload` after provisioning.\nThe serialized payload that is verified by one or\nmore signatures. A base64-encoded string."]
    pub fn serialized_payload(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serialized_payload", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAnalysisOccurrenceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAnalysisOccurrenceTimeoutsEl {
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
impl ToListMappable for ContainerAnalysisOccurrenceTimeoutsEl {
    type O = BlockAssignable<ContainerAnalysisOccurrenceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisOccurrenceTimeoutsEl {}
impl BuildContainerAnalysisOccurrenceTimeoutsEl {
    pub fn build(self) -> ContainerAnalysisOccurrenceTimeoutsEl {
        ContainerAnalysisOccurrenceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAnalysisOccurrenceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisOccurrenceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAnalysisOccurrenceTimeoutsElRef {
        ContainerAnalysisOccurrenceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisOccurrenceTimeoutsElRef {
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
struct ContainerAnalysisOccurrenceDynamic {
    attestation: Option<DynamicBlock<ContainerAnalysisOccurrenceAttestationEl>>,
}
